// Copyright (C) 2025 Tim Blechmann
// SPDX-License-Identifier: MIT

#![feature(portable_simd)]


use nova_easing::EasingArgument;
use plotters::prelude::*;
use std::path::Path;

#[cfg(feature = "nightly")]
use std::simd::f32x4;

fn generate_samples_f32<F>(func: F) -> Vec<(f32, f32)>
where
    F: Fn(f32) -> f32,
{
    (0..512)
        .map(|i| {
            let x = i as f32 / 511.0;
            let y = func(x);
            (x, y)
        })
        .collect()
}

#[cfg(feature = "nightly")]
fn generate_samples_f32x4<F>(func: F) -> Vec<(f32, f32)>
where
    F: Fn(f32x4) -> f32x4,
{
    (0..512)
        .map(|i| {
            let x = i as f32 / 511.0;
            let input = f32x4::splat(x);
            let output = func(input);
            (x, output[0])
        })
        .collect()
}

fn plot_samples(samples: &[(f32, f32)], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create parent directory if it doesn't exist
    let path = Path::new(filename);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let root = BitMapBackend::new(filename, (512, 512)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..1f32, -0.3f32..1.3f32)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(samples.iter().cloned(), &RED))?;
    root.present()?;
    Ok(())
}

macro_rules! generate_plots {
    ($func_name:ident) => {{
        let samples = generate_samples_f32(|x| EasingArgument::$func_name(x));
        plot_samples(
            &samples,
            concat!("demo_plots/f32/", stringify!($func_name), ".png"),
        )
        .unwrap();
        println!("Generated plot for {} f32", stringify!($func_name));

        #[cfg(feature = "nightly")]
        {
            let samples = generate_samples_f32x4(|x| EasingArgument::$func_name(x));
            plot_samples(
                &samples,
                concat!("demo_plots/f32x4/", stringify!($func_name), ".png"),
            )
            .unwrap();
            println!("Generated plot for {} f32x4", stringify!($func_name));
        }
    }};
}

fn main() {
    println!("Generating easing function plots...");

    generate_plots!(ease_in_quad);
    generate_plots!(ease_out_quad);
    generate_plots!(ease_in_out_quad);
    generate_plots!(ease_in_cubic);
    generate_plots!(ease_out_cubic);
    generate_plots!(ease_in_out_cubic);
    generate_plots!(ease_in_quart);
    generate_plots!(ease_out_quart);
    generate_plots!(ease_in_out_quart);
    generate_plots!(ease_in_quint);
    generate_plots!(ease_out_quint);
    generate_plots!(ease_in_out_quint);
    generate_plots!(ease_in_sine);
    generate_plots!(ease_out_sine);
    generate_plots!(ease_in_out_sine);
    generate_plots!(ease_in_circ);
    generate_plots!(ease_out_circ);
    generate_plots!(ease_in_out_circ);
    generate_plots!(ease_in_back);
    generate_plots!(ease_out_back);
    generate_plots!(ease_in_out_back);
    generate_plots!(ease_in_bounce);
    generate_plots!(ease_out_bounce);
    generate_plots!(ease_in_out_bounce);
    generate_plots!(ease_in_expo);
    generate_plots!(ease_out_expo);
    generate_plots!(ease_in_out_expo);
    generate_plots!(ease_in_elastic);
    generate_plots!(ease_out_elastic);
    generate_plots!(ease_in_out_elastic);

    // Generate plots for ease_in_curve with different curve factors
    let curve_factors = [-4.0, -1.0, 0.0, 1.0, 4.0];
    for &curve in &curve_factors {
        let samples = generate_samples_f32(|x| EasingArgument::ease_in_curve(x, curve));
        let filename = format!(
            "demo_plots/f32/ease_in_curve_{}.png",
            if curve < 0.0 {
                format!("neg{}", curve.abs() as i32)
            } else {
                curve.to_string()
            }
        );
        plot_samples(&samples, &filename).unwrap();
        println!("Generated plot for ease_in_curve f32 with curve {}", curve);

        #[cfg(feature = "nightly")]
        {
            let samples = generate_samples_f32x4(|x| EasingArgument::ease_in_curve(x, curve));
            let filename = format!(
                "demo_plots/f32x4/ease_in_curve_{}.png",
                if curve < 0.0 {
                    format!("neg{}", curve.abs() as i32)
                } else {
                    curve.to_string()
                }
            );
            plot_samples(&samples, &filename).unwrap();
            println!(
                "Generated plot for ease_in_curve f32x4 with curve {}",
                curve
            );
        }
    }

    // Generate plots for ease_out_curve with different curve factors
    for &curve in &curve_factors {
        let samples = generate_samples_f32(|x| EasingArgument::ease_out_curve(x, curve));
        let filename = format!(
            "demo_plots/f32/ease_out_curve_{}.png",
            if curve < 0.0 {
                format!("neg{}", curve.abs() as i32)
            } else {
                curve.to_string()
            }
        );
        plot_samples(&samples, &filename).unwrap();
        println!("Generated plot for ease_out_curve f32 with curve {}", curve);

        #[cfg(feature = "nightly")]
        {
            let samples = generate_samples_f32x4(|x| EasingArgument::ease_out_curve(x, curve));
            let filename = format!(
                "demo_plots/f32x4/ease_out_curve_{}.png",
                if curve < 0.0 {
                    format!("neg{}", curve.abs() as i32)
                } else {
                    curve.to_string()
                }
            );
            plot_samples(&samples, &filename).unwrap();
            println!(
                "Generated plot for ease_out_curve f32x4 with curve {}",
                curve
            );
        }
    }

    // Generate plots for ease_in_out_curve with different curve factors
    for &curve in &curve_factors {
        let samples = generate_samples_f32(|x| EasingArgument::ease_in_out_curve(x, curve));
        let filename = format!(
            "demo_plots/f32/ease_in_out_curve_{}.png",
            if curve < 0.0 {
                format!("neg{}", curve.abs() as i32)
            } else {
                curve.to_string()
            }
        );
        plot_samples(&samples, &filename).unwrap();
        println!(
            "Generated plot for ease_in_out_curve f32 with curve {}",
            curve
        );

        #[cfg(feature = "nightly")]
        {
            let samples = generate_samples_f32x4(|x| EasingArgument::ease_in_out_curve(x, curve));
            let filename = format!(
                "demo_plots/f32x4/ease_in_out_curve_{}.png",
                if curve < 0.0 {
                    format!("neg{}", curve.abs() as i32)
                } else {
                    curve.to_string()
                }
            );
            plot_samples(&samples, &filename).unwrap();
            println!(
                "Generated plot for ease_in_out_curve f32x4 with curve {}",
                curve
            );
        }
    }

    println!("All plots generated in demo_plots/");
}
