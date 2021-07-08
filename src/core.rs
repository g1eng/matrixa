//! Tensors::Matrix is a struct for matrices manipulation.
//! 
//!
//! ```rust
//! use tensors::core::{Matrix,TensorProcessor,Numbers};
//! use tensors::mat;
//!
//!  let mut im = Numbers::<i32>::new();
//!  let mut fm = mat![f32: [1.0,2.0,3.0]];
//!  im.push(vec![1,2,3,4,5])
//!    .push(vec![5,6,7,8,9]);
//!  fm.push(vec![1.23,4.56,7.89]);
//!  im.add(1).print();
//!  im.multiple(3).print();
//!  fm.print();
//! ```
//!

pub trait Matrix {
    fn print(&self);
    fn replace_row(&mut self, src: usize, dst: usize) -> &mut Self;
    fn replace_col(&mut self, src: usize, dst: usize) -> &mut Self;
    fn transpose(&mut self) -> &mut Self;
    fn check_zero_len(&self);
    fn debug(&mut self) -> &mut Self;
}

pub struct Numbers<T> {
    pub data: Vec<Vec<T>>,
    current: usize,
    max: usize,
    debug: bool,
}

pub trait TensorProcessor<T> {
    fn zero(&mut self) -> &mut Self;
    fn add(&mut self, val: T) -> &mut Self;
    fn substract(&mut self, val: T) -> &mut Self;
    fn multiple(&mut self, val: T) -> &mut Self;
    fn divide(&mut self, val: T) -> &mut Self;
    fn by(&mut self, val: Numbers<T>) -> &mut Self;
}

// + std::ops::{AddAssign,SubAssign,MulAssign,DivAssign}
impl<T: std::fmt::Debug> Numbers<T> {
    pub fn new() -> Self {
        let v: Vec<Vec<T>> = Vec::new();
        Numbers {
            data: v,
            current: 0,
            max: 0,
            debug: false,
        }
    }
    pub fn push(&mut self, data: Vec<T>) -> &mut Self {
        self.max += 1;
        let rowlen = self.data.len();

        if rowlen != 0 {
            if self.data[0].len() != data.len(){
                panic!("Invalid vector length: {}, expected: {}",data.len(), rowlen);
            }
        }
        if self.debug {
            println!("pushing {:?}", data);
        }
        self.data.push(data);
        self
    }
}

impl<T> Matrix for Numbers<T> 
where
    T: std::fmt::Debug + Copy
{
    fn print(&self){
        println!("{:?}",self.data)
    }

    fn debug(&mut self) -> &mut Self {
        self.debug = true;
        println!("debug on");
        println!("{:?}",self.data);
        self
    }

    //行列サイズ確認
    // 長さ0で強制終了
    fn check_zero_len(&self) {
        if self.data.len() == 0 {
            panic!("zero matrix length detected");
        }
    }

    //行置換操作
    //
    fn replace_row(&mut self, src: usize, dst: usize) -> &mut Self {

        if src >= self.data.len() {
            panic!("src address out of range {}", src);
        } else if dst >= self.data.len() {
            panic!("dst address out of range {}", dst);
        }


        let mut src_data = Vec::new();
        let mut dst_data = Vec::new();

        for i in 0..self.data.len() {
            src_data.push(self.data[src][i]);
            dst_data.push(self.data[dst][i]);
        }
        self.data[src] = dst_data;
        self.data[dst] = src_data;
        if self.debug {
            println!("matrix row replacement: {} with {}", src, dst );
            println!("{:?}",self.data);
        }
        self
    }

    //列置換操作
    //
    fn replace_col(&mut self, src: usize, dst: usize) -> &mut Self {

        if src >= self.data[0].len() {
            panic!("src address out of range {}", src);
        } else if dst >= self.data[0].len() {
            panic!("dst address out of range {}", dst);
        }

        for i in 0..self.data.len() {
            for _ in 0..self.data[0].len() {
                let src_data = self.data[i][src];
                let dst_data = self.data[i][dst];
                self.data[i][src] = dst_data;
                self.data[i][dst] = src_data;
            }
        }
        if self.debug {
            println!("matrix column replacement: {} with {}", src, dst );
            println!("{:?}",self.data);
        }
        self

    }

    // 転置
    //
    fn transpose (&mut self) -> &mut Self {
        if self.data.len() != self.data[0].len() {
            panic!("not a square matrix");
        }
        let limit = self.data.len() / 2;

        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                if i<=limit && j >= limit {
                    let src = self.data[i][j];
                    let dst = self.data[j][i];
                    self.data[i][j] = dst;
                    self.data[j][i] = src;
                }
            }
        }
        if self.debug {
            println!("matrix transpose");
            println!("{:?}",self.data);
        }
        self
   }


}

impl<T> Iterator for Numbers<T> where T: Clone {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Vec<T>> {
        self.current += 1;
        if self.current - 1 < self.max {
            let data = &self.data[self.current-1];
            Some(data.to_vec())
        } else {
            None
        }
    }
}

impl<T> Numbers <T>
where
    T: Copy + std::ops::RemAssign + std::fmt::Display + std::fmt::Debug
{
    //剰余計算
    pub fn residue (&mut self, val: T) -> &mut Self {
        for i in 0 .. self.data.len() {
            for j in 0..self.data[0].len() {
                self.data[i][j] %= val;
            }
        }
        if self.debug {
            println!("residue {} foreach", val);
            println!("{:?}",self.data);
        }
        self
    }

}

impl<T> TensorProcessor<T> for Numbers<T>
where
    T: Copy + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign + std::ops::DivAssign + std::ops::Sub<Output=T> + std::ops::Mul<Output=T> + std::fmt::Display + std::fmt::Debug
{

    //型Tにおけるゼロ値でデータを初期化
    fn zero(&mut self) -> &mut Self {
        for i in 0..self.data.len(){
            for j in 0..self.data[0].len(){
                self.data[i][j] = self.data[i][j] - self.data[i][j];
            }
        }
        self
    }

    //一括加算
    //
    fn add(&mut self, val: T) -> &mut Self {
        //self.check_zero_len();
        for i in 0 .. self.data.len() {
            for j in 0..self.data[0].len() {
                self.data[i][j] += val;
            }
        }
        if self.debug {
            println!("add {} foreach", val);
            println!("{:?}",self.data);
        }
        self
    }

    //一括減算
    //
    fn substract(&mut self, val: T) -> &mut Self {
        //self.check_zero_len();
        for i in 0 .. self.data.len() {
            for j in 0..self.data[0].len() {
                self.data[i][j] -= val;
            }
        }
        if self.debug {
            println!("sub {} foreach", val);
            println!("{:?}",self.data);
        }
        self
    }

    //一括乗算
    //
    fn multiple(&mut self, val: T) -> &mut Self {
        //self.check_zero_len();
        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                self.data[i][j] *= val;
            }
        }
        if self.debug {
            println!("mul {} foreach", val);
            println!("{:?}",self.data);
        }
        self
    }

    //一括除算
    // (i32デフォルトでは端数切捨て)
    fn divide(&mut self, val: T) -> &mut Self {
        //self.check_zero_len();
        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                self.data[i][j] /= val;
            }
        }
        if self.debug {
            println!("divide {} foreach", val);
            println!("{:?}",self.data);
        }
        self
    }

    //行列積
    fn by (&mut self, m: Numbers<T>) -> &mut Self {
        //m.check_zero_len();
        if self.data.len() != m.data[0].len() {
            panic!("row length not matched to the col length of argument")
        } else if self.data[0].len() != m.data.len() {
            panic!("col length not matched to the row length of argument")
        }
        //解行列のサイズ
        let res_length: usize = self.data.len();
        let mut res: Vec<Vec<T>> = Vec::new();

        //解行列の初期化
        while res.len() < res_length {
            res.push(Vec::new());
        }
        for i in 0..res_length {
            let mut j = 0;
            while res[i].len() < res_length {
                res[i].push( self.data[i][j] - self.data[i][j] );
                j += 1;
            }
        }
        
        //解行列の計算
        for i in 0..self.data.len() {
            for j in 0..m.data.len() {
                for seq in 0..res_length {
                    //println!("res[{}][{}] += self.data[{}][{}] + m.data[{}][{}] = {}", i, seq, seq, j, j, seq, self.data[seq][j] * m.data[j][seq]);
                    res[i][seq] += self.data[i][j] * m.data[j][seq];
                }
            }
        }
        //println!("res: {:?}",res);

        // self.data 行列のリサイズ
        while self.data.len() < res_length {
            self.data.push(Vec::new());
        }
        for i in 0..self.data.len() {
            while self.data[i].len() > res_length {
                self.data[i].pop();
            }
        }


        //解行列のselfへの適用
        for i in 0..res_length {
            for j in 0..res_length {
                self.data[i][j] = res[i][j];
            }
        }
        //println!("by");

        if self.debug {
            println!("matrix multiple for m: {:?}", m.data);
            println!("{:?}",self.data);
        }

        self
    }
}


// 行列インスンタンス初期化用マクロ
//
// ```rust
// use tensors::matrix::{Matrix,TensorProcessor,Numbers};
// use tensors::mat;
//
//  let mut fm = mat![f32: [1.0,2.0,3.0]];
//  assert_eq!(im.data[0].len(), 5)
//  assert_eq!(fm.data[0].len(), 3)
//  assert_eq!(fm.data[0][2], 3.0)
//
//  im.print();
//  fm.print();
// ```
#[macro_export]
macro_rules! mat {
    ( $t:ty : $( [ $( $x:expr ),+ ] ),* ) => {
        {
            let mut matrix: Numbers<$t> = Numbers::new();
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
            Numbers::<$x>::new()
        }
    };
}



#[cfg(test)]
mod tests_matrix {
    use crate::{Matrix,Numbers};

    #[test]
    fn test_new_i32(){
        let m = Numbers::<i32>::new();
        assert_eq!(m.data.len(),0);
    }

    #[test]
    fn test_new_f32(){
        let m = Numbers::<f32>::new();
        assert_eq!(m.data.len(),0);
    }

    #[test]
    fn test_macro_with_type(){
        let mut m = mat![f32];
        assert_eq!(m.data.len(), 0);
        m.data.push(vec![1.234,5.678]);
    }

    #[test]
    fn test_macro_with_values(){
        let mut m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        assert_eq!(m.data.len(), 3);
        assert_eq!(m.data[0].len(), 5);
        assert_eq!(m.data[2][2], 5);
        m.data.push(vec![5,6,7,8,9]);
    }

    #[test]
    #[should_panic]
    fn test_macro_invalid_len(){
        mat![i32: [1,23],[4,5,6]];
    }

    #[test]
    fn test_print(){
        let m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        m.print();
    }

    #[test]
    fn test_push(){
        Numbers::<i32>::new().push(vec![1,2]).push(vec![3,4]);
    }

    #[test]
    #[should_panic]
    fn test_push_unmatched_len(){
        Numbers::<i32>::new().push(vec![1,2,3]).push(vec![1]);
    }
}

#[cfg(test)]
mod tests_processor {
    use crate::{Matrix,Numbers,TensorProcessor};

    #[test]
    fn test_add(){
        let mut m = mat![i32: [1,2,3],[4,5,6],[7,8,9]];
        assert_eq!(m.data[0][1],2);
        assert_eq!(m.data[1][2],6);
        m.add(1);
        assert_eq!(m.data[0][1],3);
        assert_eq!(m.data[1][2],7);
    }
}