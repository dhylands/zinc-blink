This is my test program for working with zinc.

It will start life as a copy of zinc/examples/blink_stm32f4 but is done as a
standalone repo.

Since I'm actively using this for development, it uses a path reference to
locate zinc rather than a github reference.

I'm currently building using nightly-2016-05-24 (installed via rustup).
When I tested on Aug 30, 2016 there were build issues with the latest nightly.

This code has been tested on a [STM32F4 discovery board](http://www.digikey.com/product-search/en?keywords=497-11455-ND)
which has an STM32F407.

I used the build.sh script to build the image, and the flash.sh script to flash
it via [stlink](https://github.com/texane/stlink)
