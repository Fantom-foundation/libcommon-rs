libcommon-rs
===========
[![Build Status](https://travis-ci.org/Fantom-foundation/libcommon-rs.svg?branch=master)](https://travis-ci.org/Fantom-foundation/libcommon-rs)

libcommon-rs in Rust.

## RFCs

https://github.com/Fantom-foundation/fantom-rfcs

# Developer guide

Install the latest version of [Rust](https://www.rust-lang.org). We tend to use nightly versions. [CLI tool for installing Rust](https://rustup.rs).

We use [rust-clippy](https://github.com/rust-lang-nursery/rust-clippy) linters to improve code quality.

There are plenty of [IDEs](https://areweideyet.com) and other [Rust development tools to consider](https://github.com/rust-unofficial/awesome-rust#development-tools).

### Description

This library defines a set of commonly used traits for handling distributed networking, including the
storage and configuration of individual peers across a single network. The crate strictly
defines traits, namely the Peer, PeerId, and PeerList traits, it is up to the developer to implement
these traits for their individual use.

### Example Implementation

```
// Example code extracted from libtransport/generic_tests.rs repository: 
https://github.com/Fantom-foundation/libtransport/blob/master/src/generic_test.rs

// Dummy ID, also uses a u32 for instantiation.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Id(pub u32);

// A simple test struct for holding peer information. This includes both an id and an address.
// NOTE: This specific implementation is only for testing purposes.
pub struct TestPeer<Id> {
    pub id: Id,
    pub net_addr: String,
}

// Implement the Peer trait for TestPeer.
impl Peer<Id> for TestPeer<Id> {
    // Create a new peer
    fn new(id: Id, addr: String) -> TestPeer<Id> {
        TestPeer { id, net_addr: addr }
    }
    // Getter for the Id
    fn get_id(&self) -> Id {
        self.id.clone()
    }
    // Getter for the inputted address
    fn get_net_addr(&self) -> String {
        self.net_addr.clone()
    }
}

// Creation of our own PeerList type (used for testing purposes)
pub struct TestPeerList<Id> {
    peers: Vec<TestPeer<Id>>,
}

// Allows the use of indexing to access data within the peer list.
impl<Id> Index<usize> for TestPeerList<Id> {
    type Output = TestPeer<Id>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.peers[index]
    }
}

// Allows the use of indexing to access mutable data within the peer list.
impl<Id> IndexMut<usize> for TestPeerList<Id> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.peers[index]
    }
}

// Implementation of PeerList for our TestPeerList struct.
impl PeerList<Id, Error> for TestPeerList<Id> {
    type P = TestPeer<Id>;

    // Constructor
    fn new() -> Self {
        TestPeerList {
            peers: Vec::with_capacity(1),
        }
    }
    // Function which allows adding new peers to our peer list.
    fn add(&mut self, p: TestPeer<Id>) -> std::result::Result<(), Error> {
        // Check if we're at max capacity
        if self.peers.len() == std::usize::MAX {
            return Err(AtMaxVecCapacity);
        }
        // Push value into vec
        self.peers.push(p);

        Ok(())
    }
    // Loads peers in from a json file. Not relevant to this test.
    fn get_peers_from_file(&mut self, _json_peer_path: String) -> std::result::Result<(), Error> {
        // Stub not used in tests to satisfy PeerList trait
        Ok(())
    }
    // Allows iteration over the peer list.
    fn iter(&self) -> Iter<'_, Self::P> {
        self.peers.iter()
    }
    // Allows a mutable iteration over the peer list.
    fn iter_mut(&mut self) -> IterMut<'_, Self::P> {
        self.peers.iter_mut()
    }
}
```

### Step-by-step guide
```bash
# Install Rust (nightly)
$ curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly
# Install cargo-make (cross-platform feature-rich reimplementation of Make)
$ cargo install --force cargo-make
# Install rustfmt (Rust formatter)
$ rustup component add rustfmt
# Clone this repo
$ git clone https://github.com/Fantom-foundation/libcommon-rs && cd libcommon-rs
# Run tests
$ cargo test
# Format, build and test
$ cargo make
```
