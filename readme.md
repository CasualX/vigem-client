ViGEm client in Rust
====================

[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/vigem-client.svg)](https://crates.io/crates/vigem-client)
[![docs.rs](https://docs.rs/vigem-client/badge.svg)](https://docs.rs/vigem-client)

[ViGEm](https://vigem.org/) is a Virtual Gamepad Emulation Framework.
This crate implements a client for the [ViGEmBus Driver](https://github.com/ViGEm/ViGEmBus).
The driver must be installed for this library to have any use.

The client is written 100% in Rust, ViGEm's client C library is not used.
Of course it must talk to WinAPI which means it's only available for Windows platforms.

Unlike the competition this library provides an optimized, safe and idiomatic interface.

Usage
-----

This library is available on [crates.io](https://crates.io/crates/vigem-client) and its documentation on [docs.rs](https://docs.rs/vigem-client).

In your `Cargo.toml` add:

```
[dependencies]
vigem-client = "0.1"
```

Examples
--------

Try this example out: `cargo run --example readme`:

```rust
use std::{thread, time};

fn main() {
	// Connect to the ViGEmBus driver
	let client = vigem_client::Client::connect().unwrap();

	// Create the virtual controller target
	let id = vigem_client::TargetId::XBOX360_WIRED;
	let mut target = vigem_client::Xbox360Wired::new(client, id);

	// Plugin the virtual controller
	target.plugin().unwrap();

	// Wait for the virtual controller to be ready to accept updates
	target.wait_ready().unwrap();

	// The input state of the virtual controller
	let mut gamepad = vigem_client::XGamepad {
		buttons: vigem_client::XButtons!(UP | RIGHT | LB | A | X),
		..Default::default()
	};

	let start = time::Instant::now();
	loop {
		let elapsed = start.elapsed().as_secs_f64();

		// Play for 10 seconds
		if elapsed >= 10.0 {
			break;
		}

		// Spin the left thumb stick in circles
		gamepad.thumb_lx = (elapsed.cos() * 30000.0) as i16;
		gamepad.thumb_ly = (elapsed.sin() * 30000.0) as i16;

		// Spin the right thumb stick in circles
		gamepad.thumb_rx = -gamepad.thumb_ly;
		gamepad.thumb_ry = gamepad.thumb_lx;

		// Twiddle the triggers
		gamepad.left_trigger = ((((elapsed * 1.5).sin() * 127.0) as i32) + 127) as u8;
		gamepad.right_trigger = ((((elapsed * 1.5).cos() * 127.0) as i32) + 127) as u8;

		let _ = target.update(&gamepad);

		thread::sleep(time::Duration::from_millis(10));
	}
}
```

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
