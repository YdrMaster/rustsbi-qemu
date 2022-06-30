//! § 2.2 Feature Bits

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct FeatureBits<const LEN: usize>(pub [u32; LEN]);

impl<const LEN: usize> FeatureBits<LEN> {
    /// 判断是否包含指定的特征位。
    pub fn contains(&self, rhs: Self) -> bool {
        rhs.0
            .iter()
            .enumerate()
            .all(|(i, &rhs)| self.0[i] & rhs == rhs)
    }

    pub fn read_from_device(f: impl Fn(u32) -> u32) -> Self {
        let mut ans = Self([0u32; LEN]);
        for (i, bits) in ans.0.iter_mut().enumerate() {
            *bits = f(i as _);
        }
        ans
    }

    pub fn write_to_device(&self, f: impl Fn(u32, u32)) {
        for (i, &bits) in self.0.iter().enumerate() {
            f(i as _, bits);
        }
    }
}

impl<const LEN: usize> core::ops::BitAnd for FeatureBits<LEN> {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self {
        self.0
            .iter_mut()
            .enumerate()
            .for_each(|(i, lhs)| *lhs &= rhs.0[i]);
        self
    }
}
