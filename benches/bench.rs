use std::{path::PathBuf, process::Command, time::Instant};

use criterion::{Criterion, criterion_group, criterion_main};
use escargot::CargoBuild;

fn benchmarks(c: &mut Criterion) {
    let basic = PathBuf::from("./tests/fixtures/basic/src.py");
    let complex = PathBuf::from("./tests/fixtures/basic/src.py");
    let pt_bin = CargoBuild::new()
        .bin("pt")
        .current_release()
        .run()
        .unwrap()
        .path()
        .to_owned();
    c.bench_function("pt_basic", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for _ in 0..iters {
                let _ = Command::new(&pt_bin).arg(&basic).output().unwrap();
            }
            start.elapsed()
        })
    });
    c.bench_function("pytest_basic", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for _ in 0..iters {
                let _ = Command::new("pytest").arg(&basic).output().unwrap();
            }
            start.elapsed()
        })
    });
        c.bench_function("pt_complex", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for _ in 0..iters {
                let _ = Command::new(&pt_bin).arg(&complex).output().unwrap();
            }
            start.elapsed()
        })
    });
    c.bench_function("pytest_complex", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for _ in 0..iters {
                let _ = Command::new("pytest").arg(&complex).output().unwrap();
            }
            start.elapsed()
        })
    });

}

criterion_group!(basic, benchmarks);
criterion_main!(basic);
