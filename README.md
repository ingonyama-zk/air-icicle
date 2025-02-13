# witness-generation

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

In order for this integration to be functional we had to redefine some properties of the Field trait in ICICLE, and the ICICLE branch used for this purpose can be accesed in the [p3 branch](https://github.com/ingonyama-zk/icicle/tree/p3) of the ICICLE repository.

The trace will still run on a GPU due to ICICLE device agnostic API's. Note however it is not yet optimized for this purpose. Nevertheless in order to run the benches on the GPU download the [CUDA backend release 3.4](https://github.com/ingonyama-zk/icicle/releases/tag/v3.4.0) and install it in the '
`cuda_backend` folder in the repo.

Note: We have currently not implemented a backend prover and will do so in future work. We encourage users to try different air circuits in this framework and build their own STARK provers using the ICICLE framework.

## Acknowledgements

This project is based on the [Plonky3](https://github.com/Plonky3/Plonky3) toolkit for writing AIR circuits.
