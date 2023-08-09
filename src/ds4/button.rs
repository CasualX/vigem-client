//! DualShock 4 button components.

use std::fmt::Debug;

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
#[derive(Copy, Clone, Eq, PartialEq)]
#[must_use = "This struct serves as a builder,
              and must be consumed by calling into() with the `DS4Report`/`DS4ReportEx` structs or directly with their respective builders"]
pub struct DS4Buttons(pub(super) u16);

impl Debug for DS4Buttons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DS4Buttons")
            .field("thumb_right", &(self.0 & DS4Buttons::THUMB_RIGHT != 0))
            .field("thumb_left", &(self.0 & DS4Buttons::THUMB_LEFT != 0))
            .field("options", &(self.0 & DS4Buttons::OPTIONS != 0))
            .field("share", &(self.0 & DS4Buttons::SHARE != 0))
            .field("trigger_right", &(self.0 & DS4Buttons::TRIGGER_RIGHT != 0))
            .field("trigger_left", &(self.0 & DS4Buttons::TRIGGER_LEFT != 0))
            .field(
                "shoulder_right",
                &(self.0 & DS4Buttons::SHOULDER_RIGHT != 0),
            )
            .field("shoulder_left", &(self.0 & DS4Buttons::SHOULDER_LEFT != 0))
            .field("triangle", &(self.0 & DS4Buttons::TRIANGLE != 0))
            .field("circle", &(self.0 & DS4Buttons::CIRCLE != 0))
            .field("cross", &(self.0 & DS4Buttons::CROSS != 0))
            .field("square", &(self.0 & DS4Buttons::SQUARE != 0))
            .field("dpad", &DpadDirection::from(self.0 & 0xF))
            .finish()
    }
}

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

impl From<u16> for DpadDirection {
    fn from(dpad: u16) -> Self {
        match dpad {
            DS4Buttons::DPAD_NORTH => DpadDirection::North,
            DS4Buttons::DPAD_NORTHEAST => DpadDirection::Northeast,
            DS4Buttons::DPAD_EAST => DpadDirection::East,
            DS4Buttons::DPAD_SOUTHEAST => DpadDirection::Southeast,
            DS4Buttons::DPAD_SOUTH => DpadDirection::South,
            DS4Buttons::DPAD_SOUTHWEST => DpadDirection::Southwest,
            DS4Buttons::DPAD_WEST => DpadDirection::West,
            DS4Buttons::DPAD_NORTHWEST => DpadDirection::Northwest,
            _ => DpadDirection::None,
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
#[derive(Copy, Clone, Eq, PartialEq)]
#[must_use = "This struct serves as a builder,
              and must be consumed by calling into() with the `DS4Report`/`DS4ReportEx` structs or directly with their respective builders"]
pub struct DS4SpecialButtons(pub(super) u8);

impl Debug for DS4SpecialButtons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DS4SpecialButtons")
            .field("mic_mute", &(self.0 & DS4SpecialButtons::MIC_MUTE != 0))
            .field("touchpad", &(self.0 & DS4SpecialButtons::TOUCHPAD != 0))
            .field("ps_home", &(self.0 & DS4SpecialButtons::PS_HOME != 0))
            .finish()
    }
}

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
