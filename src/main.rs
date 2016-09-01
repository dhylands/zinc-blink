#![feature(plugin)]
#![no_std]
#![plugin(macro_zinc)]

extern crate zinc;

use zinc::hal::timer::Timer;
use zinc::hal::pin::Gpio;
use zinc::hal::stm32f4::{ init, pin, timer };

#[zinc_main]
pub fn main() {
  zinc::hal::mem_init::init_stack();

  // The STM32F4 Discovery board has an 8 MHz crystal. Configure the PLL to 
  // run at 168 MHz and set it as the System Clock Source.

  let pll_conf = init::PLLConf {
    source: init::PLLClockSource::PLLClockHSE(8_000_000),
    m: 8,
    n: 336,
    p: 2,
    q: 7,
  };
  let sys_conf = init::SysConf {
      clock: init::ClockConf {
          source: init::SystemClockSource::SystemClockPLL(pll_conf),
      },
  };
  sys_conf.setup();

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

  // We're now running at 168 MHz, and Timer2's source frequency is 84 MHz
  let timer = timer::Timer::new(timer::TimerPeripheral::Timer2, 84);

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
