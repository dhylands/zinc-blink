#![feature(plugin)]
#![no_std]
#![plugin(macro_zinc)]

extern crate zinc;

use zinc::hal::timer::Timer;
use zinc::hal::pin::Gpio;
use zinc::hal::stm32f4::{pin, timer};

#[zinc_main]
pub fn main() {
  zinc::hal::mem_init::init_stack();
  zinc::hal::mem_init::init_data();

  let leds = [
    pin::Pin {
      port: pin::Port::PortD,
      pin: 12u8,
      function: pin::Function::GPIOOut
    },
    pin::Pin {
      port: pin::Port::PortD,
      pin: 13u8,
      function: pin::Function::GPIOOut
    },
    pin::Pin {
      port: pin::Port::PortD,
      pin: 14u8,
      function: pin::Function::GPIOOut
    },
    pin::Pin {
      port: pin::Port::PortD,
      pin: 15u8,
      function: pin::Function::GPIOOut
    },
  ];

  for led in leds.iter() {
    led.setup();
  }

  let timer = timer::Timer::new(timer::TimerPeripheral::Timer2, 16u32);

  loop {
    for led in leds.iter() {
      led.set_high();
      timer.wait_ms(100);
      led.set_low();
      timer.wait_ms(100);
      led.set_high();
      timer.wait_ms(100);
      led.set_low();
      timer.wait_ms(700);
    }
  }
}
