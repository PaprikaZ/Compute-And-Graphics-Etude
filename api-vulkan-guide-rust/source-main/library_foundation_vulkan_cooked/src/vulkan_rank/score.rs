use std::ops::Add;
use std::ops::Mul;


#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct VulkanRankScore(u32);

impl VulkanRankScore {
    pub const ZERO: Self = Self(0);

    pub(super) fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn as_raw(self) -> u32 {
        self.0
    }
}

impl Add for VulkanRankScore {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Mul<u32> for VulkanRankScore {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul<VulkanRankScore> for u32 {
    type Output = VulkanRankScore;

    fn mul(self, rhs: VulkanRankScore) -> Self::Output {
        VulkanRankScore(rhs.0 * self)
    }
}