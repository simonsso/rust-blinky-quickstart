#![no_std]
#![no_main]
// #![allow{dead_code}]

#![feature(alloc)]
#![feature(global_allocator)]
#![feature(lang_items)]

// Plug in the allocator crate
extern crate alloc;
//extern crate alloc_cortex_m;
use core::alloc::Layout;

#[macro_use]
extern crate cortex_m_rt as rt; // v0.5.x

use alloc::vec::Vec;
use alloc_cortex_m::CortexMHeap;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();



// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger
extern crate stm32l4x6;
extern crate stm32l4x6_hal;

#[macro_use(block)]

extern crate nb;

use cortex_m_rt::entry;
// use cortex_m_semihosting::hprint;

use stm32l4x6_hal::gpio;
use stm32l4x6_hal::gpio::{PA5,PC13,PA6,PA7,PA4,PA10,PA9};
use stm32l4x6_hal::spi;
use stm32l4x6_hal::embedded_hal::digital::OutputPin;
use stm32l4x6_hal::rcc::clocking;
use stm32l4x6_hal::time::{MegaHertz,Hertz};
use stm32l4x6_hal::timer::*;
use stm32l4x6_hal::*;
use embedded_hal as hal;
use embedded_hal::spi::{FullDuplex, Mode, Phase, Polarity};


use stm32l4x6::TIM6;


use stm32l4x6_hal::common::Constrain;
use embedded_hal::timer::{Periodic,CountDown};
use embedded_hal::blocking::spi::Transfer;
use cortex_m::asm::delay;
fn spitransmit<S> (mut spi: S  , v1:Vec<u8>) -> (S,Vec<u8>)
where
    S: hal::spi::FullDuplex<u8>,
{
    let mut readdata = Vec::with_capacity(v1.len());

    for elem in v1.iter() {
       spi.send(*elem); 
       let ans=block!(spi.read());
       match ans {
          Ok(t) => {
             readdata.push(t);
          }
          Err(_) => {
             while true{
                //trap here
             }
          }
       }
    }
    (spi,readdata)
}


#[entry]
fn main() -> ! {

    let p = stm32l4x6::Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();

    // Initialize the allocator BEFORE you use it
    let start = rt::heap_start() as usize;
    let size = 1024; // in bytes
    unsafe { ALLOCATOR.init(start, size) }

    let mut flash = p.FLASH.constrain();

    let cfgr = rcc.cfgr.sysclk(clocking::SysClkSource::MSI(clocking::MediumSpeedInternalRC::new(32_000_000, false))).hclk(MegaHertz(32)).pclk1(MegaHertz(32)).pclk2(MegaHertz(32));
    
    let spiclocks = cfgr.freeze(&mut flash.acr);


    let mut spi1 = p.SPI1;


    let mut gpioa = gpio::A::new(&mut rcc.ahb);
    let mut gpioc = gpio::C::new(&mut rcc.ahb);


    let mut timer: Timer<TIM6> = stm32l4x6_hal::timer::Timer::tim6(p.TIM6,Hertz(20), spiclocks, &mut rcc.apb1);

    let spiclk:PA5<gpio::AF5> = gpioa.PA5.into_alt_fun(&mut gpioa.moder, &mut gpioa.afrl);
    let spimiso:PA6<gpio::AF5> = gpioa.PA6.into_alt_fun(&mut gpioa.moder, &mut gpioa.afrl);
    let spimosi:PA7<gpio::AF5> = gpioa.PA7.into_alt_fun(&mut gpioa.moder, &mut gpioa.afrl);

    let spifreq= stm32l4x6_hal::time::Hertz(2_000_000);
 
    let mut spi = spi::Spi::new(spi1,(spiclk,spimiso,spimosi),spifreq,embedded_hal::spi::MODE_0,&spiclocks,&mut rcc.apb2);

    let     btn1:      PC13<gpio::Input<gpio::Floating>> = gpioc.PC13.into_input(&mut gpioc.moder, &mut gpioc.pupdr);
    let     spi_irq:   PA4<gpio::Input<gpio::Floating>> =  gpioa.PA4.into_input(&mut gpioa.moder, &mut gpioa.pupdr);
    let mut spi_reset: PA10<gpio::Output<gpio::PushPull>> = gpioa.PA10.into_output(&mut gpioa.moder, &mut gpioa.otyper);
    let mut spi_cs:    PA9<gpio::Output<gpio::PushPull>> =  gpioa.PA9.into_output(&mut gpioa.moder, &mut gpioa.otyper);
    spi_cs.set_high();



    let mut buf:[u8;10] = [0; 10];



    spi_reset.set_low();
    delay(1000);
    spi_reset.set_high();
    use embedded_hal::digital::InputPin;
    while spi_irq.is_low(){
         delay(100);
    }

    
    loop {
       spi_cs.set_low();
       if spi_irq.is_low(){
          let mut v;
          let ans = spitransmit(spi,[0xFC,0,0].to_vec());
          spi = ans.0;
          v = ans.1;
          let ans = spitransmit(spi,[0xf8].to_vec());
          spi = ans.0;
          v = ans.1;

       }
       if spi_irq.is_high(){
          let mut v;
          let ans = spitransmit(spi,[0x1C,0].to_vec());
          spi = ans.0;
          v = ans.1;
       }
       spi_cs.set_high();


      }
}
// required: define how Out Of Memory (OOM) conditions should be handled
// *if* no other crate has already defined `oom`
#[lang = "oom"]
#[no_mangle]

pub fn rust_oom(_layout: Layout) -> ! {
   // trap here for the debuger to find
   loop {
   }
}

