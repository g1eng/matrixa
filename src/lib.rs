pub mod core;
pub use self::core::{Matrix,Numbers,TensorProcessor};

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
