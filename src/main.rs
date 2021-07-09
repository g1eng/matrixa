//use std::path::PathBuf;
use tensors::core::{Matrix,TensorProcessor,Numbers};
use tensors::mat;

fn main(){
    let mut t = Numbers::<i32>::new();
    let mut c = mat![
        f32:
            [1.2, 3.4, 3.4, 4.5],
            [7.8, 9.10, 112.3, 456.78],
            [12.345, 67.89, 0.0, 0.0]
    ];

    let mut m: Numbers<i32> = Numbers::new();
    let mut n = mat![
        i32:
            [1,3],
            [5,7]
    ];


    t
        .debug()
        .push(vec![1,1,1])
        .push(vec![1,7,2])
        .push(vec![5,2,3]);

    t.print();

    t
        .add(5)
        .mul(2)
        .sub(8)
        .div(3)
        .mul(13);
    t.col_replace(0,2);
    t.residue(2);
    t.transpose().print();

    c.debug();
    c.row_replace(0,1);
    c.col_replace(0,3);
    c.add(1.0).print();

    println!("by");

    m
        .push(vec![1,2,3])
        .push(vec![4,5,7])
        .print();
    n
        .push(vec![10,10])
        .print();
    m.prod(n).unwrap().print();

    println!("{:?}", m.get());

    mat![
        i32:
            [1,3,5],
            [7,11,13],
            [17,19,23]
    ].prod(
        mat![
        i32:
            [29,31,37],
            [39,41,43],
            [47,53,59]
        ]
    ).unwrap().print();
}
