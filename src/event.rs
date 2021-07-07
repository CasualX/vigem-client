use std::ptr;
use winapi::um::handleapi::*;
use winapi::um::synchapi::*;
use winapi::shared::ntdef::HANDLE;

#[repr(transparent)]
pub struct Event {
	pub(crate) handle: HANDLE,
}
impl Event {
	#[inline]
	pub fn new(manual_reset: bool, initial_state: bool) -> Event {
		unsafe {
			let handle = CreateEventW(ptr::null_mut(), manual_reset as i32, initial_state as i32, ptr::null());
			debug_assert!(!handle.is_null());
			Event { handle }
		}
	}
	// #[inline]
	// pub fn reset(&self) {
	// 	unsafe { ResetEvent(self.handle) };
	// }
}
impl Drop for Event {
	#[inline]
	fn drop(&mut self) {
		unsafe { CloseHandle(self.handle) };
	}
}
