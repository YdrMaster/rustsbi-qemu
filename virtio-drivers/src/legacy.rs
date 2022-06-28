﻿// § 4.2.4 Legacy Interface

use crate::{
    drivers::DeviceType,
    header::{Magic, Version},
    status::{self, DeviceStatus, DeviceStatusField},
    U32Str,
};
use volatile_register::{RO, RW, WO};

#[repr(C)]
pub struct Interface {
    pub magic_value: RO<Magic>,
    pub version: RO<Version>,
    pub device_id: RO<DeviceType>,
    pub vendor_id: RO<U32Str>,
    pub host_features: RO<u32>,
    pub host_features_sel: WO<u32>,
    _align0: [u32; 2],
    pub guest_features: WO<u32>,
    pub guest_features_sel: WO<u32>,
    pub guest_page_size: WO<u32>,
    _align1: [u32; 1],
    pub queue_sel: WO<u32>,
    pub queue_num_max: RO<u32>,
    pub queue_num: WO<u32>,
    pub queue_align: WO<u32>,
    pub queue_pfn: RW<u32>,
    _align2: [u32; 3],
    pub queue_notify: WO<u32>,
    _align3: [u32; 3],
    pub interrupt_status: RO<u32>,
    pub interrupt_ack: WO<u32>,
    _align4: [u32; 2],
    status: RW<DeviceStatusField>,
}

impl Interface {
    pub fn acknowledge(addr: usize) -> &'static Self {
        let ans = unsafe { &*(addr as *const Self) };
        status::test_and_push(&ans.status, DeviceStatus::Uninitialized).unwrap();
        status::test_and_push(&ans.status, DeviceStatus::Acknowledged).unwrap();
        ans
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
    assert_eq!(base + 0x10, &test.host_features as *const _ as usize);
    assert_eq!(base + 0x14, &test.host_features_sel as *const _ as usize);
    assert_eq!(base + 0x20, &test.guest_features as *const _ as usize);
    assert_eq!(base + 0x24, &test.guest_features_sel as *const _ as usize);
    assert_eq!(base + 0x28, &test.guest_page_size as *const _ as usize);
    assert_eq!(base + 0x30, &test.queue_sel as *const _ as usize);
    assert_eq!(base + 0x34, &test.queue_num_max as *const _ as usize);
    assert_eq!(base + 0x38, &test.queue_num as *const _ as usize);
    assert_eq!(base + 0x3c, &test.queue_align as *const _ as usize);
    assert_eq!(base + 0x40, &test.queue_pfn as *const _ as usize);
    assert_eq!(base + 0x50, &test.queue_notify as *const _ as usize);
    assert_eq!(base + 0x60, &test.interrupt_status as *const _ as usize);
    assert_eq!(base + 0x64, &test.interrupt_ack as *const _ as usize);
    assert_eq!(base + 0x70, &test.status as *const _ as usize);
}
