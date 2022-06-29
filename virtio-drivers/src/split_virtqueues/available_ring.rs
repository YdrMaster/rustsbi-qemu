//! § 2.7.6 The Virtqueue Available Ring

use super::VirtqIdx;
use core::ops::Index;
use volatile_register::RW;

#[repr(C, align(2))]
pub struct VirtqAvail<const QSZ: usize> {
    flags: RW<Flags>,
    idx: RW<VirtqIdx>,
    ring: [RW<VirtqIdx>; QSZ],
    used_event: RW<VirtqIdx>,
}

impl<const QSZ: usize> VirtqAvail<QSZ> {
    #[inline]
    pub fn used_event(&self) -> Option<VirtqIdx> {
        if self.flags.read().no_interrupt() {
            Some(self.used_event.read())
        } else {
            None
        }
    }
}

impl<const QSZ: usize> Index<VirtqIdx> for VirtqAvail<QSZ> {
    type Output = RW<VirtqIdx>;

    #[inline]
    fn index(&self, index: VirtqIdx) -> &Self::Output {
        &self.ring[u16::from_le(index.0) as usize]
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Flags(u16);

impl Flags {
    const NO_INTERRUPT: u16 = u16::from_le(1);

    #[inline]
    pub const fn new(no_interrupt: bool) -> Self {
        Self(if no_interrupt { Self::NO_INTERRUPT } else { 0 })
    }

    #[inline]
    pub fn no_interrupt(&self) -> bool {
        self.0 & Self::NO_INTERRUPT != 0
    }
}
