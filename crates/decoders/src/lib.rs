pub mod drift_v2;

/// 8 byte unique identifier for a type.
pub(crate) trait Discriminator {
    const DISCRIMINATOR: [u8; 8];
    fn discriminator() -> [u8; 8] {
        Self::DISCRIMINATOR
    }
}
