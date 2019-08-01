use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait PeerId: Eq + Ord + Clone + Debug + Send + Serialize + Sync {}
impl<N> PeerId for N where N: Eq + Ord + Clone + Debug + Send + Serialize + Sync {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Peer<Id: PeerId> {
    // unique id of the peer
    #[serde(rename = "PubKeyHex")]
    pub id: Id,
    // network / transport address of the peer, in URI format, https://tools.ietf.org/html/rfc3986
    #[serde(rename = "NetAddr")]
    pub net_addr: String,
}

pub trait PeerList<Id: PeerId, Error>: Iterator {
    fn add(&mut self, p: Peer<Id>) -> std::result::Result<(), Error>;
    fn get_peers_from_file(&mut self, json_peer_path: String) -> std::result::Result<(), Error>;
}
