/// # Fantom limcommon-rs
///
/// This library defines a set of commonly used traits for handling distributed networking, including the
/// storage and configuration of individual peers across a single network. The crate strictly
/// defines traits, namely to Peer, PeerId, and PeerList traits, it is up to the developer to implement
/// these traits for their use.


pub mod errors;
pub mod peer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
