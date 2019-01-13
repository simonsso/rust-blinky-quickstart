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
extern crate nrf52832_hal;
extern crate bmlite;
use bmlite::*;

extern crate nb;

use cortex_m_rt::entry;

use nrf52832_hal::gpio::*;
use nrf52832_hal::gpio::Level;
use nrf52832_hal::gpio::p0::*;
use nrf52832_hal::prelude::GpioExt;
use nrf52832_hal::prelude::SpimExt;
use embedded_hal::digital::{InputPin,OutputPin};

// use nrf52_hal::spi;
// use nrf52_hal::embedded_hal::digital::OutputPin;
// use nrf52_hal::rcc::clocking;
// use nrf52_hal::time::{MegaHertz,Hertz};
// use nrf52_hal::timer::*;
use nrf52832_hal::*;
use cortex_m::asm::delay;
use alloc::vec::Vec;

fn hex_nible(i:u8)-> u8{
    if i&0xF <10 {
       (i&0xf)+0x30
    }else{
       (i & 0xf) + 0x41 - 10
    }
}

fn hex(a:u8) -> [u8;5] {
    [0x30,0x78,hex_nible(a>>4),hex_nible(a),0x2c]
}
#[entry]
fn main() -> ! {

    let p = nrf52832_hal::nrf52832_pac::Peripherals::take().unwrap();
    let port0 = p.P0.split();

    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, 2048 as usize) }

    let mut led0_red: P0_20<gpio::Output<PushPull>>  = port0.p0_20.into_push_pull_output(Level::Low );
    //Gpio Conflict @pin19
    let mut led2_green: P0_18<gpio::Output<PushPull>>  = port0.p0_18.into_push_pull_output(Level::Low );
    let mut led3_green: P0_17<gpio::Output<PushPull>>  = port0.p0_17.into_push_pull_output(Level::Low );
    let mut led1_red: P0_03<gpio::Output<PushPull>>  = port0.p0_03.into_push_pull_output(Level::Low );

    //arduino D13,D12,D11
    let spiclk:  P0_Pin<Output<PushPull>> = port0.p0_25.into_push_pull_output(Level::Low).degrade();
    let spimosi: P0_Pin<Output<PushPull>> = port0.p0_23.into_push_pull_output(Level::Low).degrade();
    let spimiso: P0_Pin<Input<Floating>>  = port0.p0_24.into_floating_input().degrade();

    /*
    // Uart pins connected to USB helper:
    // P0.05 RTS
    // P0.06 TXD
    // P0.07 CTS
    // P0.08 RXD
    */
    let uartpins = nrf52832_hal::uarte::Pins{
        rts: None, // Some(port0.p0_05.into_push_pull_output(Level::Low).degrade()),
        txd: port0.p0_06.into_push_pull_output(Level::Low).degrade(),
        cts: None, // Some(port0.p0_07.into_push_pull_output(Level::Low).degrade()),
        rxd: port0.p0_08.into_push_pull_output(Level::Low).degrade(),
    };
    use nrf52832_hal::uarte::Baudrate::BAUD115200;
    use nrf52832_hal::uarte::Parity::EXCLUDED;
    let mut uart = nrf52832_hal::uarte::Uarte::new(
         p.UARTE0,
         uartpins,
         EXCLUDED,
         BAUD115200,
    );

    // uarte DMA cannot handle strings in flash.
    // use Vec to enforce data on heap
    let _ = uart.write(&(b"Hello!\r\n").to_vec());

    let pins = nrf52832_hal::spim::Pins{sck:spiclk,miso:spimiso,mosi:spimosi};
    let spi = p.SPIM0.constrain(pins);

    let spi_cs = port0.p0_19.into_push_pull_output(Level::High ).degrade();
    let spi_rst = port0.p0_13.into_push_pull_output(Level::High );
    let spi_irq = port0.p0_28.into_floating_input();

//    let btn1  = port0.p0_13.into_pullup_input();
//    let btn2  = port0.p0_14.into_pullup_input();
//    let btn3  = port0.p0_15.into_pullup_input();
    let btn4  = port0.p0_16.into_pullup_input();

    let mut bm = BmLite::new(spi, spi_cs,spi_rst,spi_irq);
    let _ans = bm.reset(||{delay(100);});

    match bm.get_version() {
        Ok(message) => {
            let mut message = message;
            message.truncate(255);
            let _ = uart.write(&message);
        }
        Err(_) => {
            let _ = uart.write ( & (b"Panic, get version returned error\r\nIs sensor connected?\r\n").to_vec());
            let _ans = bm.reset(||{
                 delay(150);
            });
        }
    }
   loop {
        let _ = uart.write(&(b"Main Loop!\r\n").to_vec());
        match bm.get_template_count() {
            Ok(num) =>{
                let mut v:Vec<u8> = b"Sensor module have : ".to_vec();
                v.extend(hex(num as u8).iter().chain(b" enrolled fingers\r\n".iter()));
                let _ = uart.write(&v);
            }
            _other => {
                let _ans = bm.reset(||{
                    delay(150);
                });
            }
        }
        match bm.get_version() {
            Ok(message) => {
                let mut message = message;
                message.truncate(255);
                let _ = uart.write(&message);
            }
            Err(_) => {
                let _ = uart.write ( & (b"Panic, get version returned error\r\nIs sensor connected?\r\n").to_vec());
                let _ans = bm.reset(||{
                    delay(150);
                });
            }
        }
        // Wait for finger
        match bm.capture(0) {
            Ok(_) => {},
            Err(_) => {
                led1_red.set_high();
                // let _ans = bm.reset();
            },
        } // The user interface: touch the sensor and btn at the same time to enroll
          // Extreemly secure
        if btn4.is_low(){
                led0_red.set_high();
                led1_red.set_high();
                led2_green.set_high();
                led3_green.set_high();
                let _ans = bm.delete_all();
                let ans = bm.enroll(|_|{});
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
                    let mut v:Vec<u8> = b"Identified user: ".to_vec();
                    v.extend(hex(id as u8).iter().chain(b"\r\n".iter()));
                    match id{
                        0 => {led2_green.set_high()}
                        1 => {led3_green.set_high()}
                        2 => {led3_green.set_high();led2_green.set_high()}
                        _ => {}
                     }
                     let _ = uart.write(&v);
                     delay(5000000);
                }
                Err(bmlite::Error::NoMatch) => {
                    led0_red.set_high();
                    // uart.write(&(b"Unknon user\r\n").to_vec());
                }
                Err(_) => {/*let _ans=bm.reset();*/}
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

