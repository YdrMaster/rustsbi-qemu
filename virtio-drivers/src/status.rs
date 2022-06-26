// § 2.1 Device Status Field

use volatile_register::RW;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(transparent)]
pub struct DeviceStatusBits(pub u32);

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(u32)]
pub enum DeviceStatus {
    Uninitialized = 0,
    Acknowledged = 1,
    Driver = 2 | 1,
    FeaturesOK = 8 | 2 | 1,
    DriverOK = 8 | 4 | 2 | 1,
    Failed = 128,
    DeviceNeedsReset = 64,
}

pub fn test_and_push(
    status: &RW<DeviceStatusBits>,
    expected: DeviceStatus,
) -> Result<DeviceStatus, Result<DeviceStatus, DeviceStatusBits>> {
    use DeviceStatus::*;
    let next = match DeviceStatus::try_from(status.read()) {
        Ok(current) if current == expected => match current {
            Uninitialized => Acknowledged,
            Acknowledged => Driver,
            Driver => FeaturesOK,
            FeaturesOK => DriverOK,
            DriverOK | Failed | DeviceNeedsReset => return Ok(current),
        },
        current => return Err(current),
    };
    unsafe { status.write(DeviceStatusBits(next as _)) };
    Ok(next)
}

impl TryFrom<DeviceStatusBits> for DeviceStatus {
    type Error = DeviceStatusBits;

    fn try_from(value: DeviceStatusBits) -> Result<Self, Self::Error> {
        if value.0 & DEVICE_NEEDS_RESET != 0 {
            Ok(Self::DeviceNeedsReset)
        } else if value.0 & FAILED != 0 {
            Ok(Self::Failed)
        } else {
            match value.0 {
                DRIVER_OK => Ok(Self::DriverOK),
                FEATURES_OK => Ok(Self::FeaturesOK),
                DRIVER => Ok(Self::Driver),
                ACKNOWLEDGED => Ok(Self::Acknowledged),
                UNINITIALIZED => Ok(Self::Uninitialized),
                _ => Err(value),
            }
        }
    }
}

const UNINITIALIZED: u32 = DeviceStatus::Uninitialized as _;
const ACKNOWLEDGED: u32 = DeviceStatus::Acknowledged as _;
const DRIVER: u32 = DeviceStatus::Driver as _;
const FEATURES_OK: u32 = DeviceStatus::FeaturesOK as _;
const DRIVER_OK: u32 = DeviceStatus::DriverOK as _;
const FAILED: u32 = DeviceStatus::Failed as _;
const DEVICE_NEEDS_RESET: u32 = DeviceStatus::DeviceNeedsReset as _;
