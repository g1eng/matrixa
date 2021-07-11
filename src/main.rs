use tensors::core::{List, Matrix};
use tensors::mat;

fn main() {
    let mut c = mat![
        f32:
            [1.2, 3.4, 3.4, 4.5],
            [7.8, 9.10, 112.3, 456.78],
            [12.345, 67.89, 0.0, 0.0],
            [3.0, 1.11, 31.3, 1.11]
    ];
    let mut d = mat![
        f64:
            [1.0,2.0,0.0],
            [3.0,1.0,2.0],
            [-1.0,3.0,1.0]
    ];

    println!("det for {:?} is {}.", c.dump(), c.det());
    println!("adjugate[{},{}] for", 1, 2,);
    for e in c.dump() {
        println!("{:?}", e);
    }
    println!("is");
    for e in c.adjugate(1, 2).unwrap().dump() {
        println!("{:?}", e);
    }
    println!("");
    println!("d is ");
    for e in d.dump() {
        println!("{:?}", e);
    }
    for _ in 0..10 {
        d.set(d.inverse().unwrap().dump());
    }
    println!("d inverse 10 is {:?}", d.dump());

    /******************************
    let mut t = Matrix::<i32>::new();
    let mut c = mat![
        f32:
            [1.2, 3.4, 3.4, 4.5],
            [7.8, 9.10, 112.3, 456.78],
            [12.345, 67.89, 0.0, 0.0]
    ];

    let mut m: Matrix<i32> = Matrix::new();
    let mut n = mat![
        i32:
            [1,3],
            [5,7]
    ];

    let p = mat![
        i32:
            [10,10],
            [10,10]
    ];
    let q = mat![
        i32:
            [1,2],
            [3,4]
    ];
    let r = mat![
        i32:
            [21,32],
            [43,54]
    ];
    let s = mat![
        i32:
            [2,3],
            [2,3]
    ];




    t
        .debug()
        .push(vec![1,1,1])
        .unwrap()
        .push(vec![1,7,2])
        .unwrap()
        .push(vec![5,2,3])
        .unwrap();

    t.print();

    t
        .add(5)
        .mul(2)
        .sub(8)
        .div(3)
        .mul(13);
    t.col_replace(0,2)
        .unwrap()
        .residue(2)
        .transpose()
        .print();

    c.debug();
    c
        .row_replace(0,1)
        .unwrap()
        .col_replace(0,2)
        .unwrap()
        .add(1.0).print();

    println!("by");

    m
        .push(vec![1,2,3])
        .unwrap()
        .push(vec![4,5,7])
        .unwrap()
        .print();
    n
        .push(vec![10,10])
        .unwrap()
        .print();
    //m.prod(n).unwrap().print();

    println!("{:?}", m.dump());

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

    println!("sum");
    let e = p + q;
    e.print();
    let e = r - e;
    e.print();
    let e = s * e;
    e.print();

    let mut x = mat![
        i32:
            [1,2,3],
            [4,5,7]
    ];
    x.debug();
    let y = mat![
        i32:
            [1,3],
            [5,7],
            [10,10]
    ];
    let z = x * y;
    z.print();
    ******************************/

    let s = mat![
        &str:
            ["これは","事例"],
            ["特に","重要"]
    ];
    s.print();

    let mut matstr = mat![String];
    for e in s {
        let mut v = Vec::new();
        for p in e {
            let mut q = String::from(p);
            q.push_str("オッケー");
            v.push(q);
        }
        matstr.push(v).expect("文字列ベクトルの長さが不正です");
    }
    matstr.print();
}
