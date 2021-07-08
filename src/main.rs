//use std::path::PathBuf;
use tensors::matrix::{Matrix,TensorProcessor,Numbers};
use tensors::mat;

fn main(){
    let mut t = Numbers::<i32>::new();
    let mut c = mat![
        f32:
            [1.2,3.4,5.6],
            [7.8,9.10,10.11],
            [12.345,67.89,0.0]
    ];

    let mut m: Numbers<i32> = Numbers::new();
    let mut n = mat![
        i32:
            [1,3],
            [5,7]
    ];


    t.push(vec![1,1,1])
        .push(vec![1,7,2])
        .push(vec![5,2,3]);

    t.print();

    t.debug()
        .add(5)
        .multiple(2)
        .substract(8)
        .divide(3)
        .multiple(13);
    t.replace_col(0,2);
    c.debug();
    c.replace_row(0,2);
    c.add(1.0).print();
    c.transpose().print();

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
