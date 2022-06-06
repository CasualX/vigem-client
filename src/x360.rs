use std::{fmt, mem, ptr};
#[cfg(feature = "unstable_xtarget_notification")]
use std::{marker, pin, thread};
use std::borrow::Borrow;
use winapi::um::xinput::XINPUT_GAMEPAD;
use winapi::shared::winerror;
use crate::*;

/// XInput compatible button flags.
#[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct XButtons {
	pub raw: u16,
}

/// XInput compatible button flags.
#[allow(non_snake_case)]
#[inline]
pub const fn XButtons(raw: u16) -> XButtons {
	XButtons { raw }
}

/// XInput compatible button flags.
///
/// ```
/// let buttons = vigem_client::XButtons!(UP|RIGHT|LB|A|X);
/// assert_eq!(buttons, vigem_client::XButtons(0x5109));
/// ```
#[macro_export]
macro_rules! XButtons {
	(UP) => { $crate::XButtons { raw: $crate::XButtons::UP } };
	(DOWN) => { $crate::XButtons { raw: $crate::XButtons::DOWN } };
	(LEFT) => { $crate::XButtons { raw: $crate::XButtons::LEFT } };
	(RIGHT) => { $crate::XButtons { raw: $crate::XButtons::RIGHT } };
	(START) => { $crate::XButtons { raw: $crate::XButtons::START } };
	(BACK) => { $crate::XButtons { raw: $crate::XButtons::BACK } };
	(LTHUMB) => { $crate::XButtons { raw: $crate::XButtons::LTHUMB } };
	(RTHUMB) => { $crate::XButtons { raw: $crate::XButtons::RTHUMB } };
	(LB) => { $crate::XButtons { raw: $crate::XButtons::LB } };
	(RB) => { $crate::XButtons { raw: $crate::XButtons::RB } };
	(GUIDE) => { $crate::XButtons { raw: $crate::XButtons::GUIDE } };
	(A) => { $crate::XButtons { raw: $crate::XButtons::A } };
	(B) => { $crate::XButtons { raw: $crate::XButtons::B } };
	(X) => { $crate::XButtons { raw: $crate::XButtons::X } };
	(Y) => { $crate::XButtons { raw: $crate::XButtons::Y } };

	($($face:ident)|*) => {
		$crate::XButtons { raw: 0 $(| $crate::XButtons!($face).raw)* }
	};
}

impl XButtons {
	/// Dpad up button.
	pub const UP: u16     = 0x0001;
	/// Dpad down button.
	pub const DOWN: u16   = 0x0002;
	/// Dpad left button.
	pub const LEFT: u16   = 0x0004;
	/// Dpad right button.
	pub const RIGHT: u16  = 0x0008;
	/// Start button.
	pub const START: u16  = 0x0010;
	/// Back button.
	pub const BACK: u16   = 0x0020;
	/// Left thumb button.
	pub const LTHUMB: u16 = 0x0040;
	/// Right thumb button.
	pub const RTHUMB: u16 = 0x0080;
	/// Left shoulder button.
	pub const LB: u16     = 0x0100;
	/// Right shoulder button.
	pub const RB: u16     = 0x0200;
	/// Xbox guide button.
	pub const GUIDE: u16  = 0x0400;
	/// A button.
	pub const A: u16      = 0x1000;
	/// B button.
	pub const B: u16      = 0x2000;
	/// X button.
	pub const X: u16      = 0x4000;
	/// Y button.
	pub const Y: u16      = 0x8000;
}

impl From<u16> for XButtons {
	#[inline]
	fn from(raw: u16) -> Self {
		XButtons { raw }
	}
}
impl From<XButtons> for u16 {
	#[inline]
	fn from(buttons: XButtons) -> Self {
		buttons.raw
	}
}
impl AsRef<u16> for XButtons {
	#[inline]
	fn as_ref(&self) -> &u16 {
		&self.raw
	}
}
impl AsMut<u16> for XButtons {
	#[inline]
	fn as_mut(&mut self) -> &mut u16 {
		&mut self.raw
	}
}

impl fmt::Debug for XButtons {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if f.alternate() {
			const NAMES: [&'static str; 16] = [
				"UP", "DOWN", "LEFT", "RIGHT",
				"START", "BACK", "LTHUMB", "RTHUMB",
				"LB", "RB", "GUIDE", "?",
				"A", "B", "X", "Y",
			];
			let mut comma = false;
			for index in 0..16 {
				if self.raw & (1 << index) != 0 {
					if comma {
						f.write_str("|")?;
						comma = true;
					}
					f.write_str(NAMES[index])?;
				}
			}
			Ok(())
		}
		else {
			write!(f, "XButtons({:#x})", self.raw)
		}
	}
}

/// XInput compatible gamepad.
///
/// Represents an [`XINPUT_GAMEPAD`]-compatible report structure.
///
/// ![image](https://user-images.githubusercontent.com/2324759/124391245-f889b180-dcef-11eb-927c-4b76d2ca332d.png)
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct XGamepad {
	pub buttons: XButtons,
	pub left_trigger: u8,
	pub right_trigger: u8,
	pub thumb_lx: i16,
	pub thumb_ly: i16,
	pub thumb_rx: i16,
	pub thumb_ry: i16,
}

impl From<XINPUT_GAMEPAD> for XGamepad {
	#[inline]
	fn from(gamepad: XINPUT_GAMEPAD) -> Self {
		unsafe { mem::transmute(gamepad) }
	}
}
impl From<XGamepad> for XINPUT_GAMEPAD {
	#[inline]
	fn from(report: XGamepad) -> XINPUT_GAMEPAD {
		unsafe { mem::transmute(report) }
	}
}
impl AsRef<XINPUT_GAMEPAD> for XGamepad {
	#[inline]
	fn as_ref(&self) -> &XINPUT_GAMEPAD {
		unsafe { mem::transmute(self) }
	}
}
impl AsMut<XINPUT_GAMEPAD> for XGamepad {
	#[inline]
	fn as_mut(&mut self) -> &mut XINPUT_GAMEPAD {
		unsafe { mem::transmute(self) }
	}
}

/// XInput notification structure.
#[cfg(feature = "unstable_xtarget_notification")]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct XNotification {
	pub large_motor: u8,
	pub small_motor: u8,
	pub led_number: u8,
}

/// XInput notification request.
#[cfg(feature = "unstable_xtarget_notification")]
pub struct XRequestNotification {
	client: Client,
	xurn: bus::RequestNotification<bus::XUsbRequestNotification>,
	_unpin: marker::PhantomPinned,
}

#[cfg(feature = "unstable_xtarget_notification")]
impl XRequestNotification {
	/// Returns if the underlying target is still attached.
	#[inline]
	pub fn is_attached(&self) -> bool {
		self.xurn.buffer.SerialNo != 0
	}

	/// Spawns a thread to handle the notifications.
	///
	/// The callback `f` is invoked for every notification.
	///
	/// Returns a [`JoinHandle`](thread::JoinHandle) for the created thread.
	/// It is recommended to join the thread after the target from which the notifications are requested is dropped.
	#[inline]
	pub fn spawn_thread<F: FnMut(&XRequestNotification, XNotification) + Send + 'static>(self, mut f: F) -> thread::JoinHandle<()> {
		thread::spawn(move || {
			// Safety: the request notification object is not accessible after it is pinned
			let mut reqn = self;
			let mut reqn = unsafe { pin::Pin::new_unchecked(&mut reqn) };
			loop {
				reqn.as_mut().request();
				let result = reqn.as_mut().poll(true);
				match result {
					Ok(None) => {},
					Ok(Some(data)) => f(&reqn, data),
					// When the target is dropped the notification request is aborted
					Err(_) => break,
				}
			}
		})
	}

	/// Requests a notification.
	#[inline(never)]
	pub fn request(self: pin::Pin<&mut Self>) {
		unsafe {
			let device = self.client.device;
			let xurn = &mut self.get_unchecked_mut().xurn;
			if xurn.buffer.SerialNo != 0 {
				xurn.ioctl(device);
			}
		}
	}

	/// Polls the request for notifications.
	///
	/// If `wait` is true this method will block until a notification is received.
	/// Else returns immediately if no notification is received yet.
	///
	/// Returns:
	///
	/// * `Ok(None)`: When `wait` is false and there is no notification yet.
	/// * `Ok(Some(_))`: The notification was successfully received.  
	///   Another request should be made or any other calls to `poll` return the same result.
	/// * `Err(OperationAborted)`: The underlying target was unplugged causing any pending notification requests to abort.
	/// * `Err(_)`: An unexpected error occurred.
	#[inline(never)]
	pub fn poll(self: pin::Pin<&mut Self>, wait: bool) -> Result<Option<XNotification>, Error> {
		unsafe {
			let device = self.client.device;
			let xurn = &mut self.get_unchecked_mut().xurn;
			match xurn.poll(device, wait) {
				Ok(()) => Ok(Some(XNotification {
					large_motor: xurn.buffer.LargeMotor,
					small_motor: xurn.buffer.SmallMotor,
					led_number: xurn.buffer.LedNumber,
				})),
				Err(winerror::ERROR_IO_INCOMPLETE) => Ok(None),
				Err(winerror::ERROR_OPERATION_ABORTED) => {
					// Operation was aborted, fail all future calls
					// The is aborted when the underlying target is unplugged
					// This has the potential for a race condition:
					//  What happens if a new target is plugged inbetween calls to poll and request...
					xurn.buffer.SerialNo = 0;
					Err(Error::OperationAborted)
				},
				Err(err) => Err(Error::WinError(err)),
			}
		}
	}
}

#[cfg(feature = "unstable_xtarget_notification")]
unsafe impl Sync for XRequestNotification {}
#[cfg(feature = "unstable_xtarget_notification")]
unsafe impl Send for XRequestNotification {}

#[cfg(feature = "unstable_xtarget_notification")]
impl fmt::Debug for XRequestNotification {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("XRequestNotification")
			.field("client", &format_args!("{:?}", self.client))
			.field("serial_no", &self.xurn.buffer.SerialNo)
			.finish()
	}
}

#[cfg(feature = "unstable_xtarget_notification")]
impl Drop for XRequestNotification {
	fn drop(&mut self) {
		unsafe {
			let this = pin::Pin::new_unchecked(self);
			if this.xurn.buffer.SerialNo != 0 {
				let device = this.client.device;
				let xurn = &mut this.get_unchecked_mut().xurn;
				let _ = xurn.cancel(device);
			}
		}
	}
}

/// Virtual Microsoft Xbox 360 Controller (wired).
pub type XTarget = Xbox360Wired<Client>;

/// A virtual Microsoft Xbox 360 Controller (wired).
pub struct Xbox360Wired<CL: Borrow<Client>> {
	client: CL,
	event: Event,
	serial_no: u32,
	id: TargetId,
}

impl<CL: Borrow<Client>> Xbox360Wired<CL> {
	/// Creates a new instance.
	#[inline]
	pub fn new(client: CL, id: TargetId) -> Xbox360Wired<CL> {
		let event = Event::new(false, false);
		Xbox360Wired { client, event, serial_no: 0, id }
	}

	/// Returns if the controller is plugged in.
	#[inline]
	pub fn is_attached(&self) -> bool {
		self.serial_no != 0
	}

	/// Returns the vendor and product ids.
	#[inline]
	pub fn id(&self) -> TargetId {
		self.id
	}

	/// Returns the client.
	#[inline]
	pub fn client(&self) -> &CL {
		&self.client
	}

	/// Unplugs and destroys the controller, returning the client.
	#[inline]
	pub fn drop(mut self) -> CL {
		let _ = self.unplug();

		unsafe {
			let client = (&self.client as *const CL).read();
			ptr::drop_in_place(&mut self.event);
			mem::forget(self);
			client
		}
	}

	/// Plugs the controller in.
	#[inline(never)]
	pub fn plugin(&mut self) -> Result<(), Error> {
		if self.is_attached() {
			return Err(Error::AlreadyConnected);
		}

		let mut plugin = bus::PluginTarget::x360_wired(1, self.id.vendor, self.id.product);
		let device = self.client.borrow().device;

		// Yes this is how the driver is implemented
		while unsafe { plugin.ioctl(device, self.event.handle) }.is_err() {
			plugin.SerialNo += 1;
			if plugin.SerialNo >= u16::MAX as u32 {
				return Err(Error::NoFreeSlot);
			}
		}

		self.serial_no = plugin.SerialNo;
		Ok(())
	}

	/// Unplugs the controller.
	#[inline(never)]
	pub fn unplug(&mut self) -> Result<(), Error> {
		if !self.is_attached() {
			return Err(Error::NotPluggedIn);
		}

		unsafe {
			let mut unplug = bus::UnplugTarget::new(self.serial_no);
			let device = self.client.borrow().device;
			unplug.ioctl(device, self.event.handle)?;
		}

		self.serial_no = 0;
		Ok(())
	}

	/// Waits until the virtual controller is ready.
	///
	/// Any updates submitted before the virtual controller is ready may return an error.
	#[inline(never)]
	pub fn wait_ready(&mut self) -> Result<(), Error> {
		if !self.is_attached() {
			return Err(Error::NotPluggedIn);
		}

		unsafe {
			let mut wait = bus::WaitDeviceReady::new(self.serial_no);
			let device = self.client.borrow().device;
			wait.ioctl(device, self.event.handle)?;
		}

		Ok(())
	}

	/// Gets the user index of the device in XInput.
	#[inline(never)]
	pub fn get_user_index(&mut self) -> Result<u32, Error> {
		if !self.is_attached() {
			return Err(Error::NotPluggedIn);
		}

		let user_index = unsafe {
			let mut gui = bus::XUsbGetUserIndex::new(self.serial_no);
			let device = self.client.borrow().device;
			match gui.ioctl(device, self.event.handle) {
				Ok(()) => (),
				// Err(winerror::ERROR_ACCESS_DENIED) => return Err(Error::InvalidTarget),
				Err(winerror::ERROR_INVALID_DEVICE_OBJECT_PARAMETER) => return Err(Error::UserIndexOutOfRange),
				Err(err) => return Err(Error::WinError(err)),
			}

			gui.UserIndex
		};

		Ok(user_index)
	}

	/// Updates the virtual controller state.
	#[inline(never)]
	pub fn update(&mut self, gamepad: &XGamepad) -> Result<(), Error> {
		if !self.is_attached() {
			return Err(Error::NotPluggedIn);
		}

		unsafe {
			let mut xsr = bus::XUsbSubmitReport::new(self.serial_no, *gamepad);
			let device = self.client.borrow().device;
			match xsr.ioctl(device, self.event.handle) {
				Ok(()) => Ok(()),
				Err(winerror::ERROR_DEV_NOT_EXIST) => Err(Error::TargetNotReady),
				Err(err) => Err(Error::WinError(err)),
			}
		}
	}

	/// Request notification.
	///
	/// See examples/notification.rs for a complete example how to use this interface.
	///
	/// Do not create more than one request notification per target.
	/// Notifications may get lost or received by one or more listeners.
	#[cfg(feature = "unstable_xtarget_notification")]
	#[inline(never)]
	pub fn request_notification(&mut self) -> Result<XRequestNotification, Error> {
		if !self.is_attached() {
			return Err(Error::NotPluggedIn);
		}

		let client = self.client.borrow().try_clone()?;
		let xurn = bus::RequestNotification::new(
			bus::XUsbRequestNotification::new(self.serial_no));

		Ok(XRequestNotification { client, xurn, _unpin: marker::PhantomPinned })
	}
}

impl<CL: Borrow<Client>> fmt::Debug for Xbox360Wired<CL> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Xbox360Wired")
			.field("client", &format_args!("{:?}", self.client.borrow()))
			.field("event", &format_args!("{:?}", self.event))
			.field("serial_no", &self.serial_no)
			.field("vendor_id", &self.id.vendor)
			.field("product_id", &self.id.product)
			.finish()
	}
}

impl<CL: Borrow<Client>> Drop for Xbox360Wired<CL> {
	#[inline]
	fn drop(&mut self) {
		let _ = self.unplug();
	}
}
