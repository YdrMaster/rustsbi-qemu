// § 4.2.2
pub(crate) mod modern;
// § 4.2.4
pub(crate) mod legacy;
// abstruction
pub(crate) mod common;

use crate::{DeviceType, U32Str};
use core::fmt;

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

#[derive(Debug)]
pub enum NeogotiateError {
    LeakFeature,
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
    type FeatureBits;

    unsafe fn from_raw_parts_unchecked(addr: usize) -> &'static Self;

    fn as_common(&self) -> &common::Interface {
        unsafe { &*(self as *const _ as *const common::Interface) }
    }

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
        self.as_common().reset()
    }

    fn launch(&self) -> bool {
        self.as_common().launch()
    }

    fn vendor_id(&self) -> U32Str {
        self.as_common().vendor_id.read()
    }

    fn negotiate(&self) -> Result<Self::FeatureBits, NeogotiateError>;
}
