use core::slice::{Iter, IterMut};
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
    fn new(id: Id, net_addr: String) -> Self;
    fn get_id(&self) -> Id;
    fn get_net_addr(&self) -> String;
}

pub trait PeerList<Id: PeerId, Error> {
    type P: Peer<Id>;
    fn new() -> Self;
    fn add(&mut self, peer: Self::P) -> std::result::Result<(), Error>;
    fn get_peers_from_file(&mut self, json_peer_path: String) -> std::result::Result<(), Error>;
    fn iter(&self) -> Iter<'_, Self::P>;
    fn iter_mut(&mut self) -> IterMut<'_, Self::P>;
}
