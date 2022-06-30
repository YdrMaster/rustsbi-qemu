//! § 2.7.8 The Virtqueue Used Ring

use super::VirtqIdx;
use core::ops::Index;
use volatile_register::RO;

#[repr(C, align(4))]
pub struct VirtqUsed<const QSZ: usize> {
    flags: RO<Flags>,
    idx: RO<VirtqIdx>,
    ring: [VirtqUsedElem; QSZ],
    avail_event: RO<VirtqIdx>,
}

impl<const QSZ: usize> VirtqUsed<QSZ> {
    #[inline]
    pub fn idx(&self) -> VirtqIdx {
        self.idx.read()
    }

    #[inline]
    pub fn avail_event(&self) -> Option<VirtqIdx> {
        if self.flags.read().no_notify() {
            Some(self.avail_event.read())
        } else {
            None
        }
    }
}

impl<const QSZ: usize> Index<VirtqIdx> for VirtqUsed<QSZ> {
    type Output = VirtqUsedElem;

    #[inline]
    fn index(&self, index: VirtqIdx) -> &Self::Output {
        &self.ring[u16::from_le(index.0) as usize]
    }
}

#[repr(C)]
pub struct VirtqUsedElem {
    id: RO<u32>,
    len: RO<u32>,
}

impl VirtqUsedElem {
    #[inline]
    pub fn id(&self) -> VirtqIdx {
        VirtqIdx::new(u32::from_le(self.id.read()) as _)
    }

    #[inline]
    pub fn len(&self) -> usize {
        u32::from_le(self.len.read()) as _
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Flags(u16);

impl Flags {
    const NO_NOTIFY: u16 = u16::from_le(1);

    #[inline]
    pub const fn new(no_notify: bool) -> Self {
        Self(if no_notify { Self::NO_NOTIFY } else { 0 })
    }

    #[inline]
    pub fn no_notify(&self) -> bool {
        self.0 & Self::NO_NOTIFY != 0
    }
}
