#![no_std]
#![no_main]
// #![allow{dead_code}]

#![feature(alloc)]
// #![feature(global_allocator)]
#![feature(lang_items)]

// Plug in the allocator crate
extern crate alloc;
extern crate nb;
//extern crate alloc_cortex_m;
use core::alloc::Layout;

extern crate cortex_m_rt as rt; // v0.5.x

// use alloc::vec::Vec;
use alloc_cortex_m::CortexMHeap;

#[macro_use(block)]
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();



// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger
extern crate nrf52832_hal;

use cortex_m_rt::entry;

use nrf52832_hal::gpio::*;
use nrf52832_hal::gpio::Level;
use nrf52832_hal::gpio::p0::*;
use nrf52832_hal::prelude::GpioExt;
use nrf52832_hal::prelude::SpimExt;
use embedded_hal::digital::{InputPin,OutputPin};
use embedded_hal::spi::FullDuplex;

// use nrf52_hal::spi;
// use nrf52_hal::embedded_hal::digital::OutputPin;
// use nrf52_hal::rcc::clocking;
// use nrf52_hal::time::{MegaHertz,Hertz};
// use nrf52_hal::timer::*;
use nrf52832_hal::*;
use nb::*;

// use nrf52::TIM6;

// use nrf52_hal::common::Constrain;
use cortex_m::asm::delay;



#[entry]
fn main() -> ! {

    let p = nrf52832_hal::nrf52832_pac::Peripherals::take().unwrap();
//    let mut rcc = p.RCC.constrain();
    let port0 = p.P0.split();

    // Use SRAM2 for heap ans SRAM1 for stack
    // Todo use p this is hard coded from STM32L476RG memorymap
//    unsafe { ALLOCATOR.init(0x1000_0000 as usize, 0x8000 as usize) }
//    let mut flash = p.FLASH.constrain();


//   let cfgr = rcc.cfgr.sysclk(clocking::SysClkSource::MSI(clocking::MediumSpeedInternalRC::new(32_000_000, false))).hclk(MegaHertz(32)).pclk1(MegaHertz(32)).pclk2(MegaHertz(32));
;
//    let spiclocks = cfgr.freeze(&mut flash.acr);





//    let mut _timer: Timer<TIM6> = nrf52_hal::timer::Timer::tim6(p.TIM6,Hertz(20), spiclocks, &mut rcc.apb1);
    let mut led0_red: P0_20<gpio::Output<PushPull>>  = port0.p0_20.into_push_pull_output(Level::Low );
//Gpio Conflict
    let mut led2_green: P0_18<gpio::Output<PushPull>>  = port0.p0_18.into_push_pull_output(Level::Low );
    let mut led3_green: P0_17<gpio::Output<PushPull>>  = port0.p0_17.into_push_pull_output(Level::Low );
    let mut led1_red: P0_03<gpio::Output<PushPull>>  = port0.p0_03.into_push_pull_output(Level::Low );

    //arduino D13,D12,D11
    let spiclk:  P0_Pin<Output<PushPull>> = port0.p0_25.into_push_pull_output(Level::Low).degrade();
    let spimosi: P0_Pin<Output<PushPull>> = port0.p0_23.into_push_pull_output(Level::Low).degrade();
    let spimiso: P0_Pin<Input<Floating>>  = port0.p0_24.into_floating_input().degrade();

//    let spifreq= nrf52_hal::time::Hertz(2_000_000);

    let pins = nrf52832_hal::spim::Pins{sck:spiclk,miso:spimiso,mosi:spimosi};
//    let mut spim:<nrf52832_hal::Spim as FullDuplex<u8>> = p.SPIM0.constrain(pins);
    let mut spim = p.SPIM0.constrain(pins);

    let mut spi_cs = port0.p0_19.into_push_pull_output(Level::High ).degrade();
    let mut spi_rst = port0.p0_13.into_push_pull_output(Level::High );
    spi_rst.set_low();
    delay(10);
    spi_rst.set_high();

//    let btn1  = port0.p0_13.into_pullup_input();
    let btn2  = port0.p0_14.into_pullup_input();
    let btn3  = port0.p0_15.into_pullup_input();
    let btn4  = port0.p0_16.into_pullup_input();

        let mut txbuf:[u8;2] = [0x1c,00];
        let mut rxbuf:[u8;2] = [0 ,0];

        for i in 0..2 {
            let _ans=block!(spim.send(txbuf[i]));
            let ans:u8 = block!(FullDuplex::read(&mut spim)).unwrap();
            rxbuf[i];
        }


    //let mut txbuf:[u8;3] = [0xfc,00,00];
    // let mut rxbuf:[u8;3] = [0 ,0,0];

    
    loop{
        led0_red.set_high();
        delay(1000000);
        led0_red.set_low();
        delay(1000000);
/*
        led1_red.set_high();
        led2_green.set_high();
        led3_green.set_high();
        led1_red.set_low();
        led2_green.set_low();
        led3_green.set_low();
        if btn1.is_high(){
            led0_red.set_high();
        }else{
            led0_red.set_low();
        }
*/
        if btn2.is_high(){
            led1_red.set_high();
        }else{
            led1_red.set_low();
        }
        if btn3.is_high(){
            led2_green.set_high();
        }else{
            led2_green.set_low();
        }
        if btn4.is_high(){
            led3_green.set_high();
        }else{
            led3_green.set_low();
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

