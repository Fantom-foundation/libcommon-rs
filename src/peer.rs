/// # Fantom LibCommon-rs/peer.rs
///
/// Defines three traits for the usage of libtransport:
///  - PeerID: Identified individual peers in the network
///  - Peer: Defines what a peer's essential behaviours.
///  - PeerList: Defines a storage of all peers and how it should operate.
use core::fmt::Display;
use core::hash::Hash;
use core::slice::{Iter, IterMut};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::ops::{Index, IndexMut};

/// Allows a type to be implemented for a PeerID.
pub trait PeerId:
    Eq + Ord + Clone + Debug + Send + Serialize + DeserializeOwned + Sync + Hash + Display + Default
{
}
impl<N> PeerId for N where
    N: Eq
        + Ord
        + Clone
        + Debug
        + Send
        + Serialize
        + DeserializeOwned
        + Sync
        + Hash
        + Display
        + Default
{
}

//#[derive(Clone, Debug, Serialize, Deserialize)]
//pub struct Peer<Id: PeerId> {
//    // unique id of the peer
//    #[serde(rename = "PubKeyHex")]
//    pub id: Id,
//    // network / transport address of the peer, in URI format, https://tools.ietf.org/html/rfc3986
//    #[serde(rename = "NetAddr")]
//    pub base_addr: String,
//    pub net_addr: Vec<String>,
//}

/// Defines a networked peer's shared functionality. A peer requires an Id type to be defined before
/// usage.
pub trait Peer<Id: PeerId, Error> {
    /// Creates a new peer, requires an Id type (as defined in implementation), and a net address
    /// as a String.
    fn new(id: Id, base_addr: String) -> Self;
    /// Returns the Id of the peer.
    fn get_id(&self) -> Id;
    /// Returns the peer's base net address.
    fn get_base_addr(&self) -> String;
    /// Returns the peer's nth net address.
    fn get_net_addr(&self, n: usize) -> String;
    /// Set the peer's nth net address.
    fn set_net_addr(&mut self, n: usize, addr: String) -> std::result::Result<(), Error>;
}

/// Defines a list of peers which message and interface over a network.
/// Each PeerList requires the definition of a shared ID type, as well as indexing implementations.
pub trait PeerList<Id: PeerId, Error>: Index<usize> + IndexMut<usize> {
    type P: Peer<Id, Error>;
    /// Creates a new PeerList. Takes no input.
    fn new() -> Self;
    /// Add a new peer to the list. Returns an Error.
    fn add(&mut self, peer: Self::P) -> std::result::Result<(), Error>;
    /// Allows the extraction of peers from a JSON file.
    fn get_peers_from_file(&mut self, json_peer_path: String) -> std::result::Result<(), Error>;
    /// Allows the creation of an iterator over the PeerList.
    fn iter(&self) -> Iter<'_, Self::P>;
    /// Allows the creation of a mutable iterator over the PeerList.
    fn iter_mut(&mut self) -> IterMut<'_, Self::P>;
}
