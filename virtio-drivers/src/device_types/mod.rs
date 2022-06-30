﻿//! § 5 Device Types

mod net;

pub use net::NetMeta;

#[derive(Clone, Copy, PartialEq, Eq, Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum DeviceType {
    Reserved = 0,
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