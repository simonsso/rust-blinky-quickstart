#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::*;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprint;
use stm32l476rg::{interrupt, Interrupt};
use stm32l476rg::Peripherals;

extern crate stm32l476rg;


#[entry]
fn main() -> ! {
        //     //  // Enable GPIO Peripheral clock
        //    /* GPIOC Periph clock enable */
        //    RCC->AHB1ENR |= (RCC_AHB1ENR_GPIOAEN |RCC_AHB1ENR_GPIOBEN );

        //    // Set up direction for GPIOA B 
        //    // Each GPIO bit have 2 control bits
        //    // 00 input
        //    // 01 output
        //    // 10 alternative function
        //    // 11 Analogue
        //    GPIOA->MODER = (GPIOA->MODER & 0xFFFF0000) | 0x00005555 ;
        // GPIOB->MODER = (GPIOB->MODER & 0xFFFF0000) | 0x00005555 ;
    let p = cortex_m::Peripherals::take().unwrap();

    let mut syst = p.SYST;
    let mut nvic = p.NVIC;

    let p = stm32l476rg::Peripherals::take().unwrap();
    let mut rcc =  p.RCC;

    rcc.ahb2enr.modify(|r, w| w.gpioaen().set_bit());
    rcc.ahb2enr.modify(|r, w| w.gpiocen().set_bit());
    rcc.apb1enr1.modify(|r, w| w.tim7en().set_bit());

    let mut tim7 = p.TIM7;

    let psc = 0x7a; // 8M /65k ~= 121 
    tim7.psc.write(|w| unsafe { w.psc().bits(psc) });
    let arr = 0xA000;
    tim7.arr.write(|w| unsafe { w.arr().bits(arr) });
    tim7.cr1.write(|w| w.opm().clear_bit());



    let mut gpioa = p.GPIOA;
    let mut gpioc = p.GPIOC;
    //RCC->AHB1ENR |= RCC_AHB1ENR_GPIOAEN
    gpioa.moder.modify( |r, w| unsafe{w.bits( 0x5555 | r.bits()& 0xFFFF_0000)} );
    gpioc.moder.modify( |_, w| unsafe{w.moder13().bits(0)} );

    let mut state = false;
    loop {
/*         // Wait for an update event
        while !tim7.sr.read().uif().bit() {}

        // Clear the update event flag
         tim7.sr.modify(|_, w| w.uif().clear_bit());
       */
        while gpioc.idr.read().idr13().bit() {
            
        } 
        while ! gpioc.idr.read().idr13().bit() {
            
        } 
        // Toggle the state
        state = !state;

        if state {
            gpioa.odr.write(|w| unsafe {w.bits(0xFFFF)} );
        }else{
            gpioa.odr.write(|w| unsafe {w.bits(0x0000)} );
        }
    }
}
