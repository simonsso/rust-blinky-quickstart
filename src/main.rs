#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger
extern crate stm32l476rg;
extern crate stm32l4x6_hal;

use cortex_m::peripheral::syst::SystClkSource;
// use cortex_m::peripheral::*;
use cortex_m_rt::entry;
// use cortex_m_semihosting::hprint;

#[feature(unproven)]
#[cfg(feature = "unproven")]
use stm32l476rg::{interrupt, Interrupt};
use stm32l476rg::Peripherals;

use stm32l4x6_hal::gpio::{PA3,PC13};
use stm32l4x6_hal::*;

use stm32l4x6_hal::common::Constrain;

#[entry]
fn main() -> ! {
    let p = cortex_m::Peripherals::take().unwrap();

    let mut syst = p.SYST;
    let mut nvic = p.NVIC;

    let p = stm32l476rg::Peripherals::take().unwrap();



    let mut rcc = p.RCC.constrain();

    let mut gpioa = gpio::A::new(&mut rcc.ahb);
    let mut gpioc = gpio::C::new(&mut rcc.ahb);

    let led:  PA3<gpio::Output<gpio::PushPull>> = gpioa.PA3.into_output(&mut gpioa.moder, &mut gpioa.otyper);
    let btn1: PC13<gpio::Input<gpio::Floating>> = gpioc.PC13.into_input(&mut gpioc.moder, &mut gpioc.pupdr);

 //   let mut led: Led = gpioa
 //      .PA3
 //      .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper)
 //      .into();

    let mut state = false;
    loop {
      use stm32l4x6_hal::embedded_hal::digital::InputPin;

         while btn1.is_high(){
            ;
         }
/*      while gpioc.idr.read().idr13().bit() {
            
        } 
        while ! gpioc.idr.read().idr13().bit() {
            
        } 
        // Toggle the state
*/
        state = !state;
        use stm32l4x6_hal::embedded_hal::digital::OutputPin;

        if state {
            led.set_high(); 
        }else{
            led.set_low(); 
        }
    }
}
