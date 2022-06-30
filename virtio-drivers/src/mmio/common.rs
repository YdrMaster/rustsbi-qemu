﻿use super::{Magic, MmioInterface, ProbeError, Version, MAGIC};
use crate::{
    device_status_field::{test_and_push, DeviceStatusField},
    device_types::DeviceType,
    DeviceStatus, U32Str,
};
use volatile_register::{RO, RW, WO};

#[repr(C)]
pub struct Interface {
    magic_value: RO<Magic>,
    pub version: RO<Version>,
    pub device_id: RO<DeviceType>,
    pub vendor_id: RO<U32Str>,
    device_features: RO<u32>,
    device_features_sel: WO<u32>,
    _align0: [u32; 2],
    driver_features: WO<u32>,
    driver_features_sel: WO<u32>,
    _align1: [u32; 2],
    queue_sel: WO<u32>,
    queue_num_max: RO<u32>,
    queue_num: WO<u32>,
    _align2: [u32; 5],
    queue_notify: WO<u32>,
    _align4: [u32; 3],
    interrupt_status: RO<u32>,
    interrupt_ack: WO<u32>,
    _align5: [u32; 2],
    status: RW<DeviceStatusField>,
    _align6: [u32; 35],
    config: u32,
}

impl Interface {
    #[inline]
    pub(super) fn from_ref(other: &(impl MmioInterface + ?Sized)) -> &Self {
        unsafe { &*(other as *const _ as *const Self) }
    }
}

impl MmioInterface for Interface {
    const VERSION: Version = unreachable!(); // never used
    const TYPE: DeviceType = unreachable!(); // never used

    #[inline]
    unsafe fn from_raw_parts_unchecked(addr: usize) -> &'static Self {
        &*(addr as *const Self)
    }

    fn probe(addr: usize) -> Result<&'static Self, ProbeError> {
        use ProbeError::*;

        /// 根据文档，前三个寄存器必须按顺序访问以在不符合要求时尽量减少误操作。
        #[repr(C)]
        struct Unchecked {
            magic_value: RO<Magic>,
            version: RO<u32>,
            device_id: RO<u32>,
            vendor_id: RO<u32>,
        }

        let unchecked = unsafe { &*(addr as *const Unchecked) };
        if unchecked.magic_value.read() != MAGIC {
            return Err(InvalidMagic);
        }
        if Version::try_from(unchecked.version.read()).is_err() {
            return Err(InvalidVersion);
        }
        match DeviceType::try_from(unchecked.device_id.read()) {
            Ok(DeviceType::Reserved) => Err(NoDevice)?,
            Err(e) => Err(UnknownType(e.number))?,
            Ok(_) => {}
        }
        let ans = unsafe { &*(addr as *const Self) };
        ans.reset();
        Ok(ans)
    }

    fn reset(&self) {
        loop {
            match test_and_push(&self.status, DeviceStatus::Uninitialized) {
                Ok(_) | Err(Ok(DeviceStatus::Acknowledged)) => break,
                Err(_) => unsafe { self.status.write(DeviceStatusField::MMIO_RESET) },
            }
        }
    }

    fn launch(&self) -> bool {
        test_and_push(&self.status, DeviceStatus::Acknowledged).is_ok()
    }

    fn vendor_id(&self) -> U32Str {
        self.vendor_id.read()
    }
}

#[test]
fn test() {
    let memory = [0u8; core::mem::size_of::<Interface>()];
    let base = memory.as_ptr() as usize;
    let test = unsafe { &*(memory.as_ptr().cast::<Interface>()) };
    assert_eq!(base + 0x00, &test.magic_value as *const _ as usize);
    assert_eq!(base + 0x04, &test.version as *const _ as usize);
    assert_eq!(base + 0x08, &test.device_id as *const _ as usize);
    assert_eq!(base + 0x0c, &test.vendor_id as *const _ as usize);
    assert_eq!(base + 0x10, &test.device_features as *const _ as usize);
    assert_eq!(base + 0x14, &test.device_features_sel as *const _ as usize);
    assert_eq!(base + 0x20, &test.driver_features as *const _ as usize);
    assert_eq!(base + 0x24, &test.driver_features_sel as *const _ as usize);
    assert_eq!(base + 0x30, &test.queue_sel as *const _ as usize);
    assert_eq!(base + 0x34, &test.queue_num_max as *const _ as usize);
    assert_eq!(base + 0x38, &test.queue_num as *const _ as usize);
    assert_eq!(base + 0x50, &test.queue_notify as *const _ as usize);
    assert_eq!(base + 0x60, &test.interrupt_status as *const _ as usize);
    assert_eq!(base + 0x64, &test.interrupt_ack as *const _ as usize);
    assert_eq!(base + 0x70, &test.status as *const _ as usize);
    assert_eq!(base + 0x100, &test.config as *const _ as usize);
}
