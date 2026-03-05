use std::path::PathBuf;

use assert_cmd::{Command, cargo::cargo_bin_cmd};
use criterion::{Criterion, criterion_group, criterion_main};

fn benchmarks(c: &mut Criterion) {
    let pysrc = PathBuf::from("./tests/fixtures/basic/src.py");
    let mut pt_cmd = cargo_bin_cmd!("pt");
    pt_cmd.arg(pysrc.as_os_str());
    let mut pytest_cmd = Command::new("pytest");
    pytest_cmd.arg(pysrc.as_os_str());
    c.bench_function("pt", |b| b.iter(|| pt_cmd.output()));
    c.bench_function("pytest", |b| b.iter(|| pt_cmd.output()));
}

criterion_group!(basic, benchmarks);
criterion_main!(basic);
