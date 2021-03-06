use std::collections::HashSet;

use zebra_chain::block::BlockHeaderHash;

use super::super::types::Nonce;

/// A network request, represented in internal format.
///
/// The network layer aims to abstract away the details of the Bitcoin wire
/// protocol into a clear request/response API. Each [`Request`] documents the
/// possible [`Response`s](super::Response) it can generate; it is fine (and
/// recommended!) to match on the expected responses and treat the others as
/// `unreachable!()`, since their return indicates a bug in the network code.
#[derive(Clone, Debug)]
pub enum Request {
    /// Requests additional peers from the server.
    ///
    /// # Response
    ///
    /// Returns [`Response::Peers`](super::Response::Peers).
    Peers,

    /// Heartbeats triggered on peer connection start.
    ///
    /// This is included as a bit of a hack, it should only be used
    /// internally for connection management. You should not expect to
    /// be firing or handling `Ping` requests or `Pong` responses.
    #[doc(hidden)]
    Ping(Nonce),

    /// Request block data by block hashes.
    ///
    /// This uses a `HashSet` rather than a `Vec` for two reasons. First, it
    /// automatically deduplicates the requested blocks. Second, the internal
    /// protocol translator needs to maintain a `HashSet` anyways, in order to
    /// keep track of which requested blocks have been received and when the
    /// request is ready. Rather than force the internals to always convert into
    /// a `HashSet`, we require the caller to pass one, so that if the caller
    /// didn't start with a `Vec` but with, e.g., an iterator, they can collect
    /// directly into a `HashSet` and save work.
    ///
    /// # Returns
    ///
    /// Returns [`Response::Blocks`](super::Response::Blocks).
    BlocksByHash(HashSet<BlockHeaderHash>),

    /// Request block hashes of subsequent blocks in the chain, giving hashes of
    /// known blocks.
    ///
    /// # Returns
    ///
    /// Returns
    /// [`Response::BlockHeaderHashes`](super::Response::BlockHeaderHashes).
    ///
    /// # Warning
    ///
    /// This is implemented by sending a `getblocks` message. Bitcoin nodes
    /// respond to `getblocks` with an `inv` message containing a list of the
    /// subsequent blocks. However, Bitcoin nodes *also* send `inv` messages
    /// unsolicited in order to gossip new blocks to their peers. These gossip
    /// messages can race with the response to a `getblocks` request, and there
    /// is no way for the network layer to distinguish them. For this reason, the
    /// response may occasionally contain a single hash of a new chain tip rather
    /// than a list of hashes of subsequent blocks. We believe that unsolicited
    /// `inv` messages will always have exactly one block hash.
    FindBlocks {
        /// Hashes of known blocks, ordered from highest height to lowest height.
        known_blocks: Vec<BlockHeaderHash>,
        /// Optionally, the last header to request.
        stop: Option<BlockHeaderHash>,
    },
}
