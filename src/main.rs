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
use stm32l4x6::{interrupt, Interrupt};
use stm32l4x6::Peripherals;

extern crate stm32l4x6;

fn delay_busy(){
    for i in 0..10000{

    }
        
}

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

    let p = stm32l4x6::Peripherals::take().unwrap();
    let mut rcc =  p.RCC;
    let mut gpioa = p.GPIOA;
    //RCC->AHB1ENR |= RCC_AHB1ENR_GPIOAEN
    rcc.ahb2enr.modify(|r, w| unsafe{ w.bits( 0x0000_0001 | r.bits() )});
    gpioa.moder.modify( |r, w| unsafe{w.bits( 0x5555 | r.bits()& 0xFFFF_0000)} );


    loop {
        // your code goes here
        gpioa.odr.write(|w| unsafe {w.bits(0xFFFF)} );
        gpioa.odr.write(|w| unsafe {w.bits(0x0000)} );
    }
}
