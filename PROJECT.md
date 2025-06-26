# ZyncDB Project Overview

## Project Structure

- **crates/core**: Core logic, storage abstraction, WAL, snapshotting, TTL, transactions.
- **crates/parser**: Command parser supporting classic and SQL-like commands, batch, TTL.
- **crates/cli**: Interactive command-line interface for local database access.
- **crates/server**: TCP server for networked access, multi-client support.

## Features

- **Pluggable Storage**: Trait-based, supports in-memory and extensible to file/network backends.
- **Write-Ahead Log (WAL)**: Durable, append-only log for crash recovery.
- **Snapshot & Compaction**: Reduces WAL size, enables fast recovery.
- **TTL/Expiration**: Optional per-key expiry (like Redis).
- **Batch Operations**: Multiple commands in a single request.
- **Transactions**: Begin/commit/rollback support (in-memory).
- **Input Sanitization**: Prevents invalid keys/values.
- **Parser**: Supports classic (`put`, `get`, `delete`) and SQL-like (`insert`, `select`, `remove`) commands.
- **TCP Server**: Multi-client, thread-safe, simple text protocol.
- **CLI**: User-friendly, interactive shell.

## How to Run

### CLI
```sh
cargo run -p cli
```

### Server
```sh
cargo run -p server
```
Connect using `telnet 127.0.0.1 6379` or `nc 127.0.0.1 6379`.

## Example Commands

- `put key value`
- `get key`
- `delete key`
- `insert key value`
- `select key`
- `remove key`
- `ttl key 60`
- `batch put k1 v1 put k2 v2`
- `snapshot`
- `list`
- `exit`

## Next Steps / TODO

- [ ] Add file-based and/or networked storage backends
- [ ] RESP protocol support for Redis client compatibility
- [ ] Authentication/token support
- [ ] Graceful shutdown and WAL flush
- [ ] More tests (integration, fuzzing)
- [ ] Documentation and help command
- [ ] Metrics and logging
- [ ] Replication and clustering (long-term)

---

**ZyncDB** is a learning project inspired by Redis and Cassandra, focused on modularity, durability, and extensibility.