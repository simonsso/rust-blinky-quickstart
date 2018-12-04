#![no_std]
#![no_main]
// #![allow{dead_code}]

#![feature(alloc)]
// #![feature(global_allocator)]
#![feature(lang_items)]

// Plug in the allocator crate
extern crate alloc;
//extern crate alloc_cortex_m;
use core::alloc::Layout;

extern crate cortex_m_rt as rt; // v0.5.x

// use alloc::vec::Vec;
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
extern crate bmlite;
use bmlite::*;


extern crate nb;

use cortex_m_rt::entry;

use stm32l4x6_hal::gpio;
use stm32l4x6_hal::gpio::*;
use stm32l4x6_hal::spi;
use stm32l4x6_hal::embedded_hal::digital::OutputPin;
use stm32l4x6_hal::rcc::clocking;
use stm32l4x6_hal::time::{MegaHertz,Hertz};
use stm32l4x6_hal::timer::*;
use stm32l4x6_hal::*;

use stm32l4x6::TIM6;

use stm32l4x6_hal::common::Constrain;
use cortex_m::asm::delay;



#[entry]
fn main() -> ! {

    let p = stm32l4x6::Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();

    // Use SRAM2 for heap ans SRAM1 for stack
    // Todo use p this is hard coded from STM32L476RG memorymap
    unsafe { ALLOCATOR.init(0x1000_0000 as usize, 0x8000 as usize) }

    let mut flash = p.FLASH.constrain();

    let cfgr = rcc.cfgr.sysclk(clocking::SysClkSource::MSI(clocking::MediumSpeedInternalRC::new(32_000_000, false))).hclk(MegaHertz(32)).pclk1(MegaHertz(32)).pclk2(MegaHertz(32));

    let spiclocks = cfgr.freeze(&mut flash.acr);

    let spi1 = p.SPI1;


    let mut gpioa = gpio::A::new(&mut rcc.ahb);
    let mut gpioc = gpio::C::new(&mut rcc.ahb);


    let mut _timer: Timer<TIM6> = stm32l4x6_hal::timer::Timer::tim6(p.TIM6,Hertz(20), spiclocks, &mut rcc.apb1);
    let mut led0_red: PA0<gpio::Output<gpio::PushPull>> = gpioa.PA0.into_output(&mut gpioa.moder, &mut gpioa.otyper);
    let mut led1_red: PA1<gpio::Output<gpio::PushPull>> = gpioa.PA1.into_output(&mut gpioa.moder, &mut gpioa.otyper);
    let mut led2_green: PC1<gpio::Output<gpio::PushPull>> = gpioc.PC1.into_output(&mut gpioc.moder, &mut gpioc.otyper);
    let mut led3_green: PC0<gpio::Output<gpio::PushPull>> = gpioc.PC0.into_output(&mut gpioc.moder, &mut gpioc.otyper);

    let spiclk:PA5<gpio::AF5> = gpioa.PA5.into_alt_fun(&mut gpioa.moder, &mut gpioa.afrl);
    let spimiso:PA6<gpio::AF5> = gpioa.PA6.into_alt_fun(&mut gpioa.moder, &mut gpioa.afrl);
    let spimosi:PA7<gpio::AF5> = gpioa.PA7.into_alt_fun(&mut gpioa.moder, &mut gpioa.afrl);

    let spifreq= stm32l4x6_hal::time::Hertz(2_000_000);

    let     spi = spi::Spi::new(spi1,(spiclk,spimiso,spimosi),spifreq,embedded_hal::spi::MODE_0,&spiclocks,&mut rcc.apb2);

    let     btn1:      PC13<gpio::Input<gpio::Floating>> = gpioc.PC13.into_input(&mut gpioc.moder, &mut gpioc.pupdr);
    let     spi_irq:   PA4<gpio::Input<gpio::Floating>> =  gpioa.PA4.into_input(&mut gpioa.moder, &mut gpioa.pupdr);
    let mut spi_reset: PA10<gpio::Output<gpio::PushPull>> = gpioa.PA10.into_output(&mut gpioa.moder, &mut gpioa.otyper);
    let mut spi_cs:    PA9<gpio::Output<gpio::PushPull>> =  gpioa.PA9.into_output(&mut gpioa.moder, &mut gpioa.otyper);
    spi_cs.set_high();
    spi_reset.set_high();


    use embedded_hal::digital::InputPin;

    let mut bm = BmLite::new(spi,spi_cs,spi_reset,spi_irq);
    let _ans = bm.reset();
    led0_red.set_high();
    led1_red.set_high();
    led2_green.set_high();
    led3_green.set_high();
    led0_red.set_low();
    led1_red.set_low();
    led2_green.set_low();
    led3_green.set_low();

    loop {
        let ans = bm.capture(0);
        match ans {
            Ok(_) => {},
            Err(_) => {
                led1_red.set_high();
                let _ans = bm.reset();
            },
        } // The user interface touch the sensor and btn at the same time to ensoll
          // Extreemly secure
        if btn1.is_low(){
                led0_red.set_high();
                led1_red.set_high();
                led2_green.set_high();
                led3_green.set_high();
                // let _ans = bm.delete_all();
                let ans = bm.enroll();
                match ans {
                    Ok(_) => {
                        led0_red.set_low();
                        led1_red.set_low();
                        led2_green.set_low();
                        led3_green.set_low();
                        led3_green.set_high();
                    },
                    Err(_) => loop{
                        led0_red.set_low();
                        led1_red.set_low();
                        led2_green.set_low();
                        led3_green.set_low();
                        led0_red.set_high();
                        led1_red.set_high();
                    },
                }
        }else{
            led0_red.set_low();
            led1_red.set_low();
            led2_green.set_low();
            led3_green.set_low();
            let ans= bm.identify();
            match ans {
                Ok(id) => {
                    match id{
                        0 => {led2_green.set_high()}
                        1 => {led3_green.set_high()}
                        2 => {led3_green.set_high();led2_green.set_high()}
                        _ => {}
                     }
                     delay(5000000);
                }
                Err(bmlite::Error::NoMatch) => {led0_red.set_high()}
                Err(_) => {let _ans=bm.reset();}
            }
        }
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

