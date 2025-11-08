# nova-easing

![Rust CI](https://github.com/timblechmann/nova-easing/actions/workflows/rust.yml/badge.svg)

A collection of generic easing functions, supporting portable SIMD.

This crate provides a variety of easing functions that can be applied to scalar
types (`f32`, `f64`) and, with the `nightly` feature, to SIMD vectors from
`std::simd`.

Compare [easings.net](https://easings.net/)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
nova-easing = "0.1.0"
```

### Scalar Easing

The easing functions can be used on `f32` and `f64` values.

```rust
use nova_easing::EasingArgument;

fn main() {
    let x = 0.5f32;
    let eased = x.ease_in_out_sine();
    println!("Eased value: {}", eased); // Eased value: 0.5
}
```

### SIMD Easing (Nightly Only)

To use easing functions with `std::simd` types, you need to enable the `nightly`
feature in your `Cargo.toml` and use a nightly Rust toolchain.

```toml
[dependencies]
nova-easing = { version = "0.1.0", features = ["nightly"] }
```

Then you can apply easing functions to SIMD vectors:

```rust
#![feature(portable_simd)]

use nova_easing::EasingArgument;
use std::simd::f32x4;

fn main() {
    let values = f32x4::from_array([0.1, 0.3, 0.5, 0.8]);
    let eased_values = values.ease_in_quad();
    println!("Eased values: {:?}", eased_values.as_array());
}
```

## API Overview

The crate provides easing functions for `f32`, `f64`, and SIMD types (`f32x4`,
`f64x4`, etc. with `nightly` feature).

All easing functions follow the pattern `ease_{in|out|in_out}_{type}`, where
`type` is one of: `quad`, `cubic`, `quart`, `quint`, `sine`, `circ`, `back`,
`bounce`, `expo`, `elastic`.

### Available Functions

- `ease_in_quad`, `ease_out_quad`, `ease_in_out_quad`
- `ease_in_cubic`, `ease_out_cubic`, `ease_in_out_cubic`
- `ease_in_quart`, `ease_out_quart`, `ease_in_out_quart`
- `ease_in_quint`, `ease_out_quint`, `ease_in_out_quint`
- `ease_in_sine`, `ease_out_sine`, `ease_in_out_sine`
- `ease_in_circ`, `ease_out_circ`, `ease_in_out_circ`
- `ease_in_back`, `ease_out_back`, `ease_in_out_back`
- `ease_in_bounce`, `ease_out_bounce`, `ease_in_out_bounce`
- `ease_in_expo`, `ease_out_expo`, `ease_in_out_expo`
- `ease_in_elastic`, `ease_out_elastic`, `ease_in_out_elastic`

For visual plots of each function, see [easings.net](https://easings.net/).

## Performance Notes

- SIMD versions leverage hardware vectorization for significant performance
gains when processing multiple values simultaneously.
- Scalar versions are optimized for single-value operations.
- All implementations are branchless where possible for better performance.

## Building the Demo

The included demo binary generates visual plots for all easing functions. To build
and run it, you will need a nightly Rust toolchain and enable the `nightly` and `demo` features:

```bash
rustup default nightly
cargo run --bin demo --features nightly,demo
```

This will create PNG plots in the `demo_plots/` directory, with subdirectories for
`f32` and `f32x4` variants.

Note: The `demo` feature is optional and includes the `plotters` dependency for generating plots. It is not required for using the easing functions.

## Running Benchmarks

To run performance benchmarks for all easing functions:

```bash
rustup default nightly
cargo bench --features nightly
```

This will run benchmarks for all 180 functions (30 easing functions × 6 types:
f32, f64, f32x4, f32x8, f64x2, f64x4) and generate HTML reports in the
`target/criterion/` directory, providing detailed performance comparisons.

## License

This project is licensed under the MIT License.
