//! DualShock 4 button components.

use std::{
    convert::TryFrom,
    fmt::Debug,
    ops::{BitOr, BitOrAssign},
};

/// DS4 button flags.
///
/// This struct allows to build a set of buttons to be pressed, using either the builder pattern or
/// the bitwise operators.
/// 
/// Buttons have associated constants, which can be used to set the state and the D-Pad direction can be set
/// using the [`DpadDirection`] enum.
///
/// # Examples
///
/// ```rust
/// # use vigem_client::{DS4Buttons, DpadDirection};
/// let buttons = DS4Buttons::new();
/// let buttons = buttons.thumb_right(true).cross(true).dpad(DpadDirection::South);
/// let buttons = buttons | DS4Buttons::SHOULDER_LEFT;
/// # assert_eq!(u16::from(buttons), DS4Buttons::THUMB_RIGHT | DS4Buttons::CROSS | DpadDirection::South as u16 | DS4Buttons::SHOULDER_LEFT);
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
            .field(
                "dpad",
                &DpadDirection::try_from(self.0 & 0xF).unwrap_or(DpadDirection::None),
            )
            .finish()
    }
}
impl Default for DS4Buttons {
    #[inline]
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
    const DPAD_NONE: u16 = 0x8; // 1 << 3
    /// D-Pad North-West direction.
    const DPAD_NORTHWEST: u16 = 0x7;
    /// D-Pad West direction.
    const DPAD_WEST: u16 = 0x6;
    /// D-Pad South-West direction.
    const DPAD_SOUTHWEST: u16 = 0x5;
    /// D-Pad South direction.
    const DPAD_SOUTH: u16 = 0x4;
    /// D-Pad South-East direction.
    const DPAD_SOUTHEAST: u16 = 0x3;
    /// D-Pad East direction.
    const DPAD_EAST: u16 = 0x2;
    /// D-Pad North-East direction.
    const DPAD_NORTHEAST: u16 = 0x1;
    /// D-Pad North direction.
    const DPAD_NORTH: u16 = 0x0;
}

impl From<DS4Buttons> for u16 {
    #[inline]
    fn from(buttons: DS4Buttons) -> Self {
        buttons.0
    }
}

impl BitOr<u16> for DS4Buttons {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: u16) -> Self::Output {
        DS4Buttons(self.0 | rhs)
    }
}

impl BitOrAssign<u16> for DS4Buttons {
    #[inline]
    fn bitor_assign(&mut self, rhs: u16) {
        self.0 |= rhs;
    }
}

impl DS4Buttons {
    /// Create a new [`DS4Buttons`] instance.
    #[inline]
    pub fn new() -> Self {
        DS4Buttons::default()
    }

    /// Set the thumb right button state.
    #[inline]
    pub fn thumb_right(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4Buttons::THUMB_RIGHT;
        }
        self
    }

    /// Set the thumb left button state.
    #[inline]
    pub fn thumb_left(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4Buttons::THUMB_LEFT;
        }
        self
    }

    /// Set the options button state.
    #[inline]
    pub fn options(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4Buttons::OPTIONS;
        }
        self
    }

    /// Set the share button state.
    #[inline]
    pub fn share(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4Buttons::SHARE;
        }
        self
    }

    /// Set the trigger left button state.
    #[inline]
    pub fn trigger_left(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4Buttons::TRIGGER_LEFT;
        }
        self
    }

    /// Set the trigger right button state.
    #[inline]
    pub fn trigger_right(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4Buttons::TRIGGER_RIGHT;
        }
        self
    }

    /// Set the shoulder right button state.
    #[inline]
    pub fn shoulder_right(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4Buttons::SHOULDER_RIGHT;
        }
        self
    }

    /// Set the shoulder left button state.
    #[inline]
    pub fn shoulder_left(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4Buttons::SHOULDER_LEFT;
        }
        self
    }

    /// Set the triangle button state.
    #[inline]
    pub fn triangle(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4Buttons::TRIANGLE;
        }
        self
    }

    /// Set the circle button state.
    #[inline]
    pub fn circle(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4Buttons::CIRCLE;
        }
        self
    }

    /// Set the cross button state.
    #[inline]
    pub fn cross(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4Buttons::CROSS;
        }
        self
    }

    /// Set the square button state.
    #[inline]
    pub fn square(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4Buttons::SQUARE;
        }
        self
    }

    /// Set the D-Pad direction, with the [`DpadDirection`] enum.
    #[inline]
    pub fn dpad(mut self, dpad: DpadDirection) -> Self {
        self.0 ^= DpadDirection::None as u16;
        self.0 |= dpad as u16;
        self
    }
}

impl BitOr<DpadDirection> for DS4Buttons {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: DpadDirection) -> Self::Output {
        self.dpad(rhs)
    }
}

impl BitOrAssign<DpadDirection> for DS4Buttons {
    #[inline]
    fn bitor_assign(&mut self, rhs: DpadDirection) {
        *self = self.dpad(rhs);
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
/// let buttons = DS4Buttons::new();
/// let buttons = buttons.dpad(DpadDirection::South);
/// # assert_eq!(u16::from(buttons), DpadDirection::South as u16);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u16)]
pub enum DpadDirection {
    /// D-Pad North direction.
    North = DS4Buttons::DPAD_NORTH,
    /// D-Pad North-East direction.
    NorthEast = DS4Buttons::DPAD_NORTHEAST,
    /// D-Pad East direction.
    East = DS4Buttons::DPAD_EAST,
    /// D-Pad South-East direction.
    SouthEast = DS4Buttons::DPAD_SOUTHEAST,
    /// D-Pad South direction.
    South = DS4Buttons::DPAD_SOUTH,
    /// D-Pad South-West direction.
    SouthWest = DS4Buttons::DPAD_SOUTHWEST,
    /// D-Pad West direction.
    West = DS4Buttons::DPAD_WEST,
    /// D-Pad North-West direction.
    NorthWest = DS4Buttons::DPAD_NORTHWEST,
    /// D-Pad neutral position.
    None = DS4Buttons::DPAD_NONE,
}

#[doc(hidden)]
impl TryFrom<u16> for DpadDirection {
    type Error = ();

    #[inline]
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            DS4Buttons::DPAD_NORTH => Ok(DpadDirection::North),
            DS4Buttons::DPAD_NORTHEAST => Ok(DpadDirection::NorthEast),
            DS4Buttons::DPAD_EAST => Ok(DpadDirection::East),
            DS4Buttons::DPAD_SOUTHEAST => Ok(DpadDirection::SouthEast),
            DS4Buttons::DPAD_SOUTH => Ok(DpadDirection::South),
            DS4Buttons::DPAD_SOUTHWEST => Ok(DpadDirection::SouthWest),
            DS4Buttons::DPAD_WEST => Ok(DpadDirection::West),
            DS4Buttons::DPAD_NORTHWEST => Ok(DpadDirection::NorthWest),
            DS4Buttons::DPAD_NONE => Ok(DpadDirection::None),
            _ => Err(()),
        }
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
/// let buttons = DS4SpecialButtons::new();
/// let buttons = buttons.mic_mute(true).ps_home(true);
/// let buttons = buttons | DS4SpecialButtons::TOUCHPAD;
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
    #[inline]
    fn default() -> Self {
        DS4SpecialButtons(0)
    }
}

impl From<DS4SpecialButtons> for u8 {
    #[inline]
    fn from(buttons: DS4SpecialButtons) -> Self {
        buttons.0
    }
}

impl DS4SpecialButtons {
    #[inline]
    pub fn new() -> Self {
        DS4SpecialButtons::default()
    }

    /// Set the mic mute button state.
    #[inline]
    pub fn mic_mute(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4SpecialButtons::MIC_MUTE;
        }
        self
    }

    /// Set the touchpad button state.
    #[inline]
    pub fn touchpad(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4SpecialButtons::TOUCHPAD;
        }
        self
    }

    /// Set the PS Home button state.
    #[inline]
    pub fn ps_home(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= DS4SpecialButtons::PS_HOME;
        }
        self
    }
}

impl BitOr<u8> for DS4SpecialButtons {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: u8) -> Self::Output {
        DS4SpecialButtons(self.0 | rhs)
    }
}

impl BitOrAssign<u8> for DS4SpecialButtons {
    #[inline]
    fn bitor_assign(&mut self, rhs: u8) {
        self.0 |= rhs;
    }
}
