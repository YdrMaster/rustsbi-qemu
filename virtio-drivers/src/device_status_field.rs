// § 2.1 Device Status Field

use volatile_register::RW;

/// 设备状态字段
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(transparent)]
pub struct DeviceStatusField(u32);

impl DeviceStatusField {
    pub const MMIO_RESET: Self = Self(0);
}

/// 设备状态
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum DeviceStatus {
    /// 未初始化
    Uninitialized,
    /// 已发现
    Acknowledged,
    /// 驱动启动完毕
    DriverLaunched,
    /// 功能协商完毕
    FeaturesOK,
    /// 驱动初始化完毕
    DriverOK,
    /// 驱动错误
    Failed,
    /// 设备错误
    DeviceNeedsReset,
}

/// 如果设备状态符合预期，将设备设置到下一个状态。
pub fn test_and_push(
    status: &RW<DeviceStatusField>,
    expected: DeviceStatus,
) -> Result<DeviceStatus, Result<DeviceStatus, DeviceStatusField>> {
    use DeviceStatus::*;
    let next = match DeviceStatus::try_from(status.read()) {
        // 当前状态与预期状态一致
        // 若是正常状态，向下一个状态转移
        // 否则返回当前异常状态
        Ok(current) if current == expected => match current {
            Uninitialized => Acknowledged,
            Acknowledged => DriverLaunched,
            DriverLaunched => FeaturesOK,
            FeaturesOK => DriverOK,
            DriverOK | Failed | DeviceNeedsReset => return Ok(current),
        },
        // 当前状态与预期状态不一致
        current => return Err(current),
    };
    unsafe { status.write(DeviceStatusField(next.into_bits())) };
    Ok(next)
}

impl TryFrom<DeviceStatusField> for DeviceStatus {
    type Error = DeviceStatusField;

    /// 状态字段的每个位都只能设置，不能清除，因此，正确的值是按流程累积的。
    /// 例如，如果设置了 `DRIVER`，一定已经设置了 `ACKNOWLEDGE`。
    /// 不满足这种累积性质的值是非法值。
    fn try_from(value: DeviceStatusField) -> Result<Self, Self::Error> {
        if value.0 & bits::DEVICE_NEEDS_RESET != 0 {
            Ok(Self::DeviceNeedsReset)
        } else if value.0 & bits::FAILED != 0 {
            Ok(Self::Failed)
        } else {
            match value.0 {
                steps::UNINITIALIZED => Ok(Self::Uninitialized),
                steps::ACKNOWLEDGED => Ok(Self::Acknowledged),
                steps::DRIVER_LAUNCHED => Ok(Self::DriverLaunched),
                steps::FEATURES_OK => Ok(Self::FeaturesOK),
                steps::DRIVER_OK => Ok(Self::DriverOK),
                _ => Err(value),
            }
        }
    }
}

impl DeviceStatus {
    fn into_bits(self) -> u32 {
        match self {
            Self::Uninitialized => steps::UNINITIALIZED,
            Self::Acknowledged => steps::ACKNOWLEDGED,
            Self::DriverLaunched => steps::DRIVER_LAUNCHED,
            Self::FeaturesOK => steps::FEATURES_OK,
            Self::DriverOK => steps::DRIVER_OK,
            Self::Failed | Self::DeviceNeedsReset => unreachable!(),
        }
    }
}

mod bits {
    pub(super) const ACKNOWLEDGE: u32 = 1;
    pub(super) const DRIVER: u32 = 2;
    pub(super) const FAILED: u32 = 128;
    pub(super) const FEATURES_OK: u32 = 8;
    pub(super) const DRIVER_OK: u32 = 4;
    pub(super) const DEVICE_NEEDS_RESET: u32 = 64;
}

mod steps {
    use super::bits;
    pub(super) const UNINITIALIZED: u32 = 0;
    pub(super) const ACKNOWLEDGED: u32 = bits::ACKNOWLEDGE;
    pub(super) const DRIVER_LAUNCHED: u32 = ACKNOWLEDGED | bits::DRIVER;
    pub(super) const FEATURES_OK: u32 = DRIVER_LAUNCHED | bits::FEATURES_OK;
    pub(super) const DRIVER_OK: u32 = FEATURES_OK | bits::DRIVER_OK;
}
