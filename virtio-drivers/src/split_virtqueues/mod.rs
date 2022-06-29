﻿// § 2.7 Split Virtqueues

// § 2.7.5
mod descriptor_table;
// § 2.7.6
mod available_ring;

pub use available_ring::{Flags as VirtqAvailFlags, VirtqAvail};
pub use descriptor_table::{DescriptorTable, Flags as VirtqDescFlags, VirtqDesc};

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct VirtqIdx(u16);

impl VirtqIdx {
    #[inline]
    const fn new(idx: usize) -> Self {
        Self((idx as u16).to_le())
    }
}
