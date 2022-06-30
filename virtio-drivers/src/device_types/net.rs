//! § 5.1 Network Device

use super::DeviceType;
use crate::mmio::MmioVirtioMeta;

pub struct NetMeta;

impl MmioVirtioMeta for NetMeta {
    const TYPE: DeviceType = DeviceType::NetworkCard;

    type FeatureBits = crate::feature_bits::FeatureBits<1>;
    const MINIUM_FEATURE_SET: Self::FeatureBits = Self::FeatureBits::new([feature_bits::MAC]);
    const SUPPORTED_FEATURE_SET: Self::FeatureBits =
        Self::FeatureBits::new([feature_bits::MAC | feature_bits::STATUS]);

    type Config = Config;
}

pub struct Config {
    mac: [u8; 6],
    status: u16,
    max_virtqueue_pairs: u16,
    mtu: u16,
    speed: u32,
    duplex: u8,
    rss_max_key_size: u8,
    rss_max_indirection_table_length: u16,
    supported_hash_types: u32,
}

mod feature_bits {
    #![allow(unused)]

    use crate::feature_bits::FeatureBits;

    pub(super) const MINIUM_FEATURE_SET: FeatureBits<1> = FeatureBits::new([MAC]);
    pub(super) const SUPPORTED_FEATURE_SET: FeatureBits<1> = FeatureBits::new([MAC | STATUS]);

    pub(super) const CSUM: u32 = 1 << 0;
    pub(super) const GUEST_CSUM: u32 = 1 << 1;
    pub(super) const CTRL_GUEST_OFFLOADS: u32 = 1 << 2;
    pub(super) const MTU: u32 = 1 << 3;
    pub(super) const MAC: u32 = 1 << 5;
    pub(super) const GUEST_TSO4: u32 = 1 << 7;
    pub(super) const GUEST_TSO6: u32 = 1 << 8;
    pub(super) const GUEST_ECN: u32 = 1 << 9;
    pub(super) const GUEST_UFO: u32 = 1 << 10;
    pub(super) const HOST_TSO4: u32 = 1 << 11;
    pub(super) const HOST_TSO6: u32 = 1 << 12;
    pub(super) const HOST_ECN: u32 = 1 << 13;
    pub(super) const HOST_UFO: u32 = 1 << 14;
    pub(super) const MRG_RXBUF: u32 = 1 << 15;
    pub(super) const STATUS: u32 = 1 << 16;
    pub(super) const CTRL_VQ: u32 = 1 << 17;
    pub(super) const CTRL_RX: u32 = 1 << 18;
    pub(super) const CTRL_VLAN: u32 = 1 << 19;
    pub(super) const GUEST_ANNOUNCE: u32 = 1 << 21;
    pub(super) const MQ: u32 = 1 << 22;
    pub(super) const CTRL_MAC_ADDR: u32 = 1 << 23;
    pub(super) const HOST_USO: u64 = 1 << 56;
    pub(super) const HASH_REPORT: u64 = 1 << 57;
    pub(super) const GUEST_HDRLEN: u64 = 1 << 59;
    pub(super) const RSS: u64 = 1 << 60;
    pub(super) const RSC_EXT: u64 = 1 << 61;
    pub(super) const STANDBY: u64 = 1 << 62;
    pub(super) const SPEED_DUPLEX: u64 = 1 << 63;
}
