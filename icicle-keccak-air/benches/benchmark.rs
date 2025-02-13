use icicle_keccak_air::KeccakAir;

use icicle_babybear::field::ScalarField as Fr;

use icicle_trace::symbolic_builder::*;
use p3_matrix::Matrix;

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
    let keccak_air = KeccakAir {};
    let trace = keccak_air.generate_trace_rows::<Fr>(n, 0);
}


const SAMPLES:usize = 1<<15;

pub fn keccak_trace_cpu(c:&mut Criterion){
    set_backend_cpu();
    let mut group = c.benchmark_group("keccak trace");
    group.bench_function("keccak_cpu", |b| b.iter(|| test_trace(SAMPLES)));
    
    group.finish();
}

pub fn keccak_trace_gpu(c:&mut Criterion){
    try_load_and_set_backend_gpu();
    let mut group = c.benchmark_group("keccak trace");
    group.bench_function("keccak_gpu", |b| b.iter(|| test_trace(SAMPLES)));
    
    group.finish();
}
criterion_group!(benches, keccak_trace_cpu,keccak_trace_gpu);
criterion_main!(benches);