#!/bin/sh

cargo build --target=thumbv7em-none-eabi --features mcu_stm32f4 --release "$@"
