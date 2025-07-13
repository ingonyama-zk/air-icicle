use icicle_core::traits::Arithmetic;
use icicle_core::field::Field;
use icicle_core::bignum::BigNum;

use icicle_babybear::field::ScalarField as Fr;
//constraints
use p3_matrix::dense::RowMajorMatrix;
use p3_matrix::Matrix;
use rand::Rng;
//comment out lib to run this file

pub fn gen_random_matrix(width: usize, height: usize) -> RowMajorMatrix<Fr> {
    let mut rng = rand::thread_rng(); //
    let elements: Vec<Fr> = (0..width * height)
        .map(|_| Fr::from_u32(rng.gen()))
        .collect();

    RowMajorMatrix::new(elements, width)
}
pub fn gen_random_vector(dim: usize) -> Vec<Fr> {
    let mut rng = rand::thread_rng(); //
    let elements: Vec<Fr> = (0..dim).map(|_| Fr::from_u32(rng.gen())).collect();
    elements
}

fn main() {
    let width: usize = 2;
    let height: usize = 4;
    let m: RowMajorMatrix<Fr> = gen_random_matrix(width, height);
    println!("matrix of dimensions {:?}, is {:#?}", m.dimensions(), m);
    let _v = gen_random_vector(height);
}
