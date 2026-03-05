use std::{path::PathBuf, process::Command, time::Instant};

use criterion::{Criterion, criterion_group, criterion_main};
use escargot::CargoBuild;

fn benchmarks(c: &mut Criterion) {
    let pysrc = PathBuf::from("./tests/fixtures/basic/src.py");
    let pt_bin = CargoBuild::new()
        .bin("pt")
        .current_release()
        .run()
        .unwrap()
        .path()
        .to_owned();
    c.bench_function("pt", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for _ in 0..iters {
                let _ = Command::new(&pt_bin).arg(&pysrc).output().unwrap();
            }
            start.elapsed()
        })
    });
    c.bench_function("pytest", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for _ in 0..iters {
                let _ = Command::new("pytest").arg(&pysrc).output().unwrap();
            }
            start.elapsed()
        })
    });
}

criterion_group!(basic, benchmarks);
criterion_main!(basic);
