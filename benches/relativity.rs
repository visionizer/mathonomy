use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mathonomy::consts::*;
use mathonomy::relativity::*;

const SPEEDS_TO_MEASURE_AT: [f64; 10] = [
    0f64,
    100f64,
    SPEED_OF_LIGHT,
    SPEED_OF_LIGHT / 2f64,
    SPEED_OF_LIGHT - 1f64,
    SPEED_OF_LIGHT / 4f64,
    SPEED_OF_LIGHT / 1f64,
    10000000f64,
    SPEED_OF_LIGHT / 10000f64,
    83823847328f64,
];

const MASSES_TO_MEASURE_AT: [f64; 3] = [1_000_000f64, 1f64, 0.1f64];

fn bench_lorentz(c: &mut Criterion) {
    let mut group = c.benchmark_group("Lorentz Factor");

    for speed in SPEEDS_TO_MEASURE_AT {
        group.bench_with_input("Lorentz Factor", &speed, |b, i| {
            b.iter(|| lorentz::slorentz(*i));
        });
    }
}

fn bench_relative_kinetic_energy(c: &mut Criterion) {
    let mut group = c.benchmark_group("Relative Kinetic Energy");

    for speed in SPEEDS_TO_MEASURE_AT {
        for mass in MASSES_TO_MEASURE_AT {
            group.bench_with_input("Relative Kinetic Energy", &(speed, mass), |b, i| {
                b.iter(|| energy::skinetic_energy(speed, mass));
            });
        }
    }
}

criterion_group!(lorentz, bench_lorentz);
criterion_group!(energy, bench_relative_kinetic_energy);

criterion_main!(lorentz, energy);
