# Plonky3 on ICICLE: witness-generation

In this repository, we present an integration of the ICICLE field library (rust wrappers) for trace generation and symbolic constraints for AIR arithmetization using the Plonky3 framework. This enables users to

- write AIR circuits in the Plonky3 air language and easily interface with ICICLE APIs.
- generate trace data in ICICLE field types in any device using ICICLE’s device agnostic API’s.
- generate symbolic constraints

As a proof of concept of this integration, we have adapted the Plonky3 examples

- Fibonacci air: This example computes the trace and print the symbolic constraints in ICICLE data types. Run the following from icicle-trace repo

```rust
cargo run --package icicle-trace --example fib_icicle_trace_sym
cargo bench
```

- Blake3 Air: This example computes the trace and symbolic constraints in ICICLE data types for blake3. Run the following from icicle-blake3-air repo

```rust
cargo run --package icicle-blake3-air --example blake3_icicle_trace_sym
cargo bench
```

- Keccak Air: This example computes the trace and symbolic constraints in ICICLE data types for Keccak. Run the following from icicle-keccak-air repo

```rust
cargo run --package icicle-keccak-air --example keccak_icicle_trace_sym
cargo bench
```

* Currently field arithmetic which is not suitable for parallel compute is sent back to Host by default. So even though one might see GPU usage, the witness generation compute happens only in the host. 
* We need to redesign the witness gen to be suitable for GPU compute and it is quite non trivial. We will address this in a future release.

Note: We have currently not implemented a backend prover and will do so in future work. We encourage users to try different air circuits in this framework and build their own STARK provers using the ICICLE framework.

## Acknowledgements

This project is based on the [Plonky3](https://github.com/Plonky3/Plonky3) toolkit for writing AIR circuits.

## Rust Nightly Requirement

**Note:** This project depends on upstream crates (such as Plonky3) that require the unstable Rust 2024 edition (`edition2024`). You must use the latest nightly Rust toolchain to build and run this project.

To update and set nightly as default:

```sh
rustup update nightly
rustup default nightly
```

If you want to use nightly only for this project:

```sh
rustup override set nightly
```

If you encounter errors about `edition2024` or unstable features, ensure you are on the most recent nightly version.
