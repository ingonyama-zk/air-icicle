use icicle_blake3_air::Blake3Air;

use icicle_babybear::field::ScalarField as Fr;

use criterion::{criterion_group, criterion_main, Criterion};

use icicle_runtime::Device;

pub fn set_backend_cpu() {
    let device_cpu = Device::new("CPU", 0);
    icicle_runtime::set_device(&device_cpu).unwrap();
}

pub fn try_load_and_set_backend_gpu() {
    icicle_runtime::load_backend("../cuda_backend").unwrap();
    let device_gpu = Device::new("CUDA", 0);
    let is_cuda_device_available = icicle_runtime::is_device_available(&device_gpu);
    if is_cuda_device_available {
        icicle_runtime::set_device(&device_gpu).unwrap();
    } else {
        set_backend_cpu();
    }
}

fn test_trace(n: usize) {
    let blake3_air = Blake3Air {};
    let trace = blake3_air.generate_trace_rows::<Fr>(n);
    // let symbolic_constraints = get_symbolic_constraints::<Fr, Blake3Air>(&Blake3Air {}, 0, 0);
}

const SAMPLES: usize = 1 << 15;

pub fn blake3_cpu(c: &mut Criterion) {
    set_backend_cpu();
    let mut group = c.benchmark_group("blake3_cpu");
    group.bench_function("blak3_cpu", |b| b.iter(|| test_trace(SAMPLES)));

    group.finish();
}

pub fn blake3_gpu(c: &mut Criterion) {
    try_load_and_set_backend_gpu();
    let mut group = c.benchmark_group("blake3_gpu");
    group.bench_function("blake3_gpu", |b| b.iter(|| test_trace(SAMPLES)));

    group.finish();
}
criterion_group!(benches, blake3_cpu, blake3_gpu);
criterion_main!(benches);
