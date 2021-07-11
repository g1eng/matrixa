//! Tensors is a simple matrix manipulation library which supports
//! row and column matrix manipulations, scalar manipulation,
//! mathematical manipulation, filtering and parsing mechanisms for
//! various type of data stored in vector of vector (Vec<Vec<T>>).
//!
//! すべての行列で可能な演算については値ないしは参照そのものを返却し、
//! 行列と引数によっては演算が定義されないものについてResult型を返却する。
//!
//! ```rust
//! use tensors::core::{List,Matrix};
//! use tensors::mat;
//!
//!  let mut im = Matrix::<i32>::new();
//!  let mut fm = mat![f32: [1.0,2.0,3.0]];
//!  im.push(vec![1,2,3,4,5])
//!    .unwrap()
//!    .push(vec![5,6,7,8,9])
//!    .unwrap();
//!  fm.debug()
//!    .push(vec![1.23,4.56,7.89])
//!    .unwrap();
//!  im.add(1).print();
//!  im.mul(3).print();
//!  fm.print();
//! ```
//!

use std::ops::{Add, Mul, Sub};

pub trait List<T> {
    fn dump(&self) -> &Vec<Vec<T>>;
    fn row(&self, num: usize) -> Vec<T>;
    fn col(&self, num: usize) -> Vec<T>;
    fn is_square(&self) -> Result<&Self, &str>;
    fn rows(&self) -> usize;
    fn cols(&self) -> usize;
    fn row_replace(&mut self, src: usize, dst: usize) -> Result<&mut Self, &str>;
    fn col_replace(&mut self, src: usize, dst: usize) -> Result<&mut Self, &str>;
    fn transpose(&mut self) -> &mut Self;
    fn integrity_check(&self) -> Result<&Self, &str>;
    fn row_check(&self, row: usize) -> Result<&Self, &str>;
    fn col_check(&self, col: usize) -> Result<&Self, &str>;
    fn range_check(&self, row: usize, col: usize) -> Result<&Self, &str>;
}

pub struct Matrix<T> {
    data: Vec<Vec<T>>,
    current: usize,
    max: usize,
    debug: bool,
}

impl<T: std::fmt::Debug> Matrix<T> {
    /// 行列生成
    /// 新規の行列インスタンスを生成し、空行列として返却する
    ///
    pub fn new() -> Self {
        let v: Vec<Vec<T>> = Vec::new();
        Matrix {
            data: v,
            current: 0,
            max: 0,
            debug: false,
        }
    }

    /// データ表示関数
    ///
    pub fn print(&self) {
        println!("{:?}", self.data)
    }

    /// デバッガ
    /// デバッグフラグを有効化したインスタンスを返す。
    ///
    pub fn debug(&mut self) -> &mut Self {
        self.debug = true;
        println!("debugging for: {:?}", self.data);
        self
    }

    /// データ追加
    /// データ末尾にVec<T>型で指定した新規列を追加。
    /// マクロ実装の関係上、pushメソッドについてはMatrix型に直に記述している。
    ///
    pub fn push(&mut self, data: Vec<T>) -> Result<&mut Self, &str> {
        self.max += 1;

        if self.data.len() != 0 {
            if self.data[0].len() != data.len() {
                //println!("Invalid vector length: {}, expected: {}",data.len(), self.data.len());
                return Err("Invalid vector length");
            }
        }
        /*
        if self.debug {
            println!("pushing {:?}", data);
        }*/

        self.data.push(data);
        Ok(self)
    }
}

/// 行列インスンタンス初期化用マクロ
///
/// ```rust
/// use tensors::core::{List,Matrix};
/// use tensors::mat;
///
///  let mut im = mat![i32];
///  let fm = mat![
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
            let mut matrix: Matrix<$t> = Matrix::new();
            let mut vec_len = 0;
            $(
                let mut t_vec = Vec::new();
                $(
                    t_vec.push($x);
                )*
                if vec_len == 0 {
                    vec_len = t_vec.len();
                }
                if vec_len != t_vec.len() {
                    panic!("invalid vector length for {:?}!", t_vec)
                }
                matrix.push(t_vec).expect("failed to push new vector into the matrix");
            )*
            matrix
        }
    };
    ( $x:ty ) => {
        {
            Matrix::<$x>::new()
        }
    };
}

/// 行列の加算
/// 行列の要素ごとの加算を行い、新規インスタンスとして結果を返却する。
/// 行および列の数が一致しない行列が指定された場合はパニックする。
///
/// ```rust
/// use tensors::core::{List,Matrix};
/// use tensors::mat;
///
/// let m = mat![i32:[1,2],[3,4]];
/// let n = mat![i32:[-5,6],[7,-8]];
/// let ans = mat![i32:[-4,8],[10,-4]];
/// let res = m + n;
/// for i in 0..ans.rows() {
///     for j in 0..ans.cols() {
///         assert_eq!(res.row(i)[j], ans.row(i)[j]);
///     }
/// }
/// ```
///

impl<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T>> Add for Matrix<T>
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::fmt::Debug + From<u8>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        if self.data.len() == 0 {
            panic!("zero length origin for addition");
        } else if other.data.len() == 0 {
            panic!("zero length company for addition");
        } else if self.data.len() != other.data.len() {
            panic!(
                "row number not matched for addition: {}, {}",
                self.data.len(),
                other.data.len()
            );
        } else if self.data[0].len() != other.data[0].len() {
            panic!("row number not matched for addition");
        }

        let zero = T::from(0x0u8);
        let mut res = Self::new();

        for i in 0..self.data.len() {
            if i >= res.data.len() {
                res.data.push(Vec::new());
            }
            for j in 0..self.data[0].len() {
                res.data[i].push(zero);
                res.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        res
    }
}

/// 行列の減算
/// 行列の要素ごとの減算を行い、新規インスタンスとして結果を返却する。
/// 行および列の数が一致しない行列が指定された場合はパニックする。
///
/// ```rust
/// use tensors::core::{List,Matrix};
/// use tensors::mat;
///
/// let m = mat![i32:[1,2],[3,4]];
/// let n = mat![i32:[-5,6],[7,-8]];
/// let ans = mat![i32:[6,-4],[-4,12]];
/// let res = m - n;
/// for i in 0..ans.rows() {
///     for j in 0..ans.cols() {
///         assert_eq!(res.row(i)[j], ans.row(i)[j]);
///     }
/// }
/// ```
///

impl<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T>> Sub for Matrix<T>
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::fmt::Debug + From<u8>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        if self.data.len() == 0 {
            panic!("zero length origin for subtract");
        } else if other.data.len() == 0 {
            panic!("zero length company for subtract");
        } else if self.data.len() != other.data.len() {
            panic!(
                "row number not matched for subtract: {}, {}",
                self.data.len(),
                other.data.len()
            );
        } else if self.data[0].len() != other.data[0].len() {
            panic!("row number not matched for subtract");
        }

        let zero = T::from(0x0u8);
        let mut res = Self::new();

        for i in 0..self.data.len() {
            if i >= res.data.len() {
                res.data.push(Vec::new());
            }
            for j in 0..self.data[0].len() {
                res.data[i].push(zero);
                res.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        res
    }
}

/// 積
///
/// 行列の積の計算を行い、新規インスタンスとして結果を返却する。
/// 積の定義されない(不正な行・列の)組み合わせの行列が指定された場合は
/// パニックする。
///
/// ```rust
/// use tensors::core::{List,Matrix};
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
/// let p = m * n;
///
/// assert_eq!(p.rows(),2);
/// assert_eq!(p.cols(),2);
/// for i in 0..p.rows() {
///     for j in 0..p.cols() {
///         assert_eq!(p.dump()[i][j], res.dump()[i][j]);
///     }
/// }
/// ```
///

impl<
        T: std::ops::Mul<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Add<Output = T>
            + std::fmt::Debug,
    > Mul for Matrix<T>
where
    T: Copy
        + std::ops::Mul<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Add<Output = T>
        + std::fmt::Debug
        + From<u8>,
{
    type Output = Self;

    fn mul(self, m: Self) -> Self {
        self.integrity_check().unwrap();
        m.integrity_check().unwrap();

        if self.data.len() != m.data[0].len() {
            panic!(
                "row length of origin {} is not matched to the column length of the company {}",
                self.data.len(),
                m.data[0].len()
            )
        } else if self.data[0].len() != m.data.len() {
            panic!(
                "column length of origin {} is not matched to the row length of the company {}",
                self.data[0].len(),
                m.data.len()
            )
        }
        //解行列のサイズ
        let res_length: usize = self.data.len();

        let mut res = Self::new();
        let zero = T::from(0x0u8);

        //解行列の計算
        for i in 0..self.data.len() {
            for j in 0..m.data.len() {
                for seq in 0..res_length {
                    if i >= res.data.len() {
                        res.data.push(Vec::new());
                    }
                    if seq >= res.data[i].len() {
                        res.data[i].push(zero);
                    }
                    if self.debug {
                        println!(
                            "i: {}, j: {}, seq: {}, where res.data is {:?}",
                            i, j, seq, res.data
                        );
                    }
                    res.data[i][seq] = res.data[i][seq] + self.data[i][j] * m.data[j][seq];
                }
            }
        }

        res
    }
}

/// [行列一般] 基本メソッド群
/// 数値行列および文字行列のいずれにも対応したメソッドを定義。
///
impl<T> List<T> for Matrix<T>
where
    T: std::fmt::Debug + Copy,
{
    /// 行列データへのアクセサ
    /// ベクトルベクトルを返却する。
    fn dump(&self) -> &Vec<Vec<T>> {
        &self.data
    }

    /// 行抽出関数
    /// Vec<T>として行を返却
    ///
    fn row(&self, num: usize) -> Vec<T> {
        self.integrity_check().unwrap();
        if num >= self.data.len() {
            panic!(
                "row number {} is out of order: must be less than {}",
                num,
                self.data.len()
            );
        } else {
            let mut res: Vec<T> = Vec::new();
            for i in 0..self.data[num].len() {
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
        let mut res: Vec<T> = Vec::new();
        for i in 0..self.data.len() {
            res.push(self.data[i][num]);
        }
        res
    }

    /// 正方行列判定
    ///
    /// 正方行列であるかどうかを判定し、Result型に格納したオブジェクト参照を
    /// 返却する
    ///
    /// ```rust
    /// use tensors::core::{List,Matrix};
    /// use tensors::mat;
    ///
    /// let m = mat![i32: [1,2,3], [2,3,4],[3,4,5]];
    /// m.is_square().unwrap();
    /// ```
    ///
    fn is_square(&self) -> Result<&Self, &str> {
        if self.data.len() != self.data[0].len() {
            Err("not a square matrix")
        } else {
            Ok(&self)
        }
    }

    /// 行数表示関数
    ///
    fn rows(&self) -> usize {
        self.data.len()
    }

    /// 列数表示関数
    ///
    fn cols(&self) -> usize {
        if self.data.len() == 0 {
            0
        } else {
            self.data[0].len()
        }
    }

    /// 行置換操作
    ///
    fn row_replace(&mut self, src: usize, dst: usize) -> Result<&mut Self, &str> {
        self.integrity_check()
            .expect("data corrupted before row replacement");

        self.row_check(src)
            .expect("src row is out of order for the replacement")
            .row_check(dst)
            .expect("dst row is out of order for the replacement");

        let mut src_data = Vec::new();
        let mut dst_data = Vec::new();

        for i in 0..self.data[0].len() {
            src_data.push(self.data[src][i]);
            dst_data.push(self.data[dst][i]);
        }
        self.data[src] = dst_data;
        self.data[dst] = src_data;
        if self.debug {
            println!("matrix row replacement: {} with {}", src, dst);
            println!("{:?}", self.data);
        }

        self.integrity_check()
            .expect("data corrupted after row replacement");

        Ok(self)
    }

    /// 列置換操作
    ///
    fn col_replace(&mut self, src: usize, dst: usize) -> Result<&mut Self, &str> {
        self.integrity_check()
            .expect("data corrupted before row replacement");

        self.row_check(src)
            .expect("src column is out of order for the replacement")
            .row_check(dst)
            .expect("dst column is out of order for the replacement");

        for i in 0..self.data.len() {
            println!("col_rep try for row[{}]: {:?}", i, self.data[i]);
            let src_data = self.data[i][src];
            self.data[i][src] = self.data[i][dst];
            self.data[i][dst] = src_data;
        }
        if self.debug {
            println!("matrix column replacement: {} with {}", src, dst);
            println!("{:?}", self.data);
        }
        self.integrity_check()
            .expect("data corrupted after row replacement");

        Ok(self)
    }

    /// 転置
    /// 転置行列でデータを更新し、オブジェクト参照を返却する。
    ///
    ///```rust
    /// use tensors::core::{List,Matrix};
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
    ///                assert_eq!(m.dump()[i][j], res.dump()[i][j]);
    ///            }
    ///        }
    ///
    ///```

    fn transpose(&mut self) -> &mut Self {
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
                println!("res[{}]: {:?}", i, res[i]);
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
            println!("{:?}", self.data);
        }
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
                println!(
                    "matrix corrupted at column {} (data: {:?}, length: {}, expected {})",
                    i,
                    self.data[i],
                    self.data[i].len(),
                    len
                );
                return Err("matrix corrupted");
            }
        }
        Ok(&self)
    }

    /// 行の存在性検証
    /// 行の値をusizeで指定し、行列の高さに収まるかどうかを検証。
    /// 結果をResult型にオブジェクト参照を格納して返却
    ///
    fn row_check(&self, row: usize) -> Result<&Self, &str> {
        match row < self.data.len() {
            true => Ok(&self),
            false => Err("row is out of order"),
        }
    }

    /// 列の存在性検証
    /// 列の値をusizeで指定し、行列の幅に収まるかどうかを検証。
    /// 結果をResult型にオブジェクト参照を格納して返却
    ///
    fn col_check(&self, row: usize) -> Result<&Self, &str> {
        match row < self.data.len() {
            true => Ok(&self),
            false => Err("column is out of order"),
        }
    }

    /// 行・列の存在性検証
    /// 行および列の値をusizeで指定し、行列の幅・高さに収まるかどうかを検証。
    /// 結果をResult型にオブジェクト参照を格納して返却
    ///
    fn range_check(&self, row: usize, col: usize) -> Result<&Self, &str> {
        self.row_check(row)?.col_check(col)
    }
}

/// イテレータ実装
/// Matrix はIterator を実装しており、for文等での数え上げに使える。
///
/// ```rust
/// use tensors::core::{List,Matrix};
/// use tensors::mat;
///
/// let m = mat![i32: [1,2,3],[4,5,6]];
/// let v = vec![vec![1,2,3],vec![4,5,6]];
/// let mut i = 0;
/// for row in m {
///     let mut j = 0;
///     for datum in row {
///         assert_eq!(datum, v[i][j]);
///         j += 1;
///     }
///     i += 1;
/// }
/// ```
///
impl<T> Iterator for Matrix<T>
where
    T: Clone,
{
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Vec<T>> {
        self.current += 1;
        if self.current - 1 < self.max {
            let data = &self.data[self.current - 1];
            Some(data.to_vec())
        } else {
            None
        }
    }
}

/// 数値計算用メソッド群
///
impl<T> Matrix<T>
where
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::fmt::Display
        + std::fmt::Debug
        + PartialEq
        + From<u8>,
{
    /// ゼロ値取得関数
    ///
    fn zero(&self) -> T {
        if self.data.len() == 0 {
            panic!("no zero value defined for a blank matrix")
        }
        T::from(0x0u8)
    }

    /// ゼロ充填
    /// 型Tにおけるゼロ値で全データを更新
    ///
    fn fill_zero(&mut self) -> &mut Self {
        let zero = T::from(0x0u8);
        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                self.data[i][j] = zero;
            }
        }
        self
    }

    /// 数値行列用サイズ変更
    ///
    /// usize型で行x列サイズを指定し、selfのデータサイズを変更する。
    /// サイズが縮小する行・列についてはデータを破棄し、
    /// サイズが拡大する行・列についてはゼロ値で充填する。
    /// 起点となる行列は空行列であってはならない。(ゼロ値での補完が失敗する)
    ///
    /// ```rust
    /// use tensors::core::{List,Matrix};
    /// use tensors::mat;
    ///
    /// let mut m = mat![i32: [1]];
    /// let mut is_first = true;
    ///
    /// m.resize(5,5).unwrap();
    /// assert_eq!(m.rows(),5);
    /// assert_eq!(m.cols(),5);
    ///
    /// for v in m {
    ///     for d in v {
    ///         if is_first {
    ///             assert_eq!(d,1);
    ///             is_first = false;
    ///         } else {
    ///             assert_eq!(d,0);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    pub fn resize(&mut self, row: usize, col: usize) -> Result<&mut Self, &str> {
        if row == 0 {
            return Err("resize error: row length must not be zero");
        } else if col == 0 {
            return Err("resize error: column length must not be zero");
        }
        if self.debug {
            println!("resizing matrix to {} x {}...", row, col);
        }
        let zero = self.zero();
        while self.rows() < row {
            self.data.push(Vec::new());
        }
        while self.rows() > row {
            self.data.pop();
        }
        for i in 0..self.rows() {
            while self.data[i].len() < col {
                self.data[i].push(zero);
            }
            while self.data[i].len() > col {
                self.data.pop();
            }
        }
        Ok(self)
    }

    /// 行列セッタ
    ///
    /// Vec<Vec<T>>への参照として行列データをミュータブルにセットする関数。
    ///
    /// ```rust
    /// use tensors::core::{List,Matrix};
    /// use tensors::mat;
    ///
    /// let mut m = mat![i32:[1]];
    /// let v = vec![
    ///     vec![1,2,3],
    ///     vec![1,2,3],
    /// ];
    ///
    /// m.set(&v);
    /// for i in 0..1 {
    ///     for j in 0..2 {
    ///         assert_eq!(m.dump()[i][j],v[i][j]);
    ///     }
    /// }
    /// m.print();
    /// ```
    ///
    pub fn set(&mut self, m: &Vec<Vec<T>>) {
        if m.len() == 0 {
            panic!("argument has zero length");
        }
        if self.debug {
            println!("new data set: {:?}", m);
        }
        self.resize(m.len(), m[0].len())
            .expect("failed to resize the matrix");
        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                self.data[i][j] = m[i][j]
            }
        }
    }

    /// スカラー加算
    ///
    pub fn add(&mut self, val: T) -> &mut Self {
        self.integrity_check().unwrap();
        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                self.data[i][j] = self.data[i][j] + val;
            }
        }
        if self.debug {
            println!("add {} foreach", val);
            println!("{:?}", self.data);
        }
        self
    }

    /// スカラー減算
    ///
    pub fn sub(&mut self, val: T) -> &mut Self {
        self.integrity_check().unwrap();
        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                self.data[i][j] = self.data[i][j] - val;
            }
        }
        if self.debug {
            println!("sub {} foreach", val);
            println!("{:?}", self.data);
        }
        self
    }

    /// スカラー乗算
    ///
    pub fn mul(&mut self, val: T) -> &mut Self {
        self.integrity_check().unwrap();
        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                self.data[i][j] = self.data[i][j] * val;
            }
        }
        if self.debug {
            println!("mul {} foreach", val);
            println!("{:?}", self.data);
        }
        self
    }

    /// スカラー除算
    /// (i32では端数切捨て)
    ///
    pub fn div(&mut self, val: T) -> &mut Self {
        self.integrity_check().unwrap();
        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                self.data[i][j] = self.data[i][j] / val;
            }
        }
        if self.debug {
            println!("divide {} foreach", val);
            println!("{:?}", self.data);
        }
        self
    }

    /// 複製
    /// selfと同一のデータを有する新規インスタンスを生成する。
    /// Matrix構造体のデータはVec<VeC<T>>であるため、Copyを実装しない。
    /// そのためselfのデータを利用して各種操作を行うためには、所有権の移転
    /// が発生しない新規インスタンスを生成するのが便利だ。
    ///
    /// ```rust
    /// use tensors::core::{List,Matrix};
    /// use tensors::mat;
    ///
    /// let x = mat![i32:[1,2,3],[1,5,6]];
    /// let y = x.get();
    ///
    /// for i in 0..y.rows() {
    ///     for j in 0..y.cols() {
    ///         assert_eq!(y.row(i)[j], x.row(i)[j]);
    ///     }
    /// }
    ///
    /// // xはxを所有しており、この時点でも自身のデータにアクセスできる
    /// x.print();
    ///
    /// ```
    ///
    pub fn get(&self) -> Matrix<T> {
        let mut res = mat![T];
        let zero = self.zero();

        for i in 0..self.rows() {
            if i >= res.rows() {
                res.data.push(Vec::new());
            }
            for j in 0..self.cols() {
                res.data[i].push(zero);
                res.data[i][j] = self.data[i][j];
            }
        }
        res
    }

    /// 積
    /// 引数として与えられた同一型の行列インスタンスを用いて
    /// self.data を元とする行列の積を計算し、Result型に格納した
    /// 新規インスタンスを返却する。
    /// std::ops::Mul を実装した * 演算子を用いて同様の計算が可能だが、
    /// * 演算子を用いる場合は演算不能な場合にパニックする。prod関数
    /// を用いる場合はResult型を通じたエラー制御が可能。
    ///
    /// ```rust
    /// use tensors::core::{List,Matrix};
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
    ///         assert_eq!(p.dump()[i][j], res.dump()[i][j]);
    ///     }
    /// }
    /// ```
    pub fn prod(&self, m: Matrix<T>) -> Result<Self, &str> {
        self.integrity_check().unwrap();
        m.integrity_check().unwrap();

        if self.data.len() != m.data[0].len() {
            return Err("row length not matched to the col length of argument");
        } else if self.data[0].len() != m.data.len() {
            return Err("col length not matched to the row length of argument");
        }

        Ok(self.get() * m)
    }

    /// アダマール積
    /// 行列の要素ごとの積(element-wize or pointwise product)を求め、
    /// Result型に格納した新規インスタンスを返却する。
    ///
    /// ```rust
    /// use tensors::core::{List,Matrix};
    /// use tensors::mat;
    /// let m = mat![
    ///     i32:
    ///         [1,2],
    ///         [4,5]
    /// ];
    ///
    /// let n = mat![
    ///     i32:
    ///         [1,-2],
    ///         [-3,4]
    /// ];
    /// let res = mat![
    ///     i32:
    ///         [1,-4],
    ///         [-12,20]
    /// ];
    /// let p = m.hadamard(n).unwrap();
    /// for i in 0..p.rows() {
    ///     for j in 0..p.cols() {
    ///         assert_eq!(p.dump()[i][j], res.dump()[i][j]);
    ///     }
    /// }
    /// ```
    ///
    pub fn hadamard(&self, m: Matrix<T>) -> Result<Self, &str> {
        self.integrity_check()
            .expect("origin matrix corrupted for hadamard product");
        m.integrity_check()
            .expect("origin matrix corrupted for hadamard product");

        let zero = self.zero();
        let mut res: Matrix<T> = mat![T];

        for i in 0..self.rows() {
            if i >= res.rows() {
                res.data.push(Vec::new());
            }
            for j in 0..self.cols() {
                res.data[i].push(zero);
                res.data[i][j] = self.data[i][j] * m.data[i][j];
            }
        }

        Ok(res)
    }

    /// 余因子行列取得関数
    /// 行p, 列q についての余因子行列を取得し、Result型に
    /// くるんだ Matrix<T>型として返却する。オブジェクト本体の
    /// 変更は行わないイミュータブルな実装。
    ///
    /// ```rust
    /// use tensors::core::{List,Matrix};
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
    ///         assert_eq!(adj.dump()[i][j], res.dump()[i][j]);
    ///     }
    /// }
    /// ```
    ///
    pub fn adjugate(&self, p: usize, q: usize) -> Result<Self, &str> {
        self.integrity_check().unwrap();
        self.range_check(p, q).unwrap();
        let mut res = Matrix::<T>::new();
        for i in 0..self.data.len() {
            let mut v = Vec::new();
            if i != p {
                for j in 0..self.data[0].len() {
                    if j != q {
                        v.push(self.data[i][j]);
                    }
                }
                res.push(v).expect("[vector length error] failed to construct new matrix");
            }
        }
        for i in 0..res.data.len() {
            for j in 0..res.data[0].len() {
                match (i + j) % 2 {
                    1 => {
                        let datum = res.data[i][j];
                        res.data[i][j] = datum - datum - datum;
                    }
                    _ => (),
                }
            }
        }
        Ok(res)
    }

    /// 行列式
    /// 行列式を計算し、型Tで結果を返却する
    ///
    /// ```rust
    /// use tensors::core::{List,Matrix};
    /// use tensors::mat;
    ///
    /// let m = mat![
    ///     i32:
    ///         [2,3,4,1],
    ///         [1,2,3,4],
    ///         [4,1,2,3],
    ///         [3,2,1,4]
    /// ];
    /// assert_eq!(m.det(),80);
    /// ```
    ///
    pub fn det(&self) -> T {
        self.integrity_check().unwrap();
        self.is_square().unwrap();

        if self.data.len() == 1 {
            self.data[0][0]
        } else if self.data.len() == 2 {
            self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
        } else {
            let mut res = self.data[0][0] - self.data[0][0];
            for i in 0..self.data.len() {
                let adj = self.adjugate(i, 0).unwrap();
                if i % 2 == 0 {
                    res = res + self.data[i][0] * adj.det();
                } else {
                    res = res - self.data[i][0] * adj.det();
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
    /// use tensors::core::{List,Matrix};
    /// use tensors::mat;
    ///
    /// let r = mat![i32:[1,2],[3,4]];
    /// r.is_regular().unwrap();
    /// ```
    pub fn is_regular(&self) -> Result<&Self, &str> {
        if self.det() != self.zero() {
            Ok(self)
        } else {
            Err("the matrix is not a regular matrix")
        }
    }

    /// 逆行列
    ///
    /// 正則行列の逆行列を取得する関数。
    /// selfの逆行列としてResult型に格納したMatrix<T>を返却し、
    /// 正則行列でないものについてはErrを返却する。
    ///
    /// ```rust
    /// use tensors::core::{List,Matrix};
    /// use tensors::mat;
    ///
    /// let mut d = mat![
    ///    f64:
    ///        [1.0,2.0,0.0],
    ///        [3.0,1.0,2.0],
    ///        [-1.0,3.0,1.0]
    /// ];
    /// let result = d.inverse().unwrap().get();
    /// let result_cmp = mat![
    ///    f64:
    ///        [0.3333333333333333, 0.13333333333333333, -0.26666666666666666],
    ///        [0.3333333333333333, -0.06666666666666667, 0.13333333333333333],
    ///        [-0.6666666666666666, 0.3333333333333333, 0.3333333333333333]
    /// ];
    ///
    /// println!("d is ");
    /// for e in d.dump() {
    ///    println!("{:?}",e);
    /// }
    /// println!("d inverse is ");
    /// for i in 0..result.rows() {
    ///     println!("{:?}",result.col(i));
    ///     for j in 0..result.cols() {
    ///         assert_eq!(result.dump()[i][j], result_cmp.dump()[i][j]);
    ///     }
    /// }
    /// ```
    ///
    pub fn inverse(&self) -> Result<Self, &str> {
        match self.is_regular() {
            Err(_) => Err("not a regular matrix"),
            Ok(_) => {
                let mut res = mat![T: [self.zero()]];
                res.resize(self.rows(), self.cols())
                    .expect("[matrix resize error]: failed to generate new matrix instance for the inverse");

                let det = self.det();

                for i in 0..res.rows() {
                    for j in 0..res.cols() {
                        let datum = self.adjugate(i, j)
                            .expect("[adjugate error] failed to get adjugate for the inverse")
                            .det() / det;
                        if (i + j) % 2 == 0 {
                            res.data[i][j] = datum;
                        } else {
                            res.data[i][j] = datum - datum - datum;
                        }
                    }
                }
                Ok(res.transpose().get())
            }
        }
    }

    /// 単位行列
    /// 行列と同一サイズの単位行列が定義できる場合にはそれを生成し、
    /// 新規のMatrix<T>インスタンスとしてResult型に格納して返却する
    ///
    pub fn identity(&self) -> Result<Self, &str> {
        match self.is_square() {
            Err(_) => Err("identify matrix is not defined for a rectangle matrix"),
            Ok(_) => {
                let mut res = self.get();
                let one = T::from(0x1u8);
                res.fill_zero();
                for i in 0..res.rows() {
                    res.data[i][i] = one;
                }
                Ok(res)
            }
        }
    }

    ///トレース
    ///行列のトレースを計算し、結果をResult型に格納して返却する
    ///
    pub fn tr(&self) -> Result<T, &str> {
        match self.is_square() {
            Err(_) => Err("trace is not defined for a non-square matrix"),
            Ok(_) => {
                let mut res = self.zero();
                for i in 0..self.rows() {
                    res = res + self.data[i][i];
                }
                Ok(res)
            }
        }
    }
}

impl<T> Matrix<T>
where
    T: Copy + std::ops::RemAssign + std::fmt::Display + std::fmt::Debug,
{
    /// スカラー剰余計算
    /// int系の型のみサポート
    ///
    pub fn residue(&mut self, val: T) -> &mut Self {
        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                self.data[i][j] %= val;
            }
        }
        if self.debug {
            println!("residue {} foreach", val);
            println!("{:?}", self.data);
        }
        self
    }
}

#[cfg(test)]
mod tests_matrix {
    use crate::core::{List, Matrix};

    #[test]
    fn test_new_i32() {
        let m = Matrix::<i32>::new();
        assert_eq!(m.data.len(), 0);
    }

    #[test]
    fn test_new_f32() {
        let m = Matrix::<f32>::new();
        assert_eq!(m.data.len(), 0);
    }

    #[test]
    fn test_macro_with_type() {
        let m = mat![f32];
        assert_eq!(m.data.len(), 0);
        //m.data.push(vec![1.234,5.678]);
    }

    #[test]
    fn test_macro_with_values() {
        let m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        assert_eq!(m.data.len(), 3);
        assert_eq!(m.data[0].len(), 5);
        assert_eq!(m.data[2][2], 5);
        //m.data.push(vec![5,6,7,8,9]);
    }

    #[test]
    #[should_panic]
    fn test_macro_invalid_len() {
        mat![i32: [1,23],[4,5,6]];
    }

    #[test]
    fn test_print() {
        let m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        m.print();
    }

    #[test]
    #[should_panic]
    fn test_is_square_error() {
        let m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        m.is_square().unwrap();
    }

    #[test]
    fn test_integrity() {
        let m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        m.integrity_check().unwrap();
    }

    #[test]
    #[should_panic]
    //ゼロ値でパニック
    fn test_integrity_error_zero() {
        let m = Matrix::<i32>::new();
        m.integrity_check().unwrap();
    }

    #[test]
    #[should_panic]
    //行列でないデータではパニック
    fn test_integrity_error_corrupted() {
        let mut m = Matrix::<i32>::new();
        //privateフィールドに手動でベクトル代入
        m.data.push(vec![1, 2, 3]);
        m.data.push(vec![1, 2, 3, 4, 5]);
        m.integrity_check().unwrap();
    }

    #[test]
    fn test_push() {
        Matrix::<i32>::new()
            .push(vec![1, 2])
            .unwrap()
            .push(vec![3, 4])
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_push_unmatched_len() {
        Matrix::<i32>::new()
            .push(vec![1, 2, 3])
            .unwrap()
            .push(vec![1])
            .unwrap();
    }

    #[test]
    fn test_row() {
        let m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        assert_eq!(m.row(1)[2], 4);
        assert_eq!(m.row(0)[1], m.row(1)[0]);
    }

    #[test]
    #[should_panic]
    fn test_row_error() {
        let m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        m.row(5);
    }

    #[test]
    fn test_col() {
        let m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        assert_eq!(m.col(1)[1], 3);
        assert_eq!(m.col(1)[2], m.col(3)[0]);
    }

    #[test]
    fn test_rows() {
        let m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        assert_eq!(m.rows(), 3);
    }

    #[test]
    fn test_cols() {
        let m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        assert_eq!(m.cols(), 5);
    }

    #[test]
    #[should_panic]
    fn test_col_error() {
        let m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        m.row(4);
    }

    #[test]
    fn test_row_replace() {
        let mut a = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        let p0 = vec![1, 2, 3, 4, 5];
        let p2 = vec![3, 4, 5, 6, 7];
        for i in 0..a.data[0].len() {
            assert_eq!(a.data[0][i], p0[i]);
            assert_eq!(a.data[2][i], p2[i]);
        }

        a.row_replace(0, 2).unwrap();
        for i in 0..a.data[0].len() {
            assert_eq!(a.data[0][i], p2[i]);
            assert_eq!(a.data[2][i], p0[i]);
        }
    }

    #[test]
    fn test_col_replace() {
        let mut m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        let p0 = vec![1, 2, 3];
        let p2 = vec![3, 4, 5];
        for i in 0..m.data.len() {
            assert_eq!(m.data[i][0], p0[i]);
            assert_eq!(m.data[i][2], p2[i]);
        }

        m.col_replace(0, 2).unwrap();
        for i in 0..m.data.len() {
            assert_eq!(m.data[i][0], p2[i]);
            assert_eq!(m.data[i][2], p0[i]);
        }
    }

    #[test]
    fn test_transpose() {
        let mut m = mat![i32: [1,2,3], [3,4,5],[5,6,7]];
        let res = mat![
            i32:
                [1,3,5],
                [2,4,6],
                [3,5,7]
        ];
        m.transpose();
        for i in 0..m.data.len() {
            for j in 0..m.data[0].len() {
                assert_eq!(m.data[i][j], res.data[i][j]);
            }
        }
    }
}

#[cfg(test)]
mod tests_matrix_operation {
    use crate::core::Matrix;

    #[test]
    fn test_add() {
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
    fn test_sub() {
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
    fn test_mul() {
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
    fn test_div() {
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
    fn test_prod_error_unmatched() {
        let m = mat![
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
    fn test_prod_error_zero() {
        let m = mat![
            i32:
                [1,2,3],
                [4,5,7]
        ];
        let n = Matrix::<i32>::new();
        m.prod(n).unwrap();
    }

    #[test]
    fn test_det_2x2() {
        let m = mat![
            i32:
                [1,2],
                [-3,-4]
        ];
        assert_eq!(m.det(), 2);
    }

    #[test]
    fn test_det_1x1() {
        let m = mat![i32: [5]];
        assert_eq!(m.det(), 5);
    }

    #[test]
    fn test_det_3x3() {
        let m = mat![
            i32:
                [1,2,3],
                [0,1,1],
                [1,1,5]
        ];
        assert_eq!(m.det(), 3);
    }

    #[test]
    fn test_det_3x3_float() {
        let m = mat![
            f32:
                [1.0,2.0,3.0],
                [0.0,1.0,1.0],
                [1.0,1.0,5.0]
        ];
        assert_eq!(m.det(), 3.0);
    }

    #[test]
    #[should_panic]
    fn test_det_3x2_error() {
        let m = mat![
            i32:
                [1,2],
                [0,1],
                [1,1]
        ];
        m.det();
    }
}
