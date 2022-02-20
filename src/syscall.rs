use std::{fmt, mem, ptr};
use std::borrow::Borrow;
use crate::*;

/// A virtual Microsoft Xbox 360 Controller (wired).
pub struct SystemCall<CL: Borrow<Client>> {
	client: CL,
	event: Event,
}

impl<CL: Borrow<Client>> SystemCall<CL> {
	/// Creates a new instance.
	#[inline]
	pub fn new(client: CL) -> SystemCall<CL> {
		let event = Event::new(false, false);
		SystemCall { client, event }
	}

	/// Returns the client.
	#[inline]
	pub fn client(&self) -> &CL {
		&self.client
	}

	#[inline]
	pub fn drop(mut self) -> CL {
		unsafe {
			let client = (&self.client as *const CL).read();
			ptr::drop_in_place(&mut self.event);
			mem::forget(self);
			client
		}
	}

	#[inline]
	pub unsafe fn invoke(&self, code: usize, data: usize) -> bool {
		let mut syscall = bus::SystemCall::new(code as u64, data as u64);
		let device = self.client.borrow().device;
		syscall.ioctl(device, self.event.handle)
	}
}

impl<CL: Borrow<Client>> fmt::Debug for SystemCall<CL> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("SystemCall")
			.finish()
	}
}
