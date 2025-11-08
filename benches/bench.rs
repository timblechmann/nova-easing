// Copyright (C) 2025 Tim Blechmann
// SPDX-License-Identifier: MIT

#![feature(portable_simd)]

use criterion::{Criterion, criterion_group, criterion_main};
use nova_easing::EasingArgument;
use paste::paste;
use std::hint::black_box;

#[cfg(feature = "nightly")]
use std::simd::{f32x4, f32x8, f64x2, f64x4};

macro_rules! generate_benches {
    ($type:ty, $prefix:ident, $x:expr) => {
        paste! {
            fn [<$prefix _ease_in_quad>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_in_quad>]), |b| b.iter(|| black_box($x).ease_in_quad()));
            }
            fn [<$prefix _ease_out_quad>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_out_quad>]), |b| b.iter(|| black_box($x).ease_out_quad()));
            }
            fn [<$prefix _ease_in_out_quad>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_in_out_quad>]), |b| b.iter(|| black_box($x).ease_in_out_quad()));
            }
            fn [<$prefix _ease_in_cubic>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_in_cubic>]), |b| b.iter(|| black_box($x).ease_in_cubic()));
            }
            fn [<$prefix _ease_out_cubic>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_out_cubic>]), |b| b.iter(|| black_box($x).ease_out_cubic()));
            }
            fn [<$prefix _ease_in_out_cubic>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_in_out_cubic>]), |b| b.iter(|| black_box($x).ease_in_out_cubic()));
            }
            fn [<$prefix _ease_in_quart>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_in_quart>]), |b| b.iter(|| black_box($x).ease_in_quart()));
            }
            fn [<$prefix _ease_out_quart>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_out_quart>]), |b| b.iter(|| black_box($x).ease_out_quart()));
            }
            fn [<$prefix _ease_in_out_quart>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_in_out_quart>]), |b| b.iter(|| black_box($x).ease_in_out_quart()));
            }
            fn [<$prefix _ease_in_quint>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_in_quint>]), |b| b.iter(|| black_box($x).ease_in_quint()));
            }
            fn [<$prefix _ease_out_quint>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_out_quint>]), |b| b.iter(|| black_box($x).ease_out_quint()));
            }
            fn [<$prefix _ease_in_out_quint>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_in_out_quint>]), |b| b.iter(|| black_box($x).ease_in_out_quint()));
            }
            fn [<$prefix _ease_in_sine>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_in_sine>]), |b| b.iter(|| black_box($x).ease_in_sine()));
            }
            fn [<$prefix _ease_out_sine>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_out_sine>]), |b| b.iter(|| black_box($x).ease_out_sine()));
            }
            fn [<$prefix _ease_in_out_sine>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_in_out_sine>]), |b| b.iter(|| black_box($x).ease_in_out_sine()));
            }
            fn [<$prefix _ease_in_circ>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_in_circ>]), |b| b.iter(|| black_box($x).ease_in_circ()));
            }
            fn [<$prefix _ease_out_circ>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_out_circ>]), |b| b.iter(|| black_box($x).ease_out_circ()));
            }
            fn [<$prefix _ease_in_out_circ>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_in_out_circ>]), |b| b.iter(|| black_box($x).ease_in_out_circ()));
            }
            fn [<$prefix _ease_in_back>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_in_back>]), |b| b.iter(|| black_box($x).ease_in_back()));
            }
            fn [<$prefix _ease_out_back>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_out_back>]), |b| b.iter(|| black_box($x).ease_out_back()));
            }
            fn [<$prefix _ease_in_out_back>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_in_out_back>]), |b| b.iter(|| black_box($x).ease_in_out_back()));
            }
            fn [<$prefix _ease_in_bounce>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_in_bounce>]), |b| b.iter(|| black_box($x).ease_in_bounce()));
            }
            fn [<$prefix _ease_out_bounce>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_out_bounce>]), |b| b.iter(|| black_box($x).ease_out_bounce()));
            }
            fn [<$prefix _ease_in_out_bounce>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_in_out_bounce>]), |b| b.iter(|| black_box($x).ease_in_out_bounce()));
            }
            fn [<$prefix _ease_in_expo>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_in_expo>]), |b| b.iter(|| black_box($x).ease_in_expo()));
            }
            fn [<$prefix _ease_out_expo>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_out_expo>]), |b| b.iter(|| black_box($x).ease_out_expo()));
            }
            fn [<$prefix _ease_in_out_expo>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_in_out_expo>]), |b| b.iter(|| black_box($x).ease_in_out_expo()));
            }
            fn [<$prefix _ease_in_elastic>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_in_elastic>]), |b| b.iter(|| black_box($x).ease_in_elastic()));
            }
            fn [<$prefix _ease_out_elastic>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_out_elastic>]), |b| b.iter(|| black_box($x).ease_out_elastic()));
            }
            fn [<$prefix _ease_in_out_elastic>](c: &mut Criterion) {
                c.bench_function(stringify!([<$prefix _ease_in_out_elastic>]), |b| b.iter(|| black_box($x).ease_in_out_elastic()));
            }
        }
    };
}

generate_benches!(f32, bench_f32, 0.5f32);
generate_benches!(f64, bench_f64, 0.5f64);
#[cfg(feature = "nightly")]
generate_benches!(f32x4, bench_f32x4, f32x4::splat(0.5));
#[cfg(feature = "nightly")]
generate_benches!(f32x8, bench_f32x8, f32x8::splat(0.5));
#[cfg(feature = "nightly")]
generate_benches!(f64x2, bench_f64x2, f64x2::splat(0.5));
#[cfg(feature = "nightly")]
generate_benches!(f64x4, bench_f64x4, f64x4::splat(0.5));

criterion_group!(
    benches_f32,
    bench_f32_ease_in_quad,
    bench_f32_ease_out_quad,
    bench_f32_ease_in_out_quad,
    bench_f32_ease_in_cubic,
    bench_f32_ease_out_cubic,
    bench_f32_ease_in_out_cubic,
    bench_f32_ease_in_quart,
    bench_f32_ease_out_quart,
    bench_f32_ease_in_out_quart,
    bench_f32_ease_in_quint,
    bench_f32_ease_out_quint,
    bench_f32_ease_in_out_quint,
    bench_f32_ease_in_sine,
    bench_f32_ease_out_sine,
    bench_f32_ease_in_out_sine,
    bench_f32_ease_in_circ,
    bench_f32_ease_out_circ,
    bench_f32_ease_in_out_circ,
    bench_f32_ease_in_back,
    bench_f32_ease_out_back,
    bench_f32_ease_in_out_back,
    bench_f32_ease_in_bounce,
    bench_f32_ease_out_bounce,
    bench_f32_ease_in_out_bounce,
    bench_f32_ease_in_expo,
    bench_f32_ease_out_expo,
    bench_f32_ease_in_out_expo,
    bench_f32_ease_in_elastic,
    bench_f32_ease_out_elastic,
    bench_f32_ease_in_out_elastic
);

criterion_group!(
    benches_f64,
    bench_f64_ease_in_quad,
    bench_f64_ease_out_quad,
    bench_f64_ease_in_out_quad,
    bench_f64_ease_in_cubic,
    bench_f64_ease_out_cubic,
    bench_f64_ease_in_out_cubic,
    bench_f64_ease_in_quart,
    bench_f64_ease_out_quart,
    bench_f64_ease_in_out_quart,
    bench_f64_ease_in_quint,
    bench_f64_ease_out_quint,
    bench_f64_ease_in_out_quint,
    bench_f64_ease_in_sine,
    bench_f64_ease_out_sine,
    bench_f64_ease_in_out_sine,
    bench_f64_ease_in_circ,
    bench_f64_ease_out_circ,
    bench_f64_ease_in_out_circ,
    bench_f64_ease_in_back,
    bench_f64_ease_out_back,
    bench_f64_ease_in_out_back,
    bench_f64_ease_in_bounce,
    bench_f64_ease_out_bounce,
    bench_f64_ease_in_out_bounce,
    bench_f64_ease_in_expo,
    bench_f64_ease_out_expo,
    bench_f64_ease_in_out_expo,
    bench_f64_ease_in_elastic,
    bench_f64_ease_out_elastic,
    bench_f64_ease_in_out_elastic
);

#[cfg(feature = "nightly")]
criterion_group!(
    benches_f32x4,
    bench_f32x4_ease_in_quad,
    bench_f32x4_ease_out_quad,
    bench_f32x4_ease_in_out_quad,
    bench_f32x4_ease_in_cubic,
    bench_f32x4_ease_out_cubic,
    bench_f32x4_ease_in_out_cubic,
    bench_f32x4_ease_in_quart,
    bench_f32x4_ease_out_quart,
    bench_f32x4_ease_in_out_quart,
    bench_f32x4_ease_in_quint,
    bench_f32x4_ease_out_quint,
    bench_f32x4_ease_in_out_quint,
    bench_f32x4_ease_in_sine,
    bench_f32x4_ease_out_sine,
    bench_f32x4_ease_in_out_sine,
    bench_f32x4_ease_in_circ,
    bench_f32x4_ease_out_circ,
    bench_f32x4_ease_in_out_circ,
    bench_f32x4_ease_in_back,
    bench_f32x4_ease_out_back,
    bench_f32x4_ease_in_out_back,
    bench_f32x4_ease_in_bounce,
    bench_f32x4_ease_out_bounce,
    bench_f32x4_ease_in_out_bounce,
    bench_f32x4_ease_in_expo,
    bench_f32x4_ease_out_expo,
    bench_f32x4_ease_in_out_expo,
    bench_f32x4_ease_in_elastic,
    bench_f32x4_ease_out_elastic,
    bench_f32x4_ease_in_out_elastic
);

#[cfg(feature = "nightly")]
criterion_group!(
    benches_f32x8,
    bench_f32x8_ease_in_quad,
    bench_f32x8_ease_out_quad,
    bench_f32x8_ease_in_out_quad,
    bench_f32x8_ease_in_cubic,
    bench_f32x8_ease_out_cubic,
    bench_f32x8_ease_in_out_cubic,
    bench_f32x8_ease_in_quart,
    bench_f32x8_ease_out_quart,
    bench_f32x8_ease_in_out_quart,
    bench_f32x8_ease_in_quint,
    bench_f32x8_ease_out_quint,
    bench_f32x8_ease_in_out_quint,
    bench_f32x8_ease_in_sine,
    bench_f32x8_ease_out_sine,
    bench_f32x8_ease_in_out_sine,
    bench_f32x8_ease_in_circ,
    bench_f32x8_ease_out_circ,
    bench_f32x8_ease_in_out_circ,
    bench_f32x8_ease_in_back,
    bench_f32x8_ease_out_back,
    bench_f32x8_ease_in_out_back,
    bench_f32x8_ease_in_bounce,
    bench_f32x8_ease_out_bounce,
    bench_f32x8_ease_in_out_bounce,
    bench_f32x8_ease_in_expo,
    bench_f32x8_ease_out_expo,
    bench_f32x8_ease_in_out_expo,
    bench_f32x8_ease_in_elastic,
    bench_f32x8_ease_out_elastic,
    bench_f32x8_ease_in_out_elastic
);

#[cfg(feature = "nightly")]
criterion_group!(
    benches_f64x2,
    bench_f64x2_ease_in_quad,
    bench_f64x2_ease_out_quad,
    bench_f64x2_ease_in_out_quad,
    bench_f64x2_ease_in_cubic,
    bench_f64x2_ease_out_cubic,
    bench_f64x2_ease_in_out_cubic,
    bench_f64x2_ease_in_quart,
    bench_f64x2_ease_out_quart,
    bench_f64x2_ease_in_out_quart,
    bench_f64x2_ease_in_quint,
    bench_f64x2_ease_out_quint,
    bench_f64x2_ease_in_out_quint,
    bench_f64x2_ease_in_sine,
    bench_f64x2_ease_out_sine,
    bench_f64x2_ease_in_out_sine,
    bench_f64x2_ease_in_circ,
    bench_f64x2_ease_out_circ,
    bench_f64x2_ease_in_out_circ,
    bench_f64x2_ease_in_back,
    bench_f64x2_ease_out_back,
    bench_f64x2_ease_in_out_back,
    bench_f64x2_ease_in_bounce,
    bench_f64x2_ease_out_bounce,
    bench_f64x2_ease_in_out_bounce,
    bench_f64x2_ease_in_expo,
    bench_f64x2_ease_out_expo,
    bench_f64x2_ease_in_out_expo,
    bench_f64x2_ease_in_elastic,
    bench_f64x2_ease_out_elastic,
    bench_f64x2_ease_in_out_elastic
);

#[cfg(feature = "nightly")]
criterion_group!(
    benches_f64x4,
    bench_f64x4_ease_in_quad,
    bench_f64x4_ease_out_quad,
    bench_f64x4_ease_in_out_quad,
    bench_f64x4_ease_in_cubic,
    bench_f64x4_ease_out_cubic,
    bench_f64x4_ease_in_out_cubic,
    bench_f64x4_ease_in_quart,
    bench_f64x4_ease_out_quart,
    bench_f64x4_ease_in_out_quart,
    bench_f64x4_ease_in_quint,
    bench_f64x4_ease_out_quint,
    bench_f64x4_ease_in_out_quint,
    bench_f64x4_ease_in_sine,
    bench_f64x4_ease_out_sine,
    bench_f64x4_ease_in_out_sine,
    bench_f64x4_ease_in_circ,
    bench_f64x4_ease_out_circ,
    bench_f64x4_ease_in_out_circ,
    bench_f64x4_ease_in_back,
    bench_f64x4_ease_out_back,
    bench_f64x4_ease_in_out_back,
    bench_f64x4_ease_in_bounce,
    bench_f64x4_ease_out_bounce,
    bench_f64x4_ease_in_out_bounce,
    bench_f64x4_ease_in_expo,
    bench_f64x4_ease_out_expo,
    bench_f64x4_ease_in_out_expo,
    bench_f64x4_ease_in_elastic,
    bench_f64x4_ease_out_elastic,
    bench_f64x4_ease_in_out_elastic
);

#[cfg(feature = "nightly")]
criterion_main!(
    benches_f32,
    benches_f64,
    benches_f32x4,
    benches_f32x8,
    benches_f64x2,
    benches_f64x4
);
#[cfg(not(feature = "nightly"))]
criterion_main!(benches_f32, benches_f64);
