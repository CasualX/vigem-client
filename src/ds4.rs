use std::{fmt, mem, ptr};
use std::borrow::Borrow;
use crate::*;

/// DualShock4 HID Input report.
#[cfg(feature = "unstable_ds4")]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct DS4Report {
	pub thumb_lx: u8,
	pub thumb_ly: u8,
	pub thumb_rx: u8,
	pub thumb_ry: u8,
	pub buttons: u16,
	pub special: u8,
	pub trigger_l: u8,
	pub trigger_r: u8,
}
#[cfg(feature = "unstable_ds4")]
impl Default for DS4Report {
	#[inline]
	fn default() -> Self {
		DS4Report {
			thumb_lx: 0x80,
			thumb_ly: 0x80,
			thumb_rx: 0x80,
			thumb_ry: 0x80,
			buttons: 0x8,
			special: 0,
			trigger_l: 0,
			trigger_r: 0,
		}
	}
}

#[cfg(feature = "unstable_ds4")]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct DS4Touch {
    pub packet_counter: u8, // timestamp / packet counter associated with touch event
    pub is_up_tracking_num1: u8, // 0 means down; active low
    // unique to each finger down, so for a lift and repress the value is incremented
    pub touch_data_1: [u8; 3], // Two 12 bits values (for X and Y)
    // middle byte holds last 4 bits of X and the starting 4 bits of Y
    pub is_up_tracking_num2: u8, // second touch data immediately follows data of first touch
    pub touch_data_2: [u8; 3],   // resolution is 1920x943
}

/// DualShock4 v1 complete HID Input report.
#[cfg(feature = "unstable_ds4")]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C, packed)]
pub struct DS4ReportEx {
    pub thumb_lx: u8,
    pub thumb_ly: u8,
    pub thumb_rx: u8,
    pub thumb_ry: u8,
    pub buttons: u16,
    pub special: u8,
    pub trigger_l: u8,
    pub trigger_r: u8,
    pub timestamp: u16,
    pub battery_lvl: u8,
    pub gyro_x: i16,
    pub gyro_y: i16,
    pub gyro_z: i16,
    pub accel_x: i16,
    pub accel_y: i16,
    pub accel_z: i16,
    pub _unknown1: [u8; 5],
    pub battery_lvl_special: u8,
    pub _unknown2: [u8; 2],
    pub touch_packets_n: u8, // 0x00 to 0x03 (USB max)
    pub current_touch: DS4Touch,
    pub previous_touch: [DS4Touch; 2],
    pub _end_padding: [u8; 3], /* This struct is normally used as an union member with another member of 63 bytes,
                        we bypass this by setting directly the padding in the struct */
}

#[cfg(feature = "unstable_ds4")]
impl Default for DS4ReportEx {
    #[inline]
    fn default() -> Self {
        DS4ReportEx {
            thumb_lx: 0x80,
            thumb_ly: 0x80,
            thumb_rx: 0x80,
            thumb_ry: 0x80,
            buttons: 0x8,
            special: 0,
            trigger_l: 0,
            trigger_r: 0,
            timestamp: 0,
            battery_lvl: 0,
            gyro_x: 0,
            gyro_y: 0,
            gyro_z: 0,
            accel_x: 0,
            accel_y: 0,
            accel_z: 0,
            _unknown1: [0; 5],
            battery_lvl_special: 0,
            _unknown2: [0; 2],
            touch_packets_n: 0,
            current_touch: DS4Touch {
                packet_counter: 0,
                is_up_tracking_num1: 0,
                touch_data_1: [0; 3],
                is_up_tracking_num2: 0,
                touch_data_2: [0; 3],
            },
            previous_touch: [DS4Touch {
                packet_counter: 0,
                is_up_tracking_num1: 0,
                touch_data_1: [0; 3],
                is_up_tracking_num2: 0,
                touch_data_2: [0; 3],
            }; 2],
            _end_padding: [0; 3],
        }
    }
}

/// A virtual Sony DualShock 4 (wired).
pub struct DualShock4Wired<CL: Borrow<Client>> {
	client: CL,
	event: Event,
	serial_no: u32,
	id: TargetId,
}

impl<CL: Borrow<Client>> DualShock4Wired<CL> {
	/// Creates a new instance.
	#[inline]
	pub fn new(client: CL, id: TargetId) -> DualShock4Wired<CL> {
		let event = Event::new(false, false);
		DualShock4Wired { client, event, serial_no: 0, id }
	}

	/// Returns if the controller is plugged in.
	#[inline]
	pub fn is_attached(&self) -> bool {
		self.serial_no != 0
	}

	/// Returns the id the controller was constructed with.
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

		self.serial_no = unsafe {
			let mut plugin = bus::PluginTarget::ds4_wired(1, self.id.vendor, self.id.product);
			let device = self.client.borrow().device;

			// Yes this is how the driver is implemented
			while plugin.ioctl(device, self.event.handle).is_err() {
				plugin.SerialNo += 1;
				if plugin.SerialNo >= u16::MAX as u32 {
					return Err(Error::NoFreeSlot);
				}
			}

			plugin.SerialNo
		};

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

	/// Updates the virtual controller state.
	#[cfg(feature = "unstable_ds4")]
	#[inline(never)]
	pub fn update(&mut self, report: &DS4Report) -> Result<(), Error> {
		if !self.is_attached() {
			return Err(Error::NotPluggedIn);
		}

		unsafe {
			let mut dsr = bus::DS4SubmitReport::new(self.serial_no, *report);
			let device = self.client.borrow().device;
			dsr.ioctl(device, self.event.handle)?;
		}

		Ok(())
	}

    #[inline(never)]
    #[cfg(feature = "unstable_ds4")]
    pub fn update_ex(&mut self, report: &DS4ReportEx) -> Result<(), Error> {
        if !self.is_attached() {
            return Err(Error::NotPluggedIn);
        }

        unsafe {
            let mut dsr = bus::DS4SubmitReportEx::new(self.serial_no, *report);
            let device = self.client.borrow().device;
            dsr.ioctl(device, self.event.handle)?;
        }

        Ok(())
    }
}

impl<CL: Borrow<Client>> fmt::Debug for DualShock4Wired<CL> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("DualShock4Wired")
			.field("serial_no", &self.serial_no)
			.field("vendor_id", &self.id.vendor)
			.field("product_id", &self.id.product)
			.finish()
	}
}

impl<CL: Borrow<Client>> Drop for DualShock4Wired<CL> {
	#[inline]
	fn drop(&mut self) {
		let _ = self.unplug();
	}
}
