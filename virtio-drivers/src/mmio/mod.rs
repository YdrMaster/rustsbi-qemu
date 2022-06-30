// todo
mod legacy;
// todo
mod common;
mod modern;

use crate::{DeviceType, U32Str};
use core::fmt;

pub use legacy::Interface as MmioLegacyInterface;
pub use modern::Interface as MmioModernInterface;

#[derive(Debug)]
pub enum ProbeError {
    InvalidMagic,
    InvalidVersion,
    NoDevice,
    UnknownType(u32),
    VersionMismatch(Version),
    TypeMismatch(DeviceType),
    SetStatusFailed,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Magic(U32Str);

const MAGIC: Magic = Magic(U32Str::new("virt"));

impl fmt::Debug for Magic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum Version {
    Legacy = 1,
    Normal = 2,
}

pub trait MmioInterface {
    const VERSION: Version;
    const TYPE: DeviceType;

    unsafe fn from_raw_parts_unchecked(addr: usize) -> &'static Self;

    fn probe(addr: usize) -> Result<&'static Self, ProbeError> {
        let common = common::Interface::probe(addr)?;
        let version = common.version.read();
        if version != Self::VERSION {
            Err(ProbeError::VersionMismatch(version))?;
        }
        let device_type = common.device_id.read();
        if device_type != Self::TYPE {
            Err(ProbeError::TypeMismatch(device_type))?;
        }
        if !common.launch() {
            Err(ProbeError::SetStatusFailed)?;
        }
        Ok(unsafe { Self::from_raw_parts_unchecked(addr) })
    }

    fn reset(&self) {
        common::Interface::from_ref(self).reset();
    }

    fn launch(&self) -> bool {
        common::Interface::from_ref(self).launch()
    }

    fn vendor_id(&self) -> U32Str {
        common::Interface::from_ref(self).vendor_id.read()
    }
}
