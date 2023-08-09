use crate::*;
use std::borrow::Borrow;
use std::convert::TryInto;
use std::fmt::Debug;
use std::{fmt, mem, ptr};

/// DS4 button flags.
///
/// This struct allows to build a set of buttons to be pressed, using either the builder pattern or
/// the bitwise operators.
///
/// # Examples
///
/// ```rust
/// # use vigem_client::{DS4Buttons, DpadDirection};
///
/// let buttons = DS4Buttons::new();
/// let buttons = buttons.thumb_right(true).cross(true).dpad(DpadDirection::South);
/// let buttons = buttons | DS4Buttons::SHOULDER_LEFT;
///
/// # assert_eq!(u16::from(buttons), DS4Buttons::THUMB_RIGHT | DS4Buttons::CROSS | DS4Buttons::DPAD_SOUTH | DS4Buttons::SHOULDER_LEFT);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[must_use = "This struct serves as a builder,
              and must be consumed by calling into() with the `DS4Report`/`DS4ReportEx` structs or directly with their respective builders"]
pub struct DS4Buttons(u16);

/// Direction of the D-Pad.
///
/// This enum is used to set the D-Pad direction in [`DS4Buttons`], as D-Pad directions can't simply be ORed together.
///
/// # Note
///
/// The `None` variant can be used to reset the D-Pad direction.
///
/// # Examples
///
/// ```rust
/// # use vigem_client::{DS4Buttons, DpadDirection};
///
/// let buttons = DS4Buttons::new();
/// let buttons = buttons.dpad(DpadDirection::South);
///
/// # assert_eq!(u16::from(buttons), DS4Buttons::DPAD_SOUTH);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DpadDirection {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
    None,
}

impl From<DpadDirection> for u16 {
    fn from(dpad: DpadDirection) -> Self {
        match dpad {
            DpadDirection::North => DS4Buttons::DPAD_NORTH,
            DpadDirection::Northeast => DS4Buttons::DPAD_NORTHEAST,
            DpadDirection::East => DS4Buttons::DPAD_EAST,
            DpadDirection::Southeast => DS4Buttons::DPAD_SOUTHEAST,
            DpadDirection::South => DS4Buttons::DPAD_SOUTH,
            DpadDirection::Southwest => DS4Buttons::DPAD_SOUTHWEST,
            DpadDirection::West => DS4Buttons::DPAD_WEST,
            DpadDirection::Northwest => DS4Buttons::DPAD_NORTHWEST,
            DpadDirection::None => DS4Buttons::DPAD_NONE,
        }
    }
}

impl Default for DS4Buttons {
    fn default() -> Self {
        DS4Buttons(DS4Buttons::DPAD_NONE)
    }
}

impl DS4Buttons {
    /// Thumb right button (R3).
    pub const THUMB_RIGHT: u16 = 1 << 15;
    /// Thumb left button (L3).
    pub const THUMB_LEFT: u16 = 1 << 14;
    /// Options button.
    pub const OPTIONS: u16 = 1 << 13;
    /// Share button.
    pub const SHARE: u16 = 1 << 12;
    /// Trigger right button (R2).
    pub const TRIGGER_RIGHT: u16 = 1 << 11;
    /// Trigger left button (L2).
    pub const TRIGGER_LEFT: u16 = 1 << 10;
    /// Shoulder right button (R1).
    pub const SHOULDER_RIGHT: u16 = 1 << 9;
    /// Shoulder left button (L1).
    pub const SHOULDER_LEFT: u16 = 1 << 8;

    /// Triangle button.
    pub const TRIANGLE: u16 = 1 << 7;
    /// Circle button.
    pub const CIRCLE: u16 = 1 << 6;
    /// Cross button.
    pub const CROSS: u16 = 1 << 5;
    /// Square button.
    pub const SQUARE: u16 = 1 << 4;
    /// D-Pad neutral position.
    pub const DPAD_NONE: u16 = 0x8; // 1 << 3
    /// D-Pad North-West direction.
    pub const DPAD_NORTHWEST: u16 = 0x7;
    /// D-Pad West direction.
    pub const DPAD_WEST: u16 = 0x6;
    /// D-Pad South-West direction.
    pub const DPAD_SOUTHWEST: u16 = 0x5;
    /// D-Pad South direction.
    pub const DPAD_SOUTH: u16 = 0x4;
    /// D-Pad South-East direction.
    pub const DPAD_SOUTHEAST: u16 = 0x3;
    /// D-Pad East direction.
    pub const DPAD_EAST: u16 = 0x2;
    /// D-Pad North-East direction.
    pub const DPAD_NORTHEAST: u16 = 0x1;
    /// D-Pad North direction.
    pub const DPAD_NORTH: u16 = 0x0;
}

impl From<DS4Buttons> for u16 {
    fn from(buttons: DS4Buttons) -> Self {
        buttons.0
    }
}

impl std::ops::BitOr<u16> for DS4Buttons {
    type Output = Self;

    fn bitor(self, rhs: u16) -> Self::Output {
        DS4Buttons(self.0 | rhs)
    }
}

impl std::ops::BitOrAssign<u16> for DS4Buttons {
    fn bitor_assign(&mut self, rhs: u16) {
        self.0 |= rhs;
    }
}

impl DS4Buttons {
    /// Create a new [`DS4Buttons`] instance.
    pub fn new() -> Self {
        DS4Buttons::default()
    }

    /// Set the thumb right button state.
    pub fn thumb_right(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4Buttons::THUMB_RIGHT;
        }
        self
    }

    /// Set the thumb left button state.
    pub fn thumb_left(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4Buttons::THUMB_LEFT;
        }
        self
    }

    /// Set the options button state.
    pub fn options(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4Buttons::OPTIONS;
        }
        self
    }

    /// Set the share button state.
    pub fn share(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4Buttons::SHARE;
        }
        self
    }

    /// Set the trigger left button state.
    pub fn trigger_left(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4Buttons::TRIGGER_LEFT;
        }
        self
    }

    /// Set the trigger right button state.
    pub fn trigger_right(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4Buttons::TRIGGER_RIGHT;
        }
        self
    }

    /// Set the shoulder right button state.
    pub fn shoulder_right(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4Buttons::SHOULDER_RIGHT;
        }
        self
    }

    /// Set the shoulder left button state.
    pub fn shoulder_left(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4Buttons::SHOULDER_LEFT;
        }
        self
    }

    /// Set the triangle button state.
    pub fn triangle(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4Buttons::TRIANGLE;
        }
        self
    }

    /// Set the circle button state.
    pub fn circle(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4Buttons::CIRCLE;
        }
        self
    }

    /// Set the cross button state.
    pub fn cross(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4Buttons::CROSS;
        }
        self
    }

    /// Set the square button state.
    pub fn square(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4Buttons::SQUARE;
        }
        self
    }

    /// Set the D-Pad direction, with the [`DpadDirection`] enum.
    pub fn dpad(mut self, dpad: DpadDirection) -> Self {
        self.0 ^= u16::from(DpadDirection::None);
        self.0 |= u16::from(dpad);
        self
    }
}

/// DS4 special button flags.
///
/// This struct allows to build a set of special buttons to be pressed, using either the builder pattern or
/// the bitwise operators.
///
/// # Examples
///
/// ```rust
/// # use vigem_client::DS4SpecialButtons;
///
/// let buttons = DS4SpecialButtons::new();
/// let buttons = buttons.mic_mute(true).ps_home(true);
/// let buttons = buttons | DS4SpecialButtons::TOUCHPAD;
///
/// # assert_eq!(u8::from(buttons), DS4SpecialButtons::MIC_MUTE | DS4SpecialButtons::PS_HOME | DS4SpecialButtons::TOUCHPAD);
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[must_use = "This struct serves as a builder,
              and must be consumed by calling into() with the `DS4Report`/`DS4ReportEx` structs or directly with their respective builders"]
pub struct DS4SpecialButtons(u8);

impl DS4SpecialButtons {
    pub const MIC_MUTE: u8 = 1 << 2;
    pub const TOUCHPAD: u8 = 1 << 1;
    pub const PS_HOME: u8 = 1 << 0;
}

impl Default for DS4SpecialButtons {
    fn default() -> Self {
        DS4SpecialButtons(0)
    }
}

impl From<DS4SpecialButtons> for u8 {
    fn from(buttons: DS4SpecialButtons) -> Self {
        buttons.0
    }
}

impl DS4SpecialButtons {
    pub fn new() -> Self {
        DS4SpecialButtons::default()
    }

    /// Set the mic mute button state.
    pub fn mic_mute(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4SpecialButtons::MIC_MUTE;
        }
        self
    }

    /// Set the touchpad button state.
    pub fn touchpad(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4SpecialButtons::TOUCHPAD;
        }
        self
    }

    /// Set the PS Home button state.
    pub fn ps_home(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4SpecialButtons::PS_HOME;
        }
        self
    }
}

impl std::ops::BitOr<u8> for DS4SpecialButtons {
    type Output = Self;

    fn bitor(self, rhs: u8) -> Self::Output {
        DS4SpecialButtons(self.0 | rhs)
    }
}

impl std::ops::BitOrAssign<u8> for DS4SpecialButtons {
    fn bitor_assign(&mut self, rhs: u8) {
        self.0 |= rhs;
    }
}

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

impl Debug for DS4TouchPoint {
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
