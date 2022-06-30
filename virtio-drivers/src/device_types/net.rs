//! § 5.1 Network Device

use super::DeviceType;
use crate::mmio::{MmioInterface, MmioLegacyInterface, Version};

pub struct LegacyMmioVirtioNet(MmioLegacyInterface<Config>);

impl MmioInterface for LegacyMmioVirtioNet {
    const VERSION: Version = Version::Legacy;
    const TYPE: DeviceType = DeviceType::NetworkCard;

    #[inline]
    unsafe fn from_raw_parts_unchecked(addr: usize) -> &'static Self {
        &*(addr as *const Self)
    }
}

pub struct FeatureBits(u64);

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

impl FeatureBits {
    const VIRTIO_NET_F_CSUM: u64 = 0;
    const VIRTIO_NET_F_GUEST_CSUM: u64 = 1;
    const VIRTIO_NET_F_CTRL_GUEST_OFFLOADS: u64 = 2;
    const VIRTIO_NET_F_MTU: u64 = 3;
    const VIRTIO_NET_F_MAC: u64 = 5;
    const VIRTIO_NET_F_GUEST_TSO4: u64 = 7;
    const VIRTIO_NET_F_GUEST_TSO6: u64 = 8;
    const VIRTIO_NET_F_GUEST_ECN: u64 = 9;
    const VIRTIO_NET_F_GUEST_UFO: u64 = 10;
    const VIRTIO_NET_F_HOST_TSO4: u64 = 11;
    const VIRTIO_NET_F_HOST_TSO6: u64 = 12;
    const VIRTIO_NET_F_HOST_ECN: u64 = 13;
    const VIRTIO_NET_F_HOST_UFO: u64 = 14;
    const VIRTIO_NET_F_MRG_RXBUF: u64 = 15;
    const VIRTIO_NET_F_STATUS: u64 = 16;
    const VIRTIO_NET_F_CTRL_VQ: u64 = 17;
    const VIRTIO_NET_F_CTRL_RX: u64 = 18;
    const VIRTIO_NET_F_CTRL_VLAN: u64 = 19;
    const VIRTIO_NET_F_GUEST_ANNOUNCE: u64 = 21;
    const VIRTIO_NET_F_MQ: u64 = 22;
    const VIRTIO_NET_F_CTRL_MAC_ADDR: u64 = 23;
    const VIRTIO_NET_F_HOST_USO: u64 = 56;
    const VIRTIO_NET_F_HASH_REPORT: u64 = 57;
    const VIRTIO_NET_F_GUEST_HDRLEN: u64 = 59;
    const VIRTIO_NET_F_RSS: u64 = 60;
    const VIRTIO_NET_F_RSC_EXT: u64 = 61;
    const VIRTIO_NET_F_STANDBY: u64 = 62;
    const VIRTIO_NET_F_SPEED_DUPLEX: u64 = 63;
}
