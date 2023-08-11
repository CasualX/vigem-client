#![allow(non_snake_case)]

use std::{mem, ptr};
use winapi::um::handleapi::*;
use winapi::um::ioapiset::*;
use winapi::um::minwinbase::*;
use winapi::um::synchapi::*;
use winapi::um::errhandlingapi::*;
use winapi::shared::winerror;
use winapi::shared::ntdef::HANDLE;
use winapi::shared::guiddef::GUID;

pub static GUID_DEVINTERFACE: GUID = GUID {
	Data1: 0x96E42B22, Data2: 0xF5E9, Data3: 0x42F8,
	Data4: [0xB0, 0x43, 0xED, 0x0F, 0x93, 0x2F, 0x01, 0x4F],
};

// IO control codes
// const IOCTL_BASE: u32 = 0x801;
pub const IOCTL_PLUGIN_TARGET: u32 = 0x2AA004; //IOCTL_BASE + 0x000;
pub const IOCTL_UNPLUG_TARGET: u32 = 0x2AA008; //IOCTL_BASE + 0x001;
pub const IOCTL_CHECK_VERSION: u32 = 0x2AA00C; //IOCTL_BASE + 0x002;
pub const IOCTL_WAIT_DEVICE_READY: u32 = 0x2AA010; //IOCTL_BASE + 0x003;
#[cfg(feature = "unstable_xtarget_notification")]
pub const IOCTL_XUSB_REQUEST_NOTIFICATION : u32 = 0x2AE804; //IOCTL_BASE + 0x200 (RW);
pub const IOCTL_XUSB_SUBMIT_REPORT: u32 = 0x2AA808; //IOCTL_BASE + 0x201;
pub const IOCTL_DS4_SUBMIT_REPORT: u32 = 0x2AA80C; //IOCTL_BASE + 0x202;
pub const IOCTL_XUSB_GET_USER_INDEX: u32 = 0x2AE81C; //IOCTL_BASE + 0x206;

#[repr(C)]
pub struct CheckVersion {
	pub Size: u32,
	pub Version: u32,
}
impl CheckVersion {
	pub const COMMON: u32 = 0x0001;
	#[inline]
	pub const fn common() -> CheckVersion {
		CheckVersion {
			Size: mem::size_of::<CheckVersion>() as u32,
			Version: Self::COMMON,
		}
	}
	#[inline]
	pub unsafe fn ioctl(&mut self, device: HANDLE) -> bool {
		let mut transferred = 0;
		let mut overlapped: OVERLAPPED = mem::zeroed();
		overlapped.hEvent = CreateEventW(ptr::null_mut(), 0, 0, ptr::null());

		DeviceIoControl(
			device,
			IOCTL_CHECK_VERSION,
			self as *mut _ as _,
			mem::size_of_val(self) as u32,
			ptr::null_mut(),
			0,
			&mut transferred,
			&mut overlapped);

		let success = GetOverlappedResult(device, &mut overlapped, &mut transferred, /*bWait: */1);
		CloseHandle(overlapped.hEvent);
		return success != 0;
	}
}

pub const TARGET_TYPE_XBOX360_WIRED: i32 = 0;
pub const TARGET_TYPE_DUALSHOCK4_WIRED: i32 = 2;

#[repr(C)]
pub struct PluginTarget {
	pub Size: u32,
	pub SerialNo: u32,
	pub TargetType: i32,
	pub VendorId: u16,
	pub ProductId: u16,
}
impl PluginTarget {
	#[inline]
	pub const fn new(serial_no: u32, target_type: i32, vendor_id: u16, product_id: u16) -> PluginTarget {
		PluginTarget {
			Size: mem::size_of::<PluginTarget>() as u32,
			SerialNo: serial_no,
			TargetType: target_type,
			VendorId: vendor_id,
			ProductId: product_id,
		}
	}
	#[inline]
	pub const fn x360_wired(serial_no: u32, vendor_id: u16, product_id: u16) -> PluginTarget {
		PluginTarget::new(serial_no, TARGET_TYPE_XBOX360_WIRED, vendor_id, product_id)
	}
	#[inline]
	pub const fn ds4_wired(serial_no: u32, vendor_id: u16, product_id: u16) -> PluginTarget {
		PluginTarget::new(serial_no, TARGET_TYPE_DUALSHOCK4_WIRED, vendor_id, product_id)
	}
	#[inline]
	pub unsafe fn ioctl(&mut self, device: HANDLE, event: HANDLE) -> Result<(), u32> {
		let mut transferred = 0;
		let mut overlapped: OVERLAPPED = mem::zeroed();
		overlapped.hEvent = event;

		DeviceIoControl(
			device,
			IOCTL_PLUGIN_TARGET,
			self as *mut _ as _,
			mem::size_of_val(self) as u32,
			ptr::null_mut(),
			0,
			&mut transferred,
			&mut overlapped);

		if GetOverlappedResult(device, &mut overlapped, &mut transferred, /*bWait: */1) == 0 {
			return Err(GetLastError());
		}

		Ok(())
	}
}

#[repr(C)]
pub struct WaitDeviceReady {
	pub Size: u32,
	pub SerialNo: u32,
}
impl WaitDeviceReady {
	#[inline]
	pub const fn new(serial_no: u32) -> WaitDeviceReady {
		WaitDeviceReady {
			Size: mem::size_of::<WaitDeviceReady>() as u32,
			SerialNo: serial_no,
		}
	}
	#[inline]
	pub unsafe fn ioctl(&mut self, device: HANDLE, event: HANDLE) -> Result<(), u32> {
		let mut transferred = 0;
		let mut overlapped: OVERLAPPED = mem::zeroed();
		overlapped.hEvent = event;

		DeviceIoControl(
			device,
			IOCTL_WAIT_DEVICE_READY,
			self as *mut _ as _,
			mem::size_of_val(self) as u32,
			ptr::null_mut(),
			0,
			&mut transferred,
			&mut overlapped);

		if GetOverlappedResult(device, &mut overlapped, &mut transferred, /*bWait: */1) == 0 {
			let err = GetLastError();
			// Version pre-1.17 where this IOCTL doesn't exist
			if err != winerror::ERROR_INVALID_PARAMETER {
				return Err(err);
			}
		}

		Ok(())
	}
}

#[repr(C)]
pub struct UnplugTarget {
	pub Size: u32,
	pub SerialNo: u32,
}
impl UnplugTarget {
	#[inline]
	pub const fn new(serial_no: u32) -> UnplugTarget {
		UnplugTarget {
			Size: mem::size_of::<UnplugTarget>() as u32,
			SerialNo: serial_no,
		}
	}
	#[inline]
	pub unsafe fn ioctl(&mut self, device: HANDLE, event: HANDLE) -> Result<(), u32> {
		let mut transferred = 0;
		let mut overlapped: OVERLAPPED = mem::zeroed();
		overlapped.hEvent = event;

		DeviceIoControl(
			device,
			IOCTL_UNPLUG_TARGET,
			self as *mut _ as _,
			mem::size_of_val(self) as u32,
			ptr::null_mut(),
			0,
			&mut transferred,
			&mut overlapped);

		if GetOverlappedResult(device, &mut overlapped, &mut transferred, /*bWait: */1) == 0 {
			return Err(GetLastError());
		}

		Ok(())
	}
}

#[repr(C)]
pub struct XUsbSubmitReport {
	pub Size: u32,
	pub SerialNo: u32,
	pub Report: crate::XGamepad,
}
impl XUsbSubmitReport {
	#[inline]
	pub const fn new(serial_no: u32, report: crate::XGamepad) -> XUsbSubmitReport {
		XUsbSubmitReport {
			Size: mem::size_of::<XUsbSubmitReport>() as u32,
			SerialNo: serial_no,
			Report: report,
		}
	}
	#[inline]
	pub unsafe fn ioctl(&mut self, device: HANDLE, event: HANDLE) -> Result<(), u32> {
		let mut transferred = 0;
		let mut overlapped: OVERLAPPED = mem::zeroed();
		overlapped.hEvent = event;

		DeviceIoControl(
			device,
			IOCTL_XUSB_SUBMIT_REPORT,
			self as *mut _ as _,
			mem::size_of_val(self) as u32,
			ptr::null_mut(),
			0,
			&mut transferred,
			&mut overlapped);

		if GetOverlappedResult(device, &mut overlapped, &mut transferred, /*bWait: */1) == 0 {
			return Err(GetLastError());
		}

		Ok(())
	}
}

#[cfg(feature = "unstable_xtarget_notification")]
#[repr(C)]
pub struct XUsbRequestNotification {
	pub Size: u32,
	pub SerialNo: u32,
	pub LargeMotor: u8,
	pub SmallMotor: u8,
	pub LedNumber: u8,
}

#[cfg(feature = "unstable_xtarget_notification")]
impl XUsbRequestNotification {
	#[inline]
	pub const fn new(serial_no: u32) -> XUsbRequestNotification {
		XUsbRequestNotification {
			Size: mem::size_of::<XUsbRequestNotification>() as u32,
			SerialNo: serial_no,
			LargeMotor: 0,
			SmallMotor: 0,
			LedNumber: 0,
		}
	}
}

#[cfg(feature = "unstable_xtarget_notification")]
#[repr(C)]
pub struct RequestNotification<T> {
	pub overlapped: OVERLAPPED,
	pub buffer: T,
}
// Safety: This instance must have a stable address (eg. on the heap)
// Required for non-blocking DeviceIoControl, see msdn.
#[cfg(feature = "unstable_xtarget_notification")]
impl<T> RequestNotification<T> {
	#[inline]
	pub fn new(buffer: T) -> RequestNotification<T> {
		let mut overlapped: OVERLAPPED = unsafe { mem::zeroed() };
		overlapped.hEvent = unsafe { CreateEventW(ptr::null_mut(), 0, 0, ptr::null()) };
		RequestNotification { overlapped, buffer }
	}
	#[inline]
	pub unsafe fn ioctl(&mut self, device: HANDLE) {
		let mut transferred = 0;

		let buffer_ptr = &mut self.buffer as *mut _ as _;
		let buffer_size = mem::size_of::<T>() as u32;

		DeviceIoControl(
			device,
			IOCTL_XUSB_REQUEST_NOTIFICATION,
			buffer_ptr,
			buffer_size,
			buffer_ptr,
			buffer_size,
			&mut transferred,
			&mut self.overlapped);
	}
	#[inline]
	pub unsafe fn cancel(&mut self, device: HANDLE) -> Result<(), u32> {
		if CancelIoEx(device, &mut self.overlapped) == 0 {
			let err = GetLastError();
			// If no pending IO then everything is fine
			if err == winerror::ERROR_NOT_FOUND {
				return Ok(());
			}
			return Err(err);
		}
		let mut transferred = 0;
		if GetOverlappedResult(device, &mut self.overlapped, &mut transferred, /*bWait: */1) == 0 {
			let err = GetLastError();
			// Expect the operation to be aborted
			if err != winerror::ERROR_OPERATION_ABORTED {
				return Err(err);
			}
		}
		Ok(())
	}
	#[inline]
	pub unsafe fn poll(&mut self, device: HANDLE, wait: bool) -> Result<(), u32> {
		let mut transferred = 0;
		if GetOverlappedResult(device, &mut self.overlapped, &mut transferred, wait as i32) == 0 {
			return Err(GetLastError());
		}
		Ok(())
	}
}
#[cfg(feature = "unstable_xtarget_notification")]
impl<T> Drop for RequestNotification<T> {
	fn drop(&mut self) {
		unsafe { CloseHandle(self.overlapped.hEvent); }
	}
}

#[repr(C)]
pub struct DS4SubmitReport {
	pub Size: u32,
	pub SerialNo: u32,
	pub Report: crate::DS4Report,
}
impl DS4SubmitReport {
	#[inline]
	pub const fn new(serial_no: u32, report: crate::DS4Report) -> DS4SubmitReport {
		DS4SubmitReport {
			Size: mem::size_of::<DS4SubmitReport>() as u32,
			SerialNo: serial_no,
			Report: report,
		}
	}
	#[inline]
	pub unsafe fn ioctl(&mut self, device: HANDLE, event: HANDLE) -> Result<(), u32> {
		let mut transferred = 0;
		let mut overlapped: OVERLAPPED = mem::zeroed();
		overlapped.hEvent = event;

		DeviceIoControl(
			device,
			IOCTL_DS4_SUBMIT_REPORT,
			self as *mut _ as _,
			mem::size_of_val(self) as u32,
			ptr::null_mut(),
			0,
			&mut transferred,
			&mut overlapped);

		if GetOverlappedResult(device, &mut overlapped, &mut transferred, /*bWait: */1) == 0 {
			return Err(GetLastError());
		}

		Ok(())
	}
}

#[repr(C, packed)]
pub struct DS4SubmitReportEx {
    pub Size: u32,
    pub SerialNo: u32,
    pub Report: crate::DS4ReportEx,
}
impl DS4SubmitReportEx {
    #[inline]
    pub const fn new(serial_no: u32, report: crate::DS4ReportEx) -> DS4SubmitReportEx {
        DS4SubmitReportEx {
            Size: mem::size_of::<DS4SubmitReportEx>() as u32,
            SerialNo: serial_no,
            Report: report,
        }
    }
    #[inline]
    pub unsafe fn ioctl(&mut self, device: HANDLE, event: HANDLE) -> Result<(), u32> {
        let mut transferred = 0;
        let mut overlapped: OVERLAPPED = mem::zeroed();
        overlapped.hEvent = event;

        DeviceIoControl(
            device,
            IOCTL_DS4_SUBMIT_REPORT,
            self as *mut _ as _,
            mem::size_of_val(self) as u32,
            ptr::null_mut(),
            0,
            &mut transferred,
            &mut overlapped,
        );

        if GetOverlappedResult(
            device,
            &mut overlapped,
            &mut transferred,
            /*bWait: */ 1,
        ) == 0
        {
            return Err(GetLastError());
        }

        Ok(())
    }
}


#[repr(C)]
pub struct XUsbGetUserIndex {
	pub Size: u32,
	pub SerialNo: u32,
	pub UserIndex: u32,
}
impl XUsbGetUserIndex {
	#[inline]
	pub const fn new(serial_no: u32) -> XUsbGetUserIndex {
		XUsbGetUserIndex {
			Size: mem::size_of::<XUsbGetUserIndex>() as u32,
			SerialNo: serial_no,
			UserIndex: 0,
		}
	}
	#[inline]
	pub unsafe fn ioctl(&mut self, device: HANDLE, event: HANDLE) -> Result<(), u32> {
		let mut transferred = 0;
		let mut overlapped: OVERLAPPED = mem::zeroed();
		overlapped.hEvent = event;

		DeviceIoControl(
			device,
			IOCTL_XUSB_GET_USER_INDEX,
			self as *mut _ as _,
			mem::size_of_val(self) as u32,
			self as *mut _ as _,
			mem::size_of_val(self) as u32,
			&mut transferred,
			&mut overlapped);

		if GetOverlappedResult(device, &mut overlapped, &mut transferred, /*bWait: */1) == 0 {
			return Err(GetLastError());
		}

		Ok(())
	}
}
