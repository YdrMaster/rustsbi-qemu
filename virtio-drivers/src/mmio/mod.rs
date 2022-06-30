// § 4.2.2
pub(crate) mod modern;
// § 4.2.4
pub(crate) mod legacy;
// abstruction
pub(crate) mod common;

use crate::{DeviceType, U32Str};
use core::{fmt, marker::PhantomData};

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

pub struct MmioVirtio<T> {
    addr: usize,
    legacy: bool,
    _phantom: PhantomData<T>,
}

pub trait MmioVirtioMeta {
    const TYPE: DeviceType;

    type FeatureBits;
    const MINIUM_FEATURE_SET: Self::FeatureBits;
    const SUPPORTED_FEATURE_SET: Self::FeatureBits;

    type Config;
}

impl<const LEN: usize, Meta> MmioVirtio<Meta>
where
    Meta: MmioVirtioMeta<FeatureBits = crate::feature_bits::FeatureBits<LEN>>,
{
    pub fn new(addr: usize) -> Result<Self, ProbeError> {
        let common = common::Interface::probe(addr)?;
        let legacy = matches!(common.version.read(), Version::Legacy);
        let device_type = common.device_id.read();
        if device_type != Meta::TYPE {
            Err(ProbeError::TypeMismatch(device_type))?;
        }
        if !common.launch() {
            Err(ProbeError::SetStatusFailed)?;
        }
        common
            .negotiate(Meta::MINIUM_FEATURE_SET, Meta::SUPPORTED_FEATURE_SET)
            .unwrap();
        if legacy {
            unsafe { &*(addr as *const legacy::Interface<Meta::Config>) }.set_page_size(4096);
        }
        Ok(Self {
            addr,
            legacy,
            _phantom: PhantomData,
        })
    }

    fn common(&self) -> &common::Interface {
        unsafe { &*(self.addr as *const common::Interface) }
    }

    pub fn vendor_id(&self) -> U32Str {
        self.common().vendor_id.read()
    }
}
