//use std::path::PathBuf;
use tensors::matrix::{Matrix,TensorProcessor,I32Matrix,F32Matrix};

fn main(){
    let mut c = I32Matrix::new();
    let mut t = I32Matrix::new();

    let mut m = I32Matrix::new();
    let mut n = I32Matrix::new();


    t.push(vec![1,1,1])
        .push(vec![1,7,2])
        .push(vec![5,2,3]);

    c.push(vec![1,2,3])
        .push(vec![4,5,6])
        .push(vec![7,8,9]);

    /***************************
    c.push(vec![3.33,3.33,3.33])
        .push(vec![3.3,3.0,6.66])
        .push(vec![3.3,3.3,9.0]);
    ***************************/

    t.print();

    t
        .add(5)
        .multiple(2)
        .substract(8)
        .divide(3)
        .multiple(13)
        .residue(7)
        .print();
    t.replace_col(0,2).print();
    c.replace_row(0,2).print();
    c.translate().print();


    println!("by");
    m
        .push(vec![1,2,3])
        .push(vec![4,5,7])
        .print();
    n
        .push(vec![1,3])
        .push(vec![5,7])
        .push(vec![10,10])
        .print();
    m.by(n).print();
}
