//! Tensors is a simple matrix manipulation library which supports
//! row and column matrix manipulations, scalar manipulation, 
//! mathematical manipulation, filtering and parsing mechanisms for
//! various type of data stored in vector of vector (Vec<Vec<T>>).
//!
//! すべての行列で可能な演算については値ないしは参照そのものを返却し、
//! 行列と引数によっては演算が定義されないものについてResult型を返却する。
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
//!  im.mul(3).print();
//!  fm.print();
//! ```
//!

pub trait Matrix<T> {
    fn print(&self);
    fn get(&self) -> &Vec<Vec<T>>;
    fn set(&mut self, data: Vec<Vec<T>>);
    fn row(&self, num: usize) -> Vec<T>;
    fn col(&self, num: usize) -> Vec<T>;
    fn is_square(&self) -> Result<&Self, &str>;
    fn rows(&self) -> usize;
    fn cols(&self) -> usize;
    fn row_replace(&mut self, src: usize, dst: usize) -> &mut Self;
    fn col_replace(&mut self, src: usize, dst: usize) -> &mut Self;
    fn transpose(&mut self) -> &mut Self;
    fn integrity_check(&self) -> Result<&Self, &str>;
    fn range_check(&self, row: usize, col: usize) -> Result<&Self, &str>;
    fn debug(&mut self) -> &mut Self;
}

pub struct Numbers<T> {
    data: Vec<Vec<T>>,
    current: usize,
    max: usize,
    debug: bool,
}

pub trait TensorProcessor<T> {
    fn fill_zero(&mut self) -> &mut Self;
    fn zero(&self) -> T;
    fn add(&mut self, val: T) -> &mut Self;
    fn sub(&mut self, val: T) -> &mut Self;
    fn mul(&mut self, val: T) -> &mut Self;
    fn div(&mut self, val: T) -> &mut Self;
    fn prod(&self, val: Numbers<T>) -> Result<Numbers<T>,&str>;
    fn determinant(&self) -> T;
    fn adjugate(&self, p: usize, q: usize) -> Result<Numbers<T>,&str>;
    fn is_regular(&self) -> Result<&Self, &str>;
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

        if self.data.len() != 0 {
            if self.data[0].len() != data.len(){
                panic!("Invalid vector length: {}, expected: {}",data.len(), self.data.len());
            }
        }
        if self.debug {
            println!("pushing {:?}", data);
        }

        self.data.push(data);
        self
    }
}


/// 行列インスンタンス初期化用マクロ
///
/// ```rust
/// use tensors::core::{Matrix,TensorProcessor,Numbers};
/// use tensors::mat;
///
///  let mut im = mat![i32];
///  let mut fm = mat![
///     f32: 
///         [1.0,2.0,3.0]
///  ];
///  assert_eq!(im.rows(), 0);
///  assert_eq!(fm.cols(), 3);
///  assert_eq!(fm.row(0)[2], 3.0);
///
///  im.print();
///  fm.print();
/// ```

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
                matrix.push(t_vec);
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


// [数値行列用] 基本メソッド群
impl<T> Matrix <T> for Numbers<T> 
where
    T: std::fmt::Debug + Copy
{
    /// データ表示関数
    ///
    fn print(&self){
        println!("{:?}",self.data)
    }

    /// 行列データへのアクセサ
    /// ベクトルベクトルを返却する。
    fn get(&self) -> &Vec<Vec<T>>{
        &self.data
    }

    /// 行列データのセッター
    /// 
    fn set(&mut self, m: Vec<Vec<T>>){
        if m.len() == 0 {
            panic!("argument has zero length");
        }
        if self.debug {
            println!("new data set: {:?}", m);
        }
        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                self.data[i][j] = m[i][j]
            }
        }
    }

    /// デバッガ
    /// デバッグフラグを有効化したインスタンスを返す。
    ///
    fn debug(&mut self) -> &mut Self {
        self.debug = true;
        println!("debuging for: {:?}",self.data);
        self
    }

    /// 行列データ整合性検証
    /// 長さ0もしくは長さの一致しないデータを検出した時点で強制終了
    ///
    fn integrity_check(&self) -> Result<&Self, &str> {
        if self.data.len() == 0 {
            return Err("zero matrix length detected");
        }
        for i in 0..self.data.len() {
            let len = self.data[0].len(); 
            if self.data[i].len() != len {
                println!("matrix corrupted at column {} (data: {:?}, length: {}, expected {})", i, self.data[i], self.data[i].len(), len);
                return Err("matrix corrupted");
            }
        }
        Ok(&self)
    }

    fn range_check(&self, row: usize, col: usize) -> Result<&Self, &str> {
        if row >= self.data.len() {
            return Err("row is out of order")
        } else if col >= self.data[0].len() {
            return Err("col is out of order")
        }
        Ok(&self)
    }

    /// 行抽出関数
    /// Vec<T>として行を返却
    ///
    fn row(&self, num: usize) -> Vec<T> {
        self.integrity_check().unwrap();
        if num >= self.data.len() {
            panic!("row number {} is out of order: must be less than {}",num, self.data.len());
        } else {
            let mut res :Vec<T> = Vec::new();
            for i in 0..self.data[num].len(){ 
                res.push(self.data[num][i]);
            }
            res
        }
    }

    /// 列抽出関数
    /// Vec<T>として列を返却
    ///
    fn col(&self, num: usize) -> Vec<T> {
        self.integrity_check().unwrap();
        if num >= self.data[0].len() {
            panic!("colum number {} is out of order: must be less than {}",num, self.data[0].len());
        }
        let mut res: Vec<T> = Vec::new();
        for i in 0..self.data.len() {
            res.push(self.data[i][num]);
        }
        res
    }

    /// 行数表示関数
    //
    fn rows(&self) -> usize {
        self.data.len()
    }

    //列数表示関数
    //
    fn cols(&self) -> usize {
        if self.data.len() == 0 {
            0
        } else {
            self.data[0].len()
        }
    }

    /// 正方行列判定
    ///
    /// 正方行列であるかどうかを判定し、Result型にくるんでオブジェクト参照を
    /// 返却する
    ///
    /// ```rust
    /// use tensors::core::{Matrix,TensorProcessor,Numbers};
    /// use tensors::mat;
    /// let m = mat![i32: [1,2,3], [2,3,4],[3,4,5]];
    /// m.is_square().unwrap();
    /// ```

    fn is_square(&self) -> Result<&Self, &str> {
        if self.data.len() != self.data[0].len() {
            Err("not a square matrix")
        } else {
            Ok(&self)
        }
    }

    //行置換操作
    //
    fn row_replace(&mut self, src: usize, dst: usize) -> &mut Self {

        if src >= self.data.len() {
            panic!("src address out of range {}", src);
        } else if dst >= self.data.len() {
            panic!("dst address out of range {}", dst);
        }


        let mut src_data = Vec::new();
        let mut dst_data = Vec::new();

        for i in 0..self.data[0].len() {
            src_data.push(self.data[src][i]);
            dst_data.push(self.data[dst][i]);
        }
        self.data[src] = dst_data;
        self.data[dst] = src_data;
        if self.debug {
            println!("matrix row replacement: {} with {}", src, dst );
            println!("{:?}",self.data);
        }
        self.integrity_check().unwrap();

        self
    }

    //列置換操作
    //
    fn col_replace(&mut self, src: usize, dst: usize) -> &mut Self {
        self.integrity_check().unwrap();

        if src >= self.data[0].len() {
            panic!("src address out of range {}", src);
        } else if dst >= self.data[0].len() {
            panic!("dst address out of range {}", dst);
        }

        for i in 0..self.data.len() {
            println!("col_rep try for row[{}]: {:?}", i, self.data[i] );
            let src_data = self.data[i][src];
            self.data[i][src] = self.data[i][dst];
            self.data[i][dst] = src_data;
        }
        if self.debug {
            println!("matrix column replacement: {} with {}", src, dst );
            println!("{:?}",self.data);
        }
        self

    }

	/// 転置
    /// 転置行列でデータを更新し、オブジェクト参照を返却する。
	///
	///```rust
	/// use tensors::core::{Matrix,TensorProcessor,Numbers};
	/// use tensors::mat;
	///        let mut m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
	///        assert_eq!(m.rows(),3);
	///        assert_eq!(m.cols(),5);
	///        let res = mat![
	///            i32:
	///                [1,2,3],
	///                [2,3,4],
	///                [3,4,5],
	///                [4,5,6],
	///                [5,6,7]
	///        ];
	///        m.transpose();
	///        assert_eq!(m.rows(),5);
	///        assert_eq!(m.cols(),3);
	///        for i in 0..m.rows(){
	///            for j in 0..m.cols(){
	///                assert_eq!(m.get()[i][j], res.get()[i][j]);
	///            }
	///        }
	///
	///```
	

    fn transpose (&mut self) -> &mut Self {
        self.integrity_check().unwrap();

        let mut res: Vec<Vec<T>> = Vec::new();
        let cols = self.data[0].len();
        for i in 0..cols {
            res.push(Vec::new());
            let col = self.col(i);
            for j in 0..self.data.len() {
                res[i].push(col[j]);
            }
            if self.debug {
                println!("res[{}]: {:?}",i, res[i]);
            }
        }
        while self.data.len() != 0 {
            self.data.pop();
        }
        for i in 0..cols {
            self.data.push(Vec::new());
            for j in 0..res[0].len() {
                self.data[i].push(res[i][j]);
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
    //int系の型のみサポート
    //
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
    T: Copy + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign + std::ops::DivAssign + std::ops::Sub<Output=T> + std::ops::Mul<Output=T> + std::fmt::Display + std::fmt::Debug + PartialEq
{

    /// ゼロ値取得関数
    fn zero(&self) -> T {
        self.data[0][0] - self.data[0][0]
    }

    /// ゼロ充填
    /// 型Tにおけるゼロ値で全データを更新
    fn fill_zero(&mut self) -> &mut Self {
        let zero = self.zero();
        for i in 0..self.data.len(){
            for j in 0..self.data[0].len(){
                self.data[i][j] = zero;
            }
        }
        self
    }

    /// スカラー加算
    ///
    fn add(&mut self, val: T) -> &mut Self {
        self.integrity_check().unwrap();
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

    /// スカラー減算
    ///
    fn sub(&mut self, val: T) -> &mut Self {
        self.integrity_check().unwrap();
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

    /// スカラー乗算
    ///
    fn mul(&mut self, val: T) -> &mut Self {
        self.integrity_check().unwrap();
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

    /// スカラー除算
    /// (i32では端数切捨て)
    fn div(&mut self, val: T) -> &mut Self {
        self.integrity_check().unwrap();
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

    /// 行列乗算 
    /// 引数として与えられた同一型の行列インスタンスを用いて 
    /// self.data を元とする行列の積を計算し、 データ更新後の
    /// オブジェクト参照を返却する。
    ///
    /// ```rust
    /// use tensors::core::{Matrix,TensorProcessor,Numbers};
    /// use tensors::mat;
    ///
    /// let mut m = mat![
    ///     i32:
    ///         [1,2,3],
    ///         [4,5,7]
    /// ];
    /// let n = mat![
    ///     i32:
    ///         [1,3],
    ///         [5,7],
    ///         [10,10]
    /// ];
    /// let res = mat![
    ///     i32:
    ///         [41,47],
    ///         [99,117]
    /// ];
    ///
    /// let p = m.prod(n).unwrap();
    ///
    /// assert_eq!(p.rows(),2);
    /// assert_eq!(p.cols(),2);
    /// for i in 0..p.rows() {
    ///     for j in 0..p.cols() {
    ///         assert_eq!(p.get()[i][j], res.get()[i][j]);
    ///     }
    /// }
    /// ```

    fn prod (&self, m: Numbers<T>) -> Result<Numbers<T>,&str> {
        self.integrity_check().unwrap();
        m.integrity_check().unwrap();

        if self.data.len() != m.data[0].len() {
            return Err("row length not matched to the col length of argument")
        } else if self.data[0].len() != m.data.len() {
            return Err("col length not matched to the row length of argument")
        }
        //解行列のサイズ
        let res_length: usize = self.data.len();

        let mut res: Numbers<T> = mat![T];

        //解行列の計算
        for i in 0..self.data.len() {
            for j in 0..m.data.len() {
                for seq in 0..res_length {
                    //println!("res[{}][{}] += self.data[{}][{}] + m.data[{}][{}] = {}", i, seq, seq, j, j, seq, self.data[seq][j] * m.data[j][seq]);
                    if i >= res.rows() {
                        res.data.push(Vec::new());
                    }
                    if seq >= res.data[i].len() {
                        res.data[i].push(self.zero());
                    }
                    if self.debug {
                        println!("i: {}, j: {}, seq: {}, where res.data is {:?}",i,j,seq, res.data);
                    }
                    res.data[i][seq] += self.data[i][j] * m.data[j][seq];
                }
            }
        }

        //println!("res: {:?}",res);
        Ok(res)
    }

    /// 余因子行列取得関数
    /// 行p, 列q についての余因子行列を取得し、Result型に
    /// くるんだ Numbers<T>型として返却する。オブジェクト本体の
    /// 変更は行わないイミュータブルな実装。
    ///
    /// ```rust
    /// use tensors::core::{Matrix,TensorProcessor,Numbers};
    /// use tensors::mat;
    ///
    /// let m = mat![
    ///     i32:
    ///         [1,2,3],
    ///         [4,5,7],
    ///         [9,17,13289]
    /// ];
    /// let res = mat![
    ///     i32:
    ///         [1,-3],
    ///         [-9,13289]
    /// ];
    /// let adj = m.adjugate(1,1).unwrap();
    /// assert_eq!(adj.rows(), 2);
    /// assert_eq!(adj.cols(), 2);
    /// for i in 0..adj.rows() {
    ///     for j in 0..adj.cols() {
    ///         assert_eq!(adj.get()[i][j], res.get()[i][j]);
    ///     }
    /// }
    /// ```
    ///

    fn adjugate(&self, p: usize, q: usize) -> Result<Numbers<T>,&str> {
        self.integrity_check().unwrap();
        self.range_check(p,q).unwrap();
        let mut res = Numbers::<T>::new();
        for i in 0..self.data.len() {
            let mut v = Vec::new();
            if i != p {
                for j in 0..self.data[0].len() {
                    if j != q {
                         v.push(self.data[i][j]);
                    }
                }
                res.push(v);
            }
        }
        for i in 0..res.data.len() {
                for j in 0..res.data[0].len() {
                    match (i + j) % 2 {
                        1 => {
                            let datum = res.data[i][j];
                            res.data[i][j] = datum - datum - datum;
                        },
                        _ => ()
                    }
                }
        }
        Ok(res)
    }


    /// 行列式
    /// 行列式を計算し、型Tで結果を返却する
    ///
    /// ```rust
    /// use tensors::core::{Matrix,TensorProcessor,Numbers};
    /// use tensors::mat;
    ///
    /// let m = mat![
    ///     i32:
    ///         [2,3,4,1],
    ///         [1,2,3,4],
    ///         [4,1,2,3],
    ///         [3,2,1,4]
    /// ];
    /// assert_eq!(m.determinant(),80);
    /// ```
    ///

    fn determinant(&self) -> T {
        self.integrity_check().unwrap();
        self.is_square().unwrap();

        if self.data.len() == 1 {
            self.data[0][0]
        } else if self.data.len() == 2 {
            self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
        } else {
            let mut res = self.data[0][0] - self.data[0][0];
            for i in 0..self.data.len() {
                let adj = self.adjugate(i,0).unwrap();
                if i % 2 == 0 {
                    res += self.data[i][0] * adj.determinant();
                } else {
                    res -= self.data[i][0] * adj.determinant();
                }
            }
            res
        }
    }


    /// 正則行列判定
    /// 正則行列であるかどうか調べ、Result型にくるんで
    /// オブジェクト参照を返却する。
    ///
    /// ```rust
    /// use tensors::{Matrix,TensorProcessor,Numbers};
    /// use tensors::mat;
    ///
    /// let r = mat![i32:[1,2],[3,4]];
    /// r.is_regular().unwrap();
    /// ```
    fn is_regular(&self) -> Result<&Self, &str> {
        if self.determinant() != self.zero() {
            Ok(self)
        } else {
            Err("the matrix is not a regular matrix")
        }
    }
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
        //m.data.push(vec![1.234,5.678]);
    }

    #[test]
    fn test_macro_with_values(){
        let mut m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        assert_eq!(m.data.len(), 3);
        assert_eq!(m.data[0].len(), 5);
        assert_eq!(m.data[2][2], 5);
        //m.data.push(vec![5,6,7,8,9]);
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
    #[should_panic]
    fn test_is_square_error(){
        let m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        m.is_square().unwrap();
    }

    #[test]
    fn test_integrity(){
        let m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        m.integrity_check().unwrap();
    }

    #[test]
    #[should_panic]
    //ゼロ値でパニック
    fn test_integrity_error_zero(){
        let mut m = Numbers::<i32>::new();
        m.integrity_check().unwrap();
    }

    #[test]
    #[should_panic]
    //行列でないデータではパニック
    fn test_integrity_error_corrupted(){
        let mut m = Numbers::<i32>::new();
        //privateフィールドに手動でベクトル代入
        m.data.push(vec![1,2,3]);
        m.data.push(vec![1,2,3,4,5]);
        m.integrity_check().unwrap();
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

    #[test]
    fn test_row(){
        let m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        assert_eq!(m.row(1)[2],4);
        assert_eq!(m.row(0)[1],m.row(1)[0]);
    }

    #[test]
    #[should_panic]
    fn test_row_error(){
        let m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        m.row(5);
    }

    #[test]
    fn test_col(){
        let m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        assert_eq!(m.col(1)[1],3);
        assert_eq!(m.col(1)[2],m.col(3)[0]);
    }

    #[test]
    fn test_rows(){
        let m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        assert_eq!(m.rows(),3);
    }

    #[test]
    fn test_cols(){
        let m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        assert_eq!(m.cols(),5);
    }

    #[test]
    #[should_panic]
    fn test_col_error(){
        let m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        m.row(4);
    }

    #[test]
    fn test_row_replace(){
        let mut a = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        let p0 = vec![1,2,3,4,5];
        let p2 = vec![3,4,5,6,7];
        for i in 0..a.data[0].len(){
            assert_eq!(a.data[0][i], p0[i]);
            assert_eq!(a.data[2][i], p2[i]);
        }

        a.row_replace(0,2);
        for i in 0..a.data[0].len(){
            assert_eq!(a.data[0][i], p2[i]);
            assert_eq!(a.data[2][i], p0[i]);
        }
    }

    #[test]
    fn test_col_replace(){
        let mut m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        let p0 = vec![1,2,3];
        let p2 = vec![3,4,5];
        for i in 0..m.data.len(){
            assert_eq!(m.data[i][0], p0[i]);
            assert_eq!(m.data[i][2], p2[i]);
        }

        m.col_replace(0,2);
        for i in 0..m.data.len(){
            assert_eq!(m.data[i][0], p2[i]);
            assert_eq!(m.data[i][2], p0[i]);
        }
    }

    #[test]
    fn test_transpose(){
        let mut m = mat![i32: [1,2,3], [3,4,5],[5,6,7]];
        let res = mat![
            i32:
                [1,3,5],
                [2,4,6],
                [3,5,7]
        ];
        m.transpose();
        for i in 0..m.data.len(){
            for j in 0..m.data[0].len(){
                assert_eq!(m.data[i][j], res.data[i][j]);
            }
        }

    }
}

#[cfg(test)]
mod tests_processor {
    use crate::{Numbers,TensorProcessor};

    #[test]
    fn test_add(){
        let mut m = mat![i32: [1,2,3],[4,5,6],[7,8,9]];
        let res = mat![i32: [2,3,4],[5,6,7],[8,9,10]];
        m.add(1);
        for i in 0..m.data.len() {
            for j in 0..m.data[0].len() {
                assert_eq!(m.data[i][j], res.data[i][j]);
            }
        }
    }

    #[test]
    fn test_sub(){
        let mut m = mat![i32: [1,2,3],[4,5,6],[7,8,9]];
        let res = mat![i32: [-4,-3,-2],[-1,0,1],[2,3,4]];
        m.sub(5);
        for i in 0..m.data.len() {
            for j in 0..m.data[0].len() {
                assert_eq!(m.data[i][j], res.data[i][j]);
            }
        }
    }

    #[test]
    fn test_mul(){
        let mut m = mat![i32: [1,2,3],[4,5,6],[7,8,9]];
        let res = mat![i32: [2,4,6],[8,10,12],[14,16,18]];
        m.mul(2);
        for i in 0..m.data.len() {
            for j in 0..m.data[0].len() {
                assert_eq!(m.data[i][j], res.data[i][j]);
            }
        }

    }

    #[test]
    fn test_div(){
        let mut m = mat![i32: [2,4,6],[8,10,12],[14,16,18]];
        let res = mat![i32: [1,2,3],[4,5,6],[7,8,9]];
        m.div(2);
        for i in 0..m.data.len() {
            for j in 0..m.data[0].len() {
                assert_eq!(m.data[i][j], res.data[i][j]);
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_prod_error_unmatched(){
        let mut m = mat![
            i32:
                [1,2,3],
                [4,5,7]
        ];
        let n = mat![
            i32:
                [1,0],
                [0,1]
        ];
        m.prod(n).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_prod_error_zero(){
        let mut m = mat![
            i32:
                [1,2,3],
                [4,5,7]
        ];
        let n = Numbers::<i32>::new();
        m.prod(n).unwrap();
    }

    #[test]
    fn test_determinant_2x2(){
        let m = mat![
            i32:
                [1,2],
                [-3,-4]
        ];
        assert_eq!(m.determinant(),2);
    }

    #[test]
    fn test_determinant_1x1(){
        let m = mat![i32: [5]];
        assert_eq!(m.determinant(),5);
    }

    #[test]
    fn test_determinant_3x3(){
        let m = mat![
            i32:
                [1,2,3],
                [0,1,1],
                [1,1,5]
        ];
        assert_eq!(m.determinant(),3);
    }

    #[test]
    fn test_determinant_3x3_float(){
        let m = mat![
            f32:
                [1.0,2.0,3.0],
                [0.0,1.0,1.0],
                [1.0,1.0,5.0]
        ];
        assert_eq!(m.determinant(),3.0);
    }

    #[test]
    #[should_panic]
    fn test_determinant_3x2_error(){
        let m = mat![
            i32:
                [1,2],
                [0,1],
                [1,1]
        ];
        m.determinant();
    }

}
