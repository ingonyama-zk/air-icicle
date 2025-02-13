use criterion::{criterion_group, criterion_main, Criterion};
use icicle_core::{
    field::Field,
    traits::{Arithmetic, FieldImpl},
};
use icicle_runtime::Device;
use icicle_babybear::field::ScalarField as Fr;
//constraints


use p3_matrix::dense::RowMajorMatrix;

const SAMPLES:usize = 32768;
const NUM_FIBONACCI_COLS: usize = 2;

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
pub struct FibonacciRow<F> {
    pub left: F,
    pub right: F,
}
impl<F> FibonacciRow<F> {
    const fn new(left: F, right: F) -> FibonacciRow<F> {
        FibonacciRow { left, right }
    }
}
pub fn generate_trace_rows<F: FieldImpl + Arithmetic>(
    a: u32,
    b: u32,
    n: usize,
) -> RowMajorMatrix<F> {
    assert!(n.is_power_of_two());
    let mut trace =
        RowMajorMatrix::new(vec![F::zero(); n * NUM_FIBONACCI_COLS], NUM_FIBONACCI_COLS);

    let (prefix, rows, suffix) = unsafe { trace.values.align_to_mut::<FibonacciRow<F>>() };
    assert!(prefix.is_empty(), "Alignment should match");
    assert!(suffix.is_empty(), "Alignment should match");
    assert_eq!(rows.len(), n);

    rows[0] = FibonacciRow::new(F::from_u32(a), F::from_u32(b));

    for i in 1..n {
        rows[i].left = rows[i - 1].right.clone();
        rows[i].right = rows[i - 1].left.clone() + rows[i - 1].right.clone();
    }
    trace
}

fn test_trace(n: usize, x: u32) {
    // generate trace
    let trace = generate_trace_rows::<Fr>(0, 1, n);
     //input public value     
}

pub fn fib_cpu(c:&mut Criterion){
    set_backend_cpu();
    let mut group = c.benchmark_group("fib_cpu");
    group.bench_function("fib_cpu", |b| b.iter(|| test_trace(1 << 15, 32768)));
    
    group.finish();
}

pub fn fib_gpu(c:&mut Criterion){
    try_load_and_set_backend_gpu();
    let mut group = c.benchmark_group("fib_gpu");
    group.bench_function("fib_gpu", |b| b.iter(|| test_trace(1 << 15, 32768)));
    
    group.finish();
}
criterion_group!(benches, fib_cpu,fib_gpu);
criterion_main!(benches);