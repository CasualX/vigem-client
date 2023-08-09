use crate::*;
use std::borrow::Borrow;
use std::{fmt, mem, ptr};

mod button;
mod reports;

pub use button::*;
pub use reports::*;

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
        DualShock4Wired {
            client,
            event,
            serial_no: 0,
            id,
        }
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

    /// Updates the virtual controller state using the extended report.
    #[inline(never)]
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
