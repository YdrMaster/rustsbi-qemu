use crate::{drivers::DeviceType, U32Str};
use core::fmt;
use volatile_register::RO;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Header {
    pub magic_value: Magic,
    pub version: Version,
    pub device_id: DeviceType,
    pub vendor_id: U32Str,
}

#[derive(Debug)]
pub enum Error {
    InvalidMagic,
    InvalidVersion,
    UnknownType(u32),
}

impl Header {
    pub fn probe(addr: usize) -> Result<Header, Error> {
        use Error::*;
        /// 根据文档，前三个寄存器必须按顺序访问以在不符合要求时尽量减少误操作。
        #[repr(C)]
        struct Unchecked {
            magic_value: RO<u32>,
            version: RO<u32>,
            device_id: RO<u32>,
            vendor_id: RO<u32>,
        }

        let unchecked = unsafe { &*(addr as *const Unchecked) };
        Ok(Header {
            magic_value: Some(Magic(U32Str(unchecked.magic_value.read())))
                .filter(|m| m == &MAGIC)
                .ok_or(InvalidMagic)?,
            version: Version::try_from(unchecked.version.read()).map_err(|_| InvalidVersion)?,
            device_id: DeviceType::try_from(unchecked.device_id.read())
                .map_err(|e| UnknownType(e.number))?,
            vendor_id: U32Str(unchecked.vendor_id.read()),
        })
    }
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
