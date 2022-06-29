﻿//! § 2.7.5 The Virtqueue Descriptor Table

use super::VirtqIdx;
use core::ops::Index;
use volatile_register::RW;

#[repr(C, align(16))]
pub struct DescriptorTable<const QSZ: usize>([VirtqDesc; QSZ]);

impl<const QSZ: usize> DescriptorTable<QSZ> {
    pub fn fill_in_order(&self) {
        for (i, desc) in self.0.iter().enumerate() {
            unsafe { desc.next.write(VirtqIdx::new(i + 1)) };
        }
    }
}

impl<const QSZ: usize> Index<VirtqIdx> for DescriptorTable<QSZ> {
    type Output = VirtqDesc;

    #[inline]
    fn index(&self, index: VirtqIdx) -> &Self::Output {
        &self.0[u16::from_le(index.0) as usize]
    }
}

#[repr(C)]
pub struct VirtqDesc {
    addr: RW<u64>,
    len: RW<u32>,
    flags: RW<Flags>,
    next: RW<VirtqIdx>,
}

impl VirtqDesc {
    #[inline]
    pub fn addr(&self) -> usize {
        u64::from_le(self.addr.read()) as _
    }

    #[inline]
    pub fn len(&self) -> usize {
        u32::from_le(self.len.read()) as _
    }

    #[inline]
    pub fn flags(&self) -> Flags {
        self.flags.read()
    }

    #[inline]
    pub fn next(&self) -> Option<VirtqIdx> {
        if self.flags.read().next() {
            Some(self.next.read())
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Flags(u16);

impl Flags {
    const NEXT: u16 = u16::from_le(1);
    const WRITE: u16 = u16::from_le(2);
    const INDIRECT: u16 = u16::from_le(4);

    pub const fn new(next: bool, write: bool, indirect: bool) -> Self {
        let mut flags = 0;
        if next {
            flags |= Self::NEXT;
        }
        if write {
            flags |= Self::WRITE;
        }
        if indirect {
            flags |= Self::INDIRECT;
        }
        Self(flags)
    }

    #[inline]
    pub fn next(&self) -> bool {
        self.0 & Self::NEXT != 0
    }

    #[inline]
    pub fn write(&self) -> bool {
        self.0 & Self::WRITE != 0
    }

    #[inline]
    pub fn indirect(&self) -> bool {
        self.0 & Self::INDIRECT != 0
    }
}
