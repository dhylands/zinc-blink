#!/bin/sh

TARGET=target/thumbv7em-none-eabi/release/zinc_blink
BIN=${TARGET}.bin

arm-none-eabi-objcopy -O binary ${TARGET} ${BIN}
st-flash --reset write ${BIN} 0x08000000
