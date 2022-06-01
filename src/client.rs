use std::{mem, ptr};
use std::os::windows::io as win_io;
use winapi::um::handleapi::*;
use winapi::um::setupapi::*;
use winapi::um::fileapi::*;
use winapi::um::winnt::*;
use winapi::um::winbase::*;
use winapi::um::errhandlingapi::*;
use winapi::shared::ntdef::HANDLE;
use crate::*;

/// The ViGEmBus service connection.
#[derive(Debug)]
pub struct Client {
	pub(crate) device: HANDLE,
}

impl Client {
	/// Connects to the ViGEmBus service.
	pub fn connect() -> Result<Client, Error> {
		unsafe {
			let mut error = Error::BusNotFound;

			let mut member_index = 0;
			let mut device_interface_data: SP_DEVICE_INTERFACE_DATA = mem::zeroed();
			device_interface_data.cbSize = mem::size_of_val(&device_interface_data) as u32;

			let mut detail_data_buffer = mem::MaybeUninit::<[u32; 0x300]>::uninit();

			let device_info_set = SetupDiGetClassDevsW(
				&bus::GUID_DEVINTERFACE,
				ptr::null(),
				ptr::null_mut(),
				DIGCF_PRESENT | DIGCF_DEVICEINTERFACE);

			if device_info_set == INVALID_HANDLE_VALUE {
				return Err(Error::WinError(GetLastError()));
			}

			// Enumerate device instances
			while SetupDiEnumDeviceInterfaces(
				device_info_set,
				ptr::null_mut(),
				&bus::GUID_DEVINTERFACE,
				member_index,
				&mut device_interface_data) != 0
			{
				member_index += 1;

				// Allocate target buffer
				// This is a fixed size stack buffer which should be big enough for everyone
				let detail_data_ptr = detail_data_buffer.as_mut_ptr() as PSP_DEVICE_INTERFACE_DETAIL_DATA_W;
				(*detail_data_ptr).cbSize = mem::size_of::<SP_DEVICE_INTERFACE_DETAIL_DATA_W>() as u32;

				// Get detail buffer
				let mut required_size = 0;
				if SetupDiGetDeviceInterfaceDetailW(
					device_info_set,
					&mut device_interface_data,
					detail_data_ptr,
					mem::size_of_val(&detail_data_buffer) as u32,
					&mut required_size,
					ptr::null_mut()) == 0
				{
					error = Error::WinError(GetLastError());
					continue;
				}

				// bus found, open it
				let device_path = (*detail_data_ptr).DevicePath.as_ptr();
				let device = CreateFileW(
					device_path,
					GENERIC_READ | GENERIC_WRITE,
					FILE_SHARE_READ | FILE_SHARE_WRITE,
					ptr::null_mut(),
					OPEN_EXISTING,
					FILE_ATTRIBUTE_NORMAL | FILE_FLAG_NO_BUFFERING | FILE_FLAG_WRITE_THROUGH | FILE_FLAG_OVERLAPPED,
					ptr::null_mut());

				if device == INVALID_HANDLE_VALUE {
					error = Error::BusAccessFailed(GetLastError());
					continue;
				}

				let mut check_version = bus::CheckVersion::common();
				if check_version.ioctl(device) {
					SetupDiDestroyDeviceInfoList(device_info_set);
					return Ok(Client { device })
				}

				// version mismatch, look for another instance
				CloseHandle(device);
				error = Error::BusVersionMismatch;
			}

			SetupDiDestroyDeviceInfoList(device_info_set);
			Err(error)
		}
	}

	/// Duplicates the ViGEmBus service handle.
	#[inline]
	pub fn try_clone(&self) -> Result<Client, Error> {
		unsafe {
			let process_handle = (!0) as *mut _;
			let mut target_handle = mem::MaybeUninit::uninit();
			let success = DuplicateHandle(
				process_handle, self.device,
				process_handle, target_handle.as_mut_ptr(),
				GENERIC_READ | GENERIC_WRITE, 0, DUPLICATE_SAME_ACCESS);
			if success == 0 {
				let err = GetLastError();
				return Err(Error::WinError(err));
			}
			Ok(Client { device: target_handle.assume_init() })
		}
	}
}

unsafe impl Sync for Client {}
unsafe impl Send for Client {}

impl win_io::AsRawHandle for Client {
	#[inline]
	fn as_raw_handle(&self) -> HANDLE {
		self.device
	}
}
impl win_io::IntoRawHandle for Client {
	#[inline]
	fn into_raw_handle(self) -> HANDLE {
		self.device
	}
}
impl win_io::FromRawHandle for Client {
	#[inline]
	unsafe fn from_raw_handle(device: HANDLE) -> Client {
		Client { device }
	}
}

impl Drop for Client {
	#[inline]
	fn drop(&mut self) {
		unsafe {
			CloseHandle(self.device);
		}
	}
}
