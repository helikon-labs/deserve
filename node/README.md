# DeServe Node Implementation

The node executable, whose primary purpose is to execute the [runtime](../runtime/README.md).

🔗 It communicates with other nodes in the network, and aims for
[consensus](https://wiki.polkadot.network/docs/learn-consensus) among them.

⚙️ It acts as a remote procedure call (RPC) server, allowing interaction with the blockchain.

👇 Here are the most important files in this node template:

- [`chain_spec.rs`](./src/chain_spec.rs): Source code file that defines the chain's initial (genesis) state for devnet and testnet configurations.
- [`service.rs`](./src/service.rs): The node implementation definition. It's a place to configure consensus-related topics.
