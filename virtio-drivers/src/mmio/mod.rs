// todo
mod legacy;
// todo
mod modern;

pub use legacy::Interface as MmioLegacyInterface;
pub use modern::Interface as MmioInterface;
