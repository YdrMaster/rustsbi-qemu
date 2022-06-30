#![no_std]

#[macro_use]
extern crate num_enum;

// § 2.1
mod device_status_field;
// § 2.2
mod feature_bits;
// § 2.7
mod split_virtqueues;
// § 4.2
mod mmio;
// § 5
mod device_types;

pub use device_status_field::DeviceStatus;
pub use device_types::{DeviceType, LegacyMmioVirtioNet};
pub use mmio::{common::Interface as MmioVirtioCommon, MmioInterface};
pub use split_virtqueues::{
    DescriptorTable, VirtqAvail, VirtqAvailFlags, VirtqDesc, VirtqDescFlags, VirtqUsed,
    VirtqUsedElem, VirtqUsedFlags,
};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct U32Str(u32);

impl U32Str {
    const fn new(s: &str) -> Self {
        if let [a, b, c, d] = *s.as_bytes() {
            Self(u32::from_ne_bytes([a, b, c, d]))
        } else {
            panic!("failed to convert str to `U32Str`, whose length is not 4");
        }
    }
}

impl core::fmt::Debug for U32Str {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if let Ok(s) = core::str::from_utf8(&self.0.to_ne_bytes()) {
            s.fmt(f)
        } else {
            self.0.fmt(f)
        }
    }
}

impl core::fmt::Display for U32Str {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if let Ok(s) = core::str::from_utf8(&self.0.to_ne_bytes()) {
            write!(f, "{s}")
        } else {
            write!(f, "{:#x}", self.0)
        }
    }
}
