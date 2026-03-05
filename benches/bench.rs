use std::{path::PathBuf, process::Command};

use criterion::{Criterion, criterion_group, criterion_main};
use escargot::CargoBuild;

fn benchmarks(c: &mut Criterion) {
    let pysrc = PathBuf::from("./tests/fixtures/basic/src.py");
    let mut pt_cmd = CargoBuild::new().bin("pt").current_release().run().unwrap().command();
    let mut pytest_cmd = Command::new("pytest");
    pytest_cmd.arg(pysrc.as_os_str());
    c.bench_function("pt", |b| b.iter(|| pt_cmd.output()));
    c.bench_function("pytest", |b| b.iter(|| pt_cmd.output()));
}

criterion_group!(basic, benchmarks);
criterion_main!(basic);
