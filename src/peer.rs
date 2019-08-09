use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub trait PeerId: Eq + Ord + Clone + Debug + Send + Serialize + DeserializeOwned + Sync {}
impl<N> PeerId for N where N: Eq + Ord + Clone + Debug + Send + Serialize + DeserializeOwned + Sync {}

//#[derive(Clone, Debug, Serialize, Deserialize)]
//pub struct Peer<Id: PeerId> {
//    // unique id of the peer
//    #[serde(rename = "PubKeyHex")]
//    pub id: Id,
//    // network / transport address of the peer, in URI format, https://tools.ietf.org/html/rfc3986
//    #[serde(rename = "NetAddr")]
//    pub net_addr: String,
//}

pub trait Peer<Id: PeerId> {
    fn get_id(&self) -> Id;
    fn get_net_addr(&self) -> String;
}

pub trait PeerList<Id: PeerId, Error> {
    // FIXME: we need iter() call implemented in PeerList trait, however
    // we need lifetime parametrised iterator type returned by iter() but
    // Rust currently having unstable generic associated types and produce the
    // following error in an implementation:
    //error[E0109]: lifetime arguments are not allowed for this type
    //   --> src/lib.rs:122:49
    //    |
    //122 |         fn iter<'a>(&'a self) -> Self::IterType<'a> {
    //    |                                                 ^^ lifetime argument not allowed
    //
    // so we leave it for future formal specification, but iter() must be provided in
    // every implementation of PeerList trait.
    //type IterType: Iterator;
    type P: Peer<Id>;
    fn add(&mut self, peer: Self::P) -> std::result::Result<(), Error>;
    fn get_peers_from_file(&mut self, json_peer_path: String) -> std::result::Result<(), Error>;
    fn iter<'a>(&'a self) -> dyn Iterator<Item = &'a Self::P>;
    //fn iter<'a>(&'a self) -> Self::IterType;
}
