#!/bin/sh

cargo build --verbose --target=thumbv7em-none-eabi --features mcu_stm32f4 --release
