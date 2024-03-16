# TODO

## General

## Deps

### Hyper

- [ ] Update to latest version of Hyper
  - https://hyper.rs/guides/1/upgrading/ 


## Targets

### FreeBSD

- fails to compile due to [os_info bug](https://github.com/stanislav-tkach/os_info/pull/372) (introduced by pact-plugin-driver)
- aarch64 failes to compile tower

```console
error[E0107]: struct takes 3 generic arguments but 2 generic arguments were supplied
  --> /home/runner/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tower-0.4.13/src/ready_cache/cache.rs:65:25
   |
65 |     pending_cancel_txs: IndexMap<K, CancelTx>,
   |                         ^^^^^^^^ -  -------- supplied 2 generic arguments
   |                         |
   |                         expected 3 generic arguments
   |
note: struct defined here, with 3 generic parameters: `K`, `V`, `S`
  --> /home/runner/.cargo/registry/src/index.crates.io-6f17d22bba15001f/indexmap-1.9.3/src/map.rs:76:12
   |
76 | pub struct IndexMap<K, V, S> {
   |            ^^^^^^^^ -  -  -
help: add missing generic argument
   |
65 |     pending_cancel_txs: IndexMap<K, CancelTx, S>,
   |                                             +++

error[E0107]: struct takes 3 generic arguments but 2 generic arguments were supplied
  --> /home/runner/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tower-0.4.13/src/ready_cache/cache.rs:73:12
   |
73 |     ready: IndexMap<K, (S, CancelPair)>,
   |            ^^^^^^^^ -  --------------- supplied 2 generic arguments
   |            |
   |            expected 3 generic arguments
   |
note: struct defined here, with 3 generic parameters: `K`, `V`, `S`
  --> /home/runner/.cargo/registry/src/index.crates.io-6f17d22bba15001f/indexmap-1.9.3/src/map.rs:76:12
   |
76 | pub struct IndexMap<K, V, S> {
   |            ^^^^^^^^ -  -  -
help: add missing generic argument
   |
73 |     ready: IndexMap<K, (S, CancelPair), S>,
   |                                       +++

For more information about this error, try `rustc --explain E0107`.
error: could not compile `tower` (lib) due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
[cross] warning: rust-std is not available for aarch64-unknown-freebsd
[cross] note: you may need to build components for the target via `-Z build-std=<components>` or in your cross configuration specify `target.aarch64-unknown-freebsd.build-std`
              the available components are core, std, alloc, and proc_macro
Error: Process completed with exit code 101.

```

Related issues

- https://github.com/tower-rs/tower/issues/466
- https://github.com/indexmap-rs/indexmap/issues/144
  
### NetBSD

- fails to compile due to [os_info bug](https://github.com/stanislav-tkach/os_info/pull/374) (introduced by pact-plugin-driver)
  