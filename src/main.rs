//use std::path::PathBuf;
use tensors::matrix::{Matrix,TensorProcessor,Numbers};
use tensors::mat;

fn main(){
    let mut c: Numbers<i32> = Numbers::new();
    let mut t: Numbers<i32> = Numbers::new();

    let mut m: Numbers<i32> = Numbers::new();
    let mut n = mat![
        i32:
            [1,3],
            [5,7]
    ];


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
        .push(vec![10,10])
        .print();
    m.by(n).print();
}
