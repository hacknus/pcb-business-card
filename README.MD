# Vacuum Gateway Unit (VGU) Control Software (Rust Code)

This repository contains the embedded software for the vacuum gateway unit.  
The hardware schematics can be found in
the [`hardware`](https://spacegit.unibe.ch/IceLab/vacuum-gateway-embedded/-/tree/master/hardware)
folder. Documentation is available in the [`docs`](https://spacegit.unibe.ch/IceLab/vacuum-gateway-embedded/-/tree/master/docs) folder.
Please read the errata in the PDF before assembly.

## Communication

To communicate with this device, use the RS-232 port.

The software uses [freertos-rust](https://github.com/lobaro/FreeRTOS-rust) (wrapper for rust) and thus requires nightly
to compile.

## Flashing the hardware:

Create a `.cargo/config.toml` file according to the `.cargo/config.toml_template` file, specify your runner (depends on
platform).

Be sure to add the LUTs for the pressure sensors, if you are using them.

Connect the board using an ST-Link V3 (with TagConnect) to a USB port on the computer. Be sure to power the board with
an
additional USB-C connector. (ST-Link does not provide power)  
First start the openocd server in a terminal/console window:
```openocd -f interface/stlink.cfg -f target/stm32f4x.cfg```   
Flash/Run the code on the hardware using:  
```cargo run --package vacuum-gateway-embedded --bin vacuum-gateway-embedded --target thumbv7em-none-eabihf --release```

To set up your system, be sure to
follow [this rust embedded stm32 guide](https://docs.rust-embedded.org/discovery/f3discovery/03-setup/index.html)!  
Don't forget to install the [ARM GNU toolchain](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads).

Now that the `rust-toolchain.toml` exists, the two steps below might not be necessary.  
Make sure you did
```rustup install nightly``` and
```rustup target add thumbv7em-none-eabihf --toolchain nightly``` to add the target to the nightly toolchain.