/*!
ViGEm client in Rust
====================

[ViGEm](https://vigem.org/) is the Virtual Gamepad Emulation Framework.
This crate implements a client for the [ViGEmBus Driver](https://github.com/ViGEm/ViGEmBus).
The driver must be installed for this library to have any use.

The [`Client`] contains the connection to the ViGEmBus driver.
Start by connecting to the service:

```
let client = vigem_client::Client::connect().unwrap();
```

With a client instance virtual controllers (targets) can be created (eg. [`Xbox360Wired::new`] and [`DualShock4Wired::new`]).
These targets are constructed from a client and a [`TargetId`].

```
let client = vigem_client::Client::connect().unwrap();

# let id = vigem_client::TargetId::XBOX360_WIRED;
// Creates a new virtual Xbox360 wired controller
// It is not yet plugged in
let target = vigem_client::Xbox360Wired::new(client, id);
```

A client can be used by multiple targets by passing a shared borrow of the client:

```
let client = vigem_client::Client::connect().unwrap();

# let id = vigem_client::TargetId::XBOX360_WIRED;
let target1 = vigem_client::Xbox360Wired::new(&client, id);
let target2 = vigem_client::Xbox360Wired::new(&client, id);
```

For memory management reasons you can also pass `Rc` or `Arc` clients:

```
use std::rc::Rc;
let client = Rc::new(vigem_client::Client::connect().unwrap());

# let id = vigem_client::TargetId::XBOX360_WIRED;
let target1 = vigem_client::Xbox360Wired::new(client.clone(), id);
let target2 = vigem_client::Xbox360Wired::new(client.clone(), id);
```

Newly created targets are not plugged in by default, many methods will return [`Error::NotPluggedIn`] except `plugin`:

```no_run
let client = vigem_client::Client::connect().unwrap();
# let id = vigem_client::TargetId::XBOX360_WIRED;
let mut target = vigem_client::Xbox360Wired::new(client, id);

// Plugin the virtual controller
target.plugin().unwrap();
```

When a target is plugged in Windows plays the 'Device Connect' sound.
You can see your virtual controller in the 'Set up USB game controllers' section of Control Panel.

When a target is unplugged (or dropped, which unplugs the target) Windows plays the 'Device Disconnect' sound.
If a target is dropped without running its destructor (eg. process is killed) then the virtual controller will remain stuck.
Under Control Panel's 'Devices and Printers' section you can manually remove the stuck controller devices.

It may take some time before the target is ready to accept updates, see `wait_ready`.
If a target is updated before it is ready it may return [`Error::TargetNotReady`] errors:

```no_run
let client = vigem_client::Client::connect().unwrap();
# let id = vigem_client::TargetId::XBOX360_WIRED;
let mut target = vigem_client::Xbox360Wired::new(client, id);

// Plugin the virtual controller
target.plugin().unwrap();

// Wait until the target is ready to accept updates
target.wait_ready().unwrap();
```

Finally the target is ready to update its input states
(note that `Xbox360Wired` and `DualShock4Wired` targets each have their own input states):

```no_run
let client = vigem_client::Client::connect().unwrap();
# let id = vigem_client::TargetId::XBOX360_WIRED;
let mut target = vigem_client::Xbox360Wired::new(client, id);

// Plugin the virtual controller
target.plugin().unwrap();

// Wait until the target is ready to accept updates
target.wait_ready().unwrap();

// Configure the gamepad pressing nothing but A and X buttons
let gamepad = vigem_client::XGamepad {
	buttons: vigem_client::XButtons!(A | X),
	..Default::default()
};

// Update the target
let _ = target.update(&gamepad);
```

The DualShock4Wired target is under development.
*/

mod bus;
mod event;
mod error;
mod client;
mod x360;
mod ds4;

use self::event::*;
pub use self::error::Error;
pub use self::client::*;
pub use self::x360::*;
pub use self::ds4::*;

/// Vendor and product ids.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct TargetId {
	pub vendor: u16,
	pub product: u16,
}
impl TargetId {
	/// Default vender and product ids for a wired Xbox360 target.
	pub const XBOX360_WIRED: TargetId = TargetId { vendor: 0x045E, product: 0x028E };
	/// Default vender and product ids for a wired DualShock4 target.
	pub const DUALSHOCK4_WIRED: TargetId = TargetId { vendor: 0x054C, product: 0x05C4 };
}
