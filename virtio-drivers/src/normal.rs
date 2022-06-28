// § 4.2.2 MMIO Device Register Layout

use crate::{
    drivers::DeviceType,
    header::{Magic, Version},
    status::{test_and_push, DeviceStatus, DeviceStatusField},
    U32Str,
};
use volatile_register::{RO, RW, WO};

#[repr(C)]
pub struct Interface {
    pub magic_value: RO<Magic>,
    pub version: RO<Version>,
    pub device_id: RO<DeviceType>,
    pub vendor_id: RO<U32Str>,
    pub device_features: RO<u32>,
    pub device_features_sel: WO<u32>,
    _align0: [u32; 2],
    pub driver_features: WO<u32>,
    pub driver_features_sel: WO<u32>,
    _align1: [u32; 2],
    pub queue_sel: WO<u32>,
    pub queue_num_max: RO<u32>,
    pub queue_num: WO<u32>,
    _align2: [u32; 2],
    pub queue_ready: RW<u32>,
    _align3: [u32; 2],
    pub queue_notify: WO<u32>,
    _align4: [u32; 3],
    pub interrupt_status: RO<u32>,
    pub interrupt_ack: WO<u32>,
    _align5: [u32; 2],
    status: RW<DeviceStatusField>,
    _align6: [u32; 3],
    pub queue_desc_low: WO<u32>,
    pub queue_desc_high: WO<u32>,
    _align7: [u32; 2],
    pub queue_driver_low: WO<u32>,
    pub queue_driver_high: WO<u32>,
    _align8: [u32; 2],
    pub queue_device_low: WO<u32>,
    pub queue_device_high: WO<u32>,
    _align9: [u32; 1],
    pub shm_sel: WO<u32>,
    pub shm_len_low: RO<u32>,
    pub shm_len_high: RO<u32>,
    pub shm_base_low: RO<u32>,
    pub shm_base_high: RO<u32>,
    pub queue_reset: RW<u32>,
    _align10: [u32; 14],
    pub config_generation: RO<u32>,
}

impl Interface {
    pub fn acknowledge(addr: usize) -> &'static Self {
        let ans = unsafe { &*(addr as *const Self) };
        test_and_push(&ans.status, DeviceStatus::Uninitialized).unwrap();
        test_and_push(&ans.status, DeviceStatus::Acknowledged).unwrap();
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
    assert_eq!(base + 0x10, &test.device_features as *const _ as usize);
    assert_eq!(base + 0x14, &test.device_features_sel as *const _ as usize);
    assert_eq!(base + 0x20, &test.driver_features as *const _ as usize);
    assert_eq!(base + 0x24, &test.driver_features_sel as *const _ as usize);
    assert_eq!(base + 0x30, &test.queue_sel as *const _ as usize);
    assert_eq!(base + 0x34, &test.queue_num_max as *const _ as usize);
    assert_eq!(base + 0x38, &test.queue_num as *const _ as usize);
    assert_eq!(base + 0x44, &test.queue_ready as *const _ as usize);
    assert_eq!(base + 0x50, &test.queue_notify as *const _ as usize);
    assert_eq!(base + 0x60, &test.interrupt_status as *const _ as usize);
    assert_eq!(base + 0x64, &test.interrupt_ack as *const _ as usize);
    assert_eq!(base + 0x70, &test.status as *const _ as usize);
    assert_eq!(base + 0x80, &test.queue_desc_low as *const _ as usize);
    assert_eq!(base + 0x84, &test.queue_desc_high as *const _ as usize);
    assert_eq!(base + 0x80, &test.queue_desc_low as *const _ as usize);
    assert_eq!(base + 0x84, &test.queue_desc_high as *const _ as usize);
    assert_eq!(base + 0x90, &test.queue_driver_low as *const _ as usize);
    assert_eq!(base + 0x94, &test.queue_driver_high as *const _ as usize);
    assert_eq!(base + 0xa0, &test.queue_device_low as *const _ as usize);
    assert_eq!(base + 0xa4, &test.queue_device_high as *const _ as usize);
    assert_eq!(base + 0xac, &test.shm_sel as *const _ as usize);
    assert_eq!(base + 0xb0, &test.shm_len_low as *const _ as usize);
    assert_eq!(base + 0xb4, &test.shm_len_high as *const _ as usize);
    assert_eq!(base + 0xb8, &test.shm_base_low as *const _ as usize);
    assert_eq!(base + 0xbc, &test.shm_base_high as *const _ as usize);
    assert_eq!(base + 0xc0, &test.queue_reset as *const _ as usize);
    assert_eq!(base + 0xfc, &test.config_generation as *const _ as usize);
}
