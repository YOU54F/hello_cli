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
  

## PB Structs

```rust
#[derive(Debug, Deserialize, Serialize)]
struct PactBrokerHalLinks {
    #[serde(rename = "self")]
    self_link: Link,
    #[serde(rename = "pb:publish-pact")]
    publish_pact: Link,
    #[serde(rename = "pb:publish-contracts")]
    publish_contracts: Link,
    #[serde(rename = "pb:latest-pact-versions")]
    latest_pact_versions: Link,
    #[serde(rename = "pb:tagged-pact-versions")]
    tagged_pact_versions: Link,
    #[serde(rename = "pb:pacticipants")]
    pacticipants: Link,
    #[serde(rename = "pb:pacticipant")]
    pacticipant: Link,
    #[serde(rename = "pb:latest-provider-pacts")]
    latest_provider_pacts: Link,
    #[serde(rename = "pb:latest-provider-pacts-with-tag")]
    latest_provider_pacts_with_tag: Link,
    #[serde(rename = "pb:provider-pacts-with-tag")]
    provider_pacts_with_tag: Link,
    #[serde(rename = "pb:provider-pacts")]
    provider_pacts: Link,
    #[serde(rename = "pb:latest-version")]
    latest_version: Link,
    #[serde(rename = "pb:latest-tagged-version")]
    latest_tagged_version: Link,
    #[serde(rename = "pb:webhooks")]
    webhooks: Link,
    #[serde(rename = "pb:webhook")]
    webhook: Link,
    #[serde(rename = "pb:integrations")]
    integrations: Link,
    #[serde(rename = "pb:pacticipant-version-tag")]
    pacticipant_version_tag: Link,
    #[serde(rename = "pb:pacticipant-branch")]
    pacticipant_branch: Link,
    #[serde(rename = "pb:pacticipant-branch-version")]
    pacticipant_branch_version: Link,
    #[serde(rename = "pb:pacticipant-version")]
    pacticipant_version: Link,
    #[serde(rename = "pb:metrics")]
    metrics: Link,
    #[serde(rename = "pb:can-i-deploy-pacticipant-version-to-tag")]
    can_i_deploy_pacticipant_version_to_tag: Link,
    #[serde(rename = "pb:can-i-deploy-pacticipant-version-to-environment")]
    can_i_deploy_pacticipant_version_to_environment: Link,
    #[serde(rename = "pb:provider-pacts-for-verification")]
    provider_pacts_for_verification: Link,
    #[serde(rename = "beta:provider-pacts-for-verification")]
    beta_provider_pacts_for_verification: Link,
    curies: Vec<Curies>,
    #[serde(rename = "pb:environments")]
    environments: Link,
    #[serde(rename = "pb:environment")]
    environment: Link,
}

```