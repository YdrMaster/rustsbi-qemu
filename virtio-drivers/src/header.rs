use crate::U32Str;
use core::fmt;
use volatile_register::RO;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Header {
    magic_value: Magic,
    version: Version,
    device_id: DeviceType,
    vendor_id: U32Str,
}

#[derive(Debug)]
pub enum Error {
    InvalidMagic,
    InvalidVersion,
    UnknownType(u32),
}

impl Header {
    pub fn from_raw_parts(addr: usize) -> Result<Header, Error> {
        use Error::*;
        /// 根据文档，前三个寄存器必须按顺序访问以在不符合要求时尽量减少误操作。
        #[repr(C)]
        struct Unchecked {
            magic_value: RO<u32>,
            version: RO<u32>,
            device_id: RO<u32>,
            vendor_id: RO<u32>,
        }

        let unchecked = unsafe { &*(addr as *const Unchecked) };
        Ok(Header {
            magic_value: Some(Magic(U32Str(unchecked.magic_value.read())))
                .filter(|m| m == &MAGIC)
                .ok_or(InvalidMagic)?,
            version: Version::try_from(unchecked.version.read()).map_err(|_| InvalidVersion)?,
            device_id: DeviceType::try_from(unchecked.device_id.read())
                .map_err(|e| UnknownType(e.number))?,
            vendor_id: U32Str(unchecked.vendor_id.read()),
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Magic(U32Str);

const MAGIC: Magic = Magic(U32Str::new("virt"));

impl fmt::Debug for Magic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum Version {
    Legacy = 1,
    Normal = 2,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum DeviceType {
    NetworkCard = 1,
    BlockDevice = 2,
    Console = 3,
    EntropySource = 4,
    MemoryBallooningTraditional = 5,
    IoMemory = 6,
    Rpmsg = 7,
    ScsiHost = 8,
    Transport9P = 9,
    Mac80211Wlan = 10,
    RprocSerial = 11,
    VirtioCaif = 12,
    MemoryBalloon = 13,
    GpuDevice = 16,
    TimerOrClockDevice = 17,
    InputDevice = 18,
    SocketDevice = 19,
    CryptoDevice = 20,
    SignalDistributionModule = 21,
    PstoreDevice = 22,
    IommuDevice = 23,
    MemoryDevice = 24,
    AudioDevice = 25,
    FileSystemDevice = 26,
    PmemDevice = 27,
    RpmbDevice = 28,
    Mac80211HwsimWireSimulationDevice = 29,
    VidioEncoderDevice = 30,
    VidioDecoderDevice = 31,
    ScmiDevice = 32,
    NitroSecureModule = 33,
    I2cAdapter = 34,
    Watchdog = 35,
    CanDevice = 36,
    ParameterServer = 38,
    AudioPolicyDevice = 39,
    BluetoothDevice = 40,
    GpioDevice = 41,
    RdmaDevice = 42,
}
