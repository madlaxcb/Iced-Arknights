#![allow(missing_docs)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hud_core::{chamfer_clamp, chamfered_rect_points, snap_to_pixel};

fn geometry_benchmarks(c: &mut Criterion) {
    c.bench_function("chamfer_clamp", |b| {
        b.iter(|| chamfer_clamp(black_box(320.0), black_box(180.0), black_box(12.0)))
    });

    c.bench_function("chamfered_rect_points", |b| {
        b.iter(|| chamfered_rect_points(black_box(320.0), black_box(180.0), black_box(12.0)))
    });

    c.bench_function("snap_to_pixel_150_percent", |b| {
        b.iter(|| snap_to_pixel(black_box(37.3), black_box(1.5)))
    });
}

criterion_group!(benches, geometry_benchmarks);
criterion_main!(benches);
