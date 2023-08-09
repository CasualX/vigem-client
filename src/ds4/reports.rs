//! DualShock4 HID reports.

use super::{DS4Buttons, DS4SpecialButtons};

use std::{convert::TryInto, fmt};

/// DualShock4 HID Input report.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct DS4Report {
    thumb_lx: u8,
    thumb_ly: u8,
    thumb_rx: u8,
    thumb_ry: u8,
    buttons: u16,
    special: u8,
    trigger_l: u8,
    trigger_r: u8,
}
impl Default for DS4Report {
    #[inline]
    fn default() -> Self {
        DS4Report {
            thumb_lx: 0x80,
            thumb_ly: 0x80,
            thumb_rx: 0x80,
            thumb_ry: 0x80,
            buttons: u16::from(DS4Buttons::default()),
            special: u8::from(DS4SpecialButtons::default()),
            trigger_l: 0,
            trigger_r: 0,
        }
    }
}

/// DualShock4 touch point.
/// The touch point is in the range 0..1920 for the X coordinate and 0..942 for the Y coordinate.
///
/// It is recommended to use [`DS4TouchPoint::new`] to create a new touch point.
///
/// # Examples
///
/// ```rust
/// # use vigem_client::DS4TouchPoint;
///
/// let point = DS4TouchPoint::new(true, 1920, 942);
/// ```
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(C, packed)]
pub struct DS4TouchPoint {
    /// Last bit is set if the touch point is inactive.
    contact: u8,
    x_lo: u8,
    x_hi_y_lo: u8, // 4 bits of x_hi, 4 bits of y_lo
    y_hi: u8,
}

impl fmt::Debug for DS4TouchPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DS4TouchPoint")
            .field("active", &self.is_active())
            .field("x", &self.x())
            .field("y", &self.y())
            .finish()
    }
}

impl DS4TouchPoint {
    /// Create a new touch point.
    pub fn new(active: bool, x: u16, y: u16) -> Self {
        DS4TouchPoint {
            contact: if active { 0 } else { 1 << 7 },
            x_lo: (x & 0xFF) as u8,
            x_hi_y_lo: (((x >> 8) & 0xF) << 4) as u8 | ((y & 0xF) as u8),
            y_hi: (y >> 4) as u8,
        }
    }

    /// Returns if the touch point is active.
    fn is_active(&self) -> bool {
        self.contact & (1 << 7) == 0
    }

    /// Get the X coordinate of the touch point.
    fn x(&self) -> u16 {
        ((self.x_hi_y_lo & 0xF0) as u16) << 4 | self.x_lo as u16
    }

    /// Get the Y coordinate of the touch point.
    fn y(&self) -> u16 {
        (self.y_hi as u16) << 4 | ((self.x_hi_y_lo & 0xF) as u16)
    }
}

impl Default for DS4TouchPoint {
    #[inline]
    fn default() -> Self {
        DS4TouchPoint {
            contact: 0,
            x_lo: 0,
            x_hi_y_lo: 0,
            y_hi: 0,
        }
    }
}

/// DualShock4 touch report.
/// The touch report contains two touch points.
///
/// It is recommended to use [`DS4TouchReport::new`] to create a new touch report.
///
/// # Examples
///
/// ```rust
/// # use vigem_client::{DS4TouchReport, DS4TouchPoint};
///
/// let report = DS4TouchReport::new(0, Some(DS4TouchPoint::new(true, 1920, 942)), None);
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct DS4TouchReport {
    pub timestamp: u8,
    pub points: [DS4TouchPoint; 2],
}

impl DS4TouchReport {
    /// Create a new touch report.
    pub fn new(
        timestamp: u8,
        point1: Option<DS4TouchPoint>,
        point2: Option<DS4TouchPoint>,
    ) -> Self {
        DS4TouchReport {
            timestamp,
            points: [point1.unwrap_or_default(), point2.unwrap_or_default()],
        }
    }

    /// Get the timestamp of the touch report.
    pub fn timestamp(&self) -> u8 {
        self.timestamp
    }

    /// Get the touch points of the touch report.
    pub fn points(&self) -> &[DS4TouchPoint; 2] {
        &self.points
    }
}

impl Default for DS4TouchReport {
    #[inline]
    fn default() -> Self {
        DS4TouchReport {
            timestamp: 0,
            points: [DS4TouchPoint::default(); 2],
        }
    }
}

/// DualShock4 v1 complete HID Input report.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C, packed)]
pub struct DS4ReportEx {
    thumb_lx: u8,
    thumb_ly: u8,
    thumb_rx: u8,
    thumb_ry: u8,
    buttons: u16,
    special: u8,
    trigger_l: u8,
    trigger_r: u8,
    timestamp: u16,
    temp: u8,
    gyro_x: i16,
    gyro_y: i16,
    gyro_z: i16,
    accel_x: i16,
    accel_y: i16,
    accel_z: i16,
    reserved2: [u8; 5],
    status: u16,
    reserved3: u8,
    num_touch_reports: u8,              // 0x00 to 0x03 (USB max)
    touch_reports: [DS4TouchReport; 3], // Most recent touch report first
    reserved: [u8; 3],
}

impl Default for DS4ReportEx {
    #[inline]
    fn default() -> Self {
        DS4ReportEx {
            thumb_lx: 0x80,
            thumb_ly: 0x80,
            thumb_rx: 0x80,
            thumb_ry: 0x80,
            buttons: DS4Buttons::default().into(),
            special: DS4SpecialButtons::default().into(),
            trigger_l: 0,
            trigger_r: 0,
            timestamp: 0,
            temp: 0,
            gyro_x: 0,
            gyro_y: 0,
            gyro_z: 0,
            accel_x: 0,
            accel_y: 0,
            accel_z: 0,
            reserved2: [0; 5],
            status: DS4Status::default().into(),
            reserved3: 0,
            num_touch_reports: 0,
            touch_reports: [DS4TouchReport::default(); 3],
            reserved: [0; 3],
        }
    }
}

// Builders for DS4 reports.

/// Battery status of the controller, mainly used for [`DS4Status`].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[must_use = "This enum serves as a builder,
              and must be consumed by using `DS4Status`"]
pub enum BatteryStatus {
    /// Charging, with the capacity in the range 0..10 reflecting the charge level in 10% steps.
    Charging(u8),
    Full,
    Error,
    Unknown,
}

impl From<BatteryStatus> for u16 {
    fn from(status: BatteryStatus) -> Self {
        match status {
            BatteryStatus::Charging(capacity) => (capacity.min(10)) as u16,
            BatteryStatus::Full => DS4Status::BATTERY_FULL,
            BatteryStatus::Error => DS4Status::CHARGE_ERROR,
            BatteryStatus::Unknown => DS4Status::NOT_CHARGING,
        }
    }
}

/// DS4 status flags.
///
/// The status reflects the battery status, the cable state and the dongle state.
/// It can be constructed using [`DS4Status::with_battery_status`].
///
/// # Examples
///
/// ```rust
/// # use vigem_client::{DS4Status, BatteryStatus};
///
/// let status = DS4Status::with_battery_status(BatteryStatus::Charging(5));
///
/// # assert_eq!(u16::from(status), DS4Status::CABLE_STATE | 5);
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DS4Status(u16);

impl DS4Status {
    const _DONGLE_STATE: u16 = 1 << 11; // 0 = not connected, 1 = connected

    pub const CABLE_STATE: u16 = 1 << 4;

    pub const BATTERY_FULL: u16 = 11; // battery is full
    pub const NOT_CHARGING: u16 = 14; // not charging due to Voltage or temperature error
    pub const CHARGE_ERROR: u16 = 15; // charge error
}

impl DS4Status {
    /// Create a new [`DS4Status`], with battery status `status` set either to:
    /// - the capacity, if `status` is in the range 0..10
    /// - a special status, if `status` is in the range 11..15:
    ///     - [`DS4Status::BATTERY_FULL`]: battery is full
    ///     - [`DS4Status::NOT_CHARGING`]: not charging due to Voltage or temperature error
    ///     - [`DS4Status::CHARGE_ERROR`]: charge error
    pub fn with_battery_status(status: BatteryStatus) -> Self {
        DS4Status(DS4Status::CABLE_STATE | u16::from(status))
    }
}

impl Default for DS4Status {
    fn default() -> Self {
        DS4Status(DS4Status::CABLE_STATE)
    }
}

impl From<DS4Status> for u16 {
    fn from(status: DS4Status) -> Self {
        status.0
    }
}

/// A builder for [`DS4Report`].
///
/// # Examples
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "This struct serves as a builder,
              and must be consumed by calling into()"]
pub struct DS4ReportBuilder {
    thumb_lx: Option<u8>,
    thumb_ly: Option<u8>,
    thumb_rx: Option<u8>,
    thumb_ry: Option<u8>,
    buttons: DS4Buttons,
    special: DS4SpecialButtons,
    trigger_l: Option<u8>,
    trigger_r: Option<u8>,
}

impl DS4ReportBuilder {
    /// Create a new builder.
    pub fn new() -> Self {
        DS4ReportBuilder {
            thumb_lx: None,
            thumb_ly: None,
            thumb_rx: None,
            thumb_ry: None,
            buttons: DS4Buttons::default(),
            special: DS4SpecialButtons::default(),
            trigger_l: None,
            trigger_r: None,
        }
    }

    /// Set the left thumb stick X axis.
    pub fn thumb_lx(mut self, value: u8) -> Self {
        self.thumb_lx = Some(value);
        self
    }

    /// Set the left thumb stick Y axis.
    pub fn thumb_ly(mut self, value: u8) -> Self {
        self.thumb_ly = Some(value);
        self
    }

    /// Set the right thumb stick X axis.
    pub fn thumb_rx(mut self, value: u8) -> Self {
        self.thumb_rx = Some(value);
        self
    }

    /// Set the right thumb stick Y axis.
    pub fn thumb_ry(mut self, value: u8) -> Self {
        self.thumb_ry = Some(value);
        self
    }

    /// Set the buttons.
    ///
    /// # Examples
    /// ```rust
    ///
    /// # use vigem_client::{DS4ReportBuilder, DS4Buttons};
    /// let report = DS4ReportBuilder::new().buttons(DS4Buttons::new().cross(true));
    /// ```
    pub fn buttons(mut self, value: DS4Buttons) -> Self {
        self.buttons = value;
        self
    }

    /// Set the special buttons.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use vigem_client::{DS4ReportBuilder, DS4SpecialButtons};
    ///
    /// let report = DS4ReportBuilder::new().special(DS4SpecialButtons::new().touchpad(true)).build();
    /// ```
    pub fn special(mut self, value: DS4SpecialButtons) -> Self {
        self.special = value;
        self
    }

    /// Set the left trigger.
    pub fn trigger_l(mut self, value: u8) -> Self {
        self.trigger_l = Some(value);
        self
    }

    /// Set the right trigger.
    pub fn trigger_r(mut self, value: u8) -> Self {
        self.trigger_r = Some(value);
        self
    }

    /// Build the report.
    pub fn build(self) -> DS4Report {
        DS4Report {
            thumb_lx: self.thumb_lx.unwrap_or(0x80),
            thumb_ly: self.thumb_ly.unwrap_or(0x80),
            thumb_rx: self.thumb_rx.unwrap_or(0x80),
            thumb_ry: self.thumb_ry.unwrap_or(0x80),
            buttons: self.buttons.into(),
            special: self.special.into(),
            trigger_l: self.trigger_l.unwrap_or(0),
            trigger_r: self.trigger_r.unwrap_or(0),
        }
    }
}

impl Default for DS4ReportBuilder {
    fn default() -> Self {
        DS4ReportBuilder::new()
    }
}

impl From<DS4ReportBuilder> for DS4Report {
    fn from(builder: DS4ReportBuilder) -> Self {
        builder.build()
    }
}

/// A builder for [`DS4ReportEx`].
///
/// # Examples
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "This struct serves as a builder,
              and must be consumed by calling into()"]
pub struct DS4ReportExBuilder {
    thumb_lx: Option<u8>,
    thumb_ly: Option<u8>,
    thumb_rx: Option<u8>,
    thumb_ry: Option<u8>,
    buttons: DS4Buttons,
    special: DS4SpecialButtons,
    trigger_l: Option<u8>,
    trigger_r: Option<u8>,
    timestamp: Option<u16>,
    temp: Option<u8>,
    gyro_x: Option<i16>,
    gyro_y: Option<i16>,
    gyro_z: Option<i16>,
    accel_x: Option<i16>,
    accel_y: Option<i16>,
    accel_z: Option<i16>,
    status: Option<DS4Status>,
    num_touch_reports: Option<u8>,
    touch_reports: [DS4TouchReport; 3],
}

impl DS4ReportExBuilder {
    pub fn new() -> Self {
        DS4ReportExBuilder {
            thumb_lx: None,
            thumb_ly: None,
            thumb_rx: None,
            thumb_ry: None,
            buttons: DS4Buttons::default(),
            special: DS4SpecialButtons::default(),
            trigger_l: None,
            trigger_r: None,
            timestamp: None,
            temp: None,
            gyro_x: None,
            gyro_y: None,
            gyro_z: None,
            accel_x: None,
            accel_y: None,
            accel_z: None,
            status: None,
            num_touch_reports: None,
            touch_reports: [DS4TouchReport::default(); 3],
        }
    }

    /// Set the left thumb stick X axis.
    pub fn thumb_lx(mut self, value: u8) -> Self {
        self.thumb_lx = Some(value);
        self
    }

    /// Set the left thumb stick Y axis.
    pub fn thumb_ly(mut self, value: u8) -> Self {
        self.thumb_ly = Some(value);
        self
    }

    /// Set the right thumb stick X axis.
    pub fn thumb_rx(mut self, value: u8) -> Self {
        self.thumb_rx = Some(value);
        self
    }

    /// Set the right thumb stick Y axis.
    pub fn thumb_ry(mut self, value: u8) -> Self {
        self.thumb_ry = Some(value);
        self
    }

    /// Set the buttons.
    pub fn buttons(mut self, value: DS4Buttons) -> Self {
        self.buttons = value;
        self
    }

    /// Set the special buttons.
    pub fn special(mut self, value: DS4SpecialButtons) -> Self {
        self.special = value;
        self
    }

    /// Set the left trigger.
    pub fn trigger_l(mut self, value: u8) -> Self {
        self.trigger_l = Some(value);
        self
    }

    /// Set the right trigger.
    pub fn trigger_r(mut self, value: u8) -> Self {
        self.trigger_r = Some(value);
        self
    }

    /// Set the timestamp.
    pub fn timestamp(mut self, value: u16) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Set the temperature.
    pub fn temp(mut self, value: u8) -> Self {
        self.temp = Some(value);
        self
    }

    /// Set the gyroscope X axis.
    pub fn gyro_x(mut self, value: i16) -> Self {
        self.gyro_x = Some(value);
        self
    }

    /// Set the gyroscope Y axis.
    pub fn gyro_y(mut self, value: i16) -> Self {
        self.gyro_y = Some(value);
        self
    }

    /// Set the gyroscope Z axis.
    pub fn gyro_z(mut self, value: i16) -> Self {
        self.gyro_z = Some(value);
        self
    }

    /// Set the accelerometer X axis.
    pub fn accel_x(mut self, value: i16) -> Self {
        self.accel_x = Some(value);
        self
    }

    /// Set the accelerometer Y axis.
    pub fn accel_y(mut self, value: i16) -> Self {
        self.accel_y = Some(value);
        self
    }

    /// Set the accelerometer Z axis.
    pub fn accel_z(mut self, value: i16) -> Self {
        self.accel_z = Some(value);
        self
    }

    /// Set the status.
    pub fn status(mut self, value: DS4Status) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the touch reports.
    pub fn touch_reports(mut self, value: &[DS4TouchReport]) -> Self {
        self.touch_reports = value
            .iter()
            .take(3)
            .chain(
                std::iter::repeat(&DS4TouchReport::default())
                    .take(3usize.saturating_sub(value.len())),
            )
            .copied()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        self
    }

    /// Build the report.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use vigem_client::{DS4ReportExBuilder, DS4ReportEx, DS4Buttons, DS4SpecialButtons, DS4Status, DS4TouchReport, DS4TouchPoint, BatteryStatus};
    ///
    /// let report = DS4ReportExBuilder::new()
    ///     .thumb_lx(0x80)
    ///     .thumb_rx(0x80)
    ///     .thumb_ry(0x80)
    ///     .buttons(DS4Buttons::new().cross(true).square(true))
    ///     .special(DS4SpecialButtons::new().touchpad(true))
    ///     .trigger_l(0)
    ///     .trigger_r(0)
    ///     .timestamp(0)
    ///     .accel_x(0)
    ///     .gyro_x(0)
    ///     .status(DS4Status::with_battery_status(BatteryStatus::Charging(9)))
    ///     .touch_reports(&[DS4TouchReport::new(0, Some(DS4TouchPoint::new(true, 1920, 942)), None)])
    ///     .build();
    /// ```
    pub fn build(self) -> DS4ReportEx {
        DS4ReportEx {
            thumb_lx: self.thumb_lx.unwrap_or(0x80),
            thumb_ly: self.thumb_ly.unwrap_or(0x80),
            thumb_rx: self.thumb_rx.unwrap_or(0x80),
            thumb_ry: self.thumb_ry.unwrap_or(0x80),
            buttons: self.buttons.into(),
            special: self.special.into(),
            trigger_l: self.trigger_l.unwrap_or(0),
            trigger_r: self.trigger_r.unwrap_or(0),
            timestamp: self.timestamp.unwrap_or(0),
            temp: self.temp.unwrap_or(0),
            gyro_x: self.gyro_x.unwrap_or(0),
            gyro_y: self.gyro_y.unwrap_or(0),
            gyro_z: self.gyro_z.unwrap_or(0),
            accel_x: self.accel_x.unwrap_or(0),
            accel_y: self.accel_y.unwrap_or(0),
            accel_z: self.accel_z.unwrap_or(0),
            reserved2: [0; 5],
            status: self.status.unwrap_or(DS4Status::default()).into(),
            reserved3: 0,
            num_touch_reports: self.num_touch_reports.unwrap_or(0),
            touch_reports: self.touch_reports,
            reserved: [0; 3],
        }
    }
}
