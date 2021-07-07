pub mod matrix;
pub use self::matrix::{Matrix,I32Matrix,F32Matrix};

pub struct I32TensorSet {
    pub dataset: Vec<I32Matrix>,
    current: usize,
    rowlen: usize,
    max: usize,
}

pub struct F32TensorSet {
    pub dataset: Vec<F32Matrix>,
    current: usize,
    rowlen: usize,
    max: usize,
}

pub trait TensorLinearOperator {
    fn add(&self);
    fn substract(&self);
    fn multiple(&self);
    fn divide(&self);
    fn residue(&self);
}

pub trait TensorOperator {
    fn multiple(&self);
    fn directive(&self);
    fn check(&self);
}

pub trait VectorLinearOperator {
    fn multiple(&self);
    fn directive(&self);
    fn check(&self);
}

pub trait VectorOperator {
    fn multiple(&self);
    fn directive(&self);
    fn check(&self);
}
