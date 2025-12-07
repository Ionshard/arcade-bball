Arcade Basket Ball
=====

Rust project for the _Arduino Uno_.

## Build Instructions
1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware.

3. Run `cargo run` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

# How this Repo was Setup

- Working on windows
	- To get USB into WSL2
		- In an admin command prompt:
			-
			  ```cmd
			  wsl --update # Need to update WSL2 kernal to support USB-IP
			  ```
		- Install usbipd-win
			- https://github.com/dorssel/usbipd-win
			- I installed via chocolatey
		- In an admin command prompt:
			-
			  ```cmd
			  usbipd list
			  # Find busid for Arduino
			  usbip bind --busid=<busid>
			  usbip attach --wsl --busid=<busid>
			  ```
		- In WSL
			-
			  ```bash
			  lsusb
			  # Should see Arduino Uno
			  ls /dev/ttyACM*
			  # Should see /dev/ttyACM0
			  ```
	- In WSL2:
		-
		  ```bash
		  sudo apt-get update
		  
		  # Install Rust
		  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
		  # Install cargo-generate dependencies
		  sudo apt install libudev-dev pkg-config
		  
		  # Install Cargo tools
		  cargo install cargo-generate
		  cargo install cargo-binstall
		  cargo install ravedude
		  cd Projects
		  mkdir embedded
		  cd embedded
		  # Create template for Arduino Project (Select Arduino Uno from menu)
		  cargo generate --git https://github.com/Rahix/avr-hal-template.git
		  # Project name: (arcade-bbal)
		  cd arcade-bball
		  # Install AVR Dependencies
		  sudo apt-get install binutils gcc-avr avr-libc avrdude
		  
		  # Flash device
		  cargo run
		  ```
	- Should now see the LED on Arduino blinking!