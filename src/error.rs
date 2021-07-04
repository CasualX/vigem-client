use std::{error, fmt};

/// ViGEm client errors.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
	/// There was an unexpected windows error.
	///
	/// See [System Error Codes](https://docs.microsoft.com/en-us/windows/win32/debug/system-error-codes) for more information.
	WinError(u32),
	/// The ViGEmBus Driver is not installed.
	///
	/// It can be installed from the [ViGEmBus](https://github.com/ViGEm/ViGEmBus) repository.
	BusNotFound,
	/// ViGEmBus was found, but accessing it returned an error.
	BusAccessFailed(u32),
	/// ViGEmBus was found, but it did not accept this client's version.
	BusVersionMismatch,
	/// There was no more room to allocate new targets.
	NoFreeSlot,
	// InvalidClient,
	// InvalidTarget,
	/// The target is already connected.
	///
	/// It is an error to try to plugin an already connected target.
	AlreadyConnected,
	/// The target is not plugged in.
	NotPluggedIn,
	/// The target is not ready.
	///
	/// After creating the desired controller, wait some time before the target is ready to accept updates.
	/// This error is returned if a target is updated before it is ready.
	TargetNotReady,
	UserIndexOutOfRange,
}

impl From<u32> for Error {
	#[inline]
	fn from(error: u32) -> Error {
		Error::WinError(error)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Error::WinError(err) => write!(f, "win error: {}", err),
			Error::BusNotFound => f.write_str("bus not found"),
			Error::BusAccessFailed(err) => write!(f, "bus access failed: {}", err),
			Error::BusVersionMismatch => f.write_str("bus version mismatch"),
			Error::NoFreeSlot => f.write_str("no free slot"),
			Error::AlreadyConnected => f.write_str("already connected"),
			Error::NotPluggedIn => f.write_str("not plugged in"),
			Error::TargetNotReady => f.write_str("target not ready"),
			Error::UserIndexOutOfRange => f.write_str("user index out of range"),
		}
	}
}

impl error::Error for Error {}
