#![feature(try_trait)]
/// # Fantom limcommon-rs
///
/// This library defines a set of commonly used traits for handling distributed networking, including the
/// storage and configuration of individual peers across a single network. The crate strictly
/// defines traits, namely to Peer, PeerId, and PeerList traits, it is up to the developer to implement
/// these traits for their use.
pub mod data;
pub mod errors;
pub mod peer;

// Stub trait for HRTB for struct without any other trait implementation
pub trait Stub {}

/// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_none() -> Result<(), errors::Error> {
        let option = Some(2);

        let _unwrapped = option?;

        Ok(())
    }
}
