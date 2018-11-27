#![no_std]
#![no_main]
// #![allow{dead_code}]

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

use stm32l4x6::TIM6;


use stm32l4x6_hal::common::Constrain;
use embedded_hal::timer::{Periodic,CountDown};
use embedded_hal::blocking::spi::Transfer;
use cortex_m::asm::delay;

#[entry]

fn main() -> ! {

    let p = stm32l4x6::Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();

    let mut flash = p.FLASH.constrain();

    let cfgr = rcc.cfgr.sysclk(clocking::SysClkSource::MSI(clocking::MediumSpeedInternalRC::new(32_000_000, false))).hclk(MegaHertz(32)).pclk1(MegaHertz(32)).pclk2(MegaHertz(32));
    
    let spiclocks = cfgr.freeze(&mut flash.acr);


    let mut spi1 = p.SPI1;


    let mut gpioa = gpio::A::new(&mut rcc.ahb);
    let mut gpioc = gpio::C::new(&mut rcc.ahb);


    let mut timer: Timer<TIM6> = stm32l4x6_hal::timer::Timer::tim6(p.TIM6,Hertz(20), spiclocks, &mut rcc.apb1);

/*  Added a blinky section here
    let mut led:  PA5<gpio::Output<gpio::PushPull>> = gpioa.PA5.into_output(&mut gpioa.moder, &mut gpioa.otyper);
    // TMR6 basic timer stm32l4x6::rcc::apb1enr1 | apb1rstr1
    let mut timer: Timer<TIM6> = stm32l4x6_hal::timer::Timer::tim6(p.TIM6,Hertz(20), spiclocks, &mut rcc.apb1);

    let mut state= false;
    loop{
       timer.start(1);
       block!(timer.wait()); // blocks for 1 second
       if state {
          led.set_high();
       }else{
          led.set_low();
       }
       state=!state;
       
    }

*/
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

 use stm32l4x6_hal::embedded_hal::spi::FullDuplex;


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
            spi.send(0xF8 as u8);
       }
       if spi_irq.is_high(){
           spi.send(0x1C as u8);
           spi.send(0x0 as u8); 
           let ans=block!(spi.read());
           match ans {
               Ok(t) => {
                   buf[0]=t;
               }
               Err(_) => {
                   while true{
                       //trap here
                   }
               }
           }
           let ans=block!(spi.read());
           match ans {
               Ok(t) => {
                   buf[1]=t;
               }
               Err(_) => {
                   while true{
                       //trap here
                   }
               }
           }
       }
       spi_cs.set_high();



/*
         //Read button for press and release
         while spi_irq.is_high(){
            delay(100);
            ;
         }
         while spi_irq.is_low(){
            delay(100);
            ;
         }
*/
    }
}
