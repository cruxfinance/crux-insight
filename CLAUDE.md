# crux-insight

Core blockchain indexer for Crux Finance. See [../CLAUDE.md](../CLAUDE.md) for full architecture.

## Role in Architecture

- Connects to Ergo node and indexes all blockchain data into PostgreSQL
- Publishes block events via ZeroMQ port 8765 for ci-modules to consume
- Provides the foundational data layer for all other services

## Structure

- chain-indexer/ - Main indexing service
- ergo-node-client/ - Custom Ergo node client library

## Tech Stack

- Rust with Tokio async runtime
- SeaORM for PostgreSQL
- ZeroMQ tmq for pub/sub

## Dependencies

- Requires indexed Ergo node with ZMQ publisher enabled

## Downstream Consumers

- ci-modules subscribes to ZMQ events
- ci-api queries indexed data
