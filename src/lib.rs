pub mod matrix;
pub use self::matrix::{Matrix,Numbers};
//pub use self::list::{DataList,StrList};

#[macro_export]
macro_rules! mat {
    ( $t:ty : $( [ $( $x:expr ),+ ] ),* ) => {
        {
            let mut matrix = Numbers::new("i32");
            let mut vec_len = 0;
            let mut row = 0;
            $(
                let mut t_vec = Vec::new();
                $(
                    t_vec.push($x);
                )*
                if vec_len == 0 {
                    vec_len = t_vec.len();
                } else if vec_len != t_vec.len() {
                    panic!("invalid vector length for {:?} on row {}!", t_vec, row)
                }
                row += 1;
                matrix.data.push(t_vec);
            )*
            matrix
        }
    };
    ( $x:ty ) => {
        {
            Numbers::new($x)
        }
    };
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
