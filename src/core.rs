//! Matrixa is a simple matrix manipulation library which supports
//! row and column matrix manipulations, scalar manipulation,
//! mathematical manipulation, filtering and parsing mechanisms for
//! various type of data stored in vector of vector (Vec<Vec<T>>).
//!
//! すべての行列で、すべての場合において可能な演算については値ないしは参照を返却し、
//! 行列と引数によっては演算が定義されないものについてはResult型を返却する。
//!
//! ```rust
//! use matrixa::core::Matrix;
//! use matrixa::mat;
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

use std::fmt::Debug;
use PartialEq;

pub struct Matrix<T> {
    pub data: Vec<Vec<T>>,
    pub debug: bool,
    current: usize,
    max: usize,
}


/// イテレータ実装 / Iterator
///
/// Matrix はIterator を実装しており、for文等での数え上げに使える。
///
/// ```rust
/// use matrixa::core::Matrix;
/// use matrixa::mat;
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
where T: Clone,
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

/// 行列の完全一致・不一致 / PartialEq
///
/// 行列の要素ごとの比較を行い、結果をboolで返却する。
/// 行および列の数が一致しない行列が指定された場合はパニックする。
///
/// ```rust
/// use matrixa::core::Matrix;
/// use matrixa::mat;
///
/// let m = mat![i32:[1,2],[3,4]];
/// let n = mat![i32:[1,2],[3,4]];
/// let o = mat![i32:[1,1],[3,4]];
/// assert_eq!(m == n, true);
/// assert_eq!(n == n, true);
/// assert_eq!(m == o, false);
/// assert_eq!(n == o, false);
/// ```
///
impl<T: Clone + std::cmp::PartialEq + std::fmt::Debug> PartialEq for Matrix<T>
where T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        if !self.has_same_size_with(other.clone()) {
            return false
        }

        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                if self.data[i][j] != other.data[i][j] {
                    return false
                }
            }
        }
        true
    }
    fn ne(&self, other: &Self) -> bool {
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                if self.data[i][j] != other.data[i][j] {
                    return true
                }
            }
        }
        false
    }
}

/// 行列インスンタンス初期化用マクロ / initialization macro
///
/// ```rust
/// use matrixa::core::Matrix;
/// use matrixa::mat;
///
///  let mut im = mat![i32];
///  assert_eq!(im.rows(), 0);
///  im.print();
///
///  let fm = mat![
///     f32:
///         [1.0,2.0,3.0]
///  ];
///  assert_eq!(fm.cols(), 3);
///  assert_eq!(fm.row(0)[2], 3.0);
///
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

impl<T> Matrix<T> {
    /// 行列生成
    ///
    /// 新規の行列インスタンスを生成し、空行列として返却する
    ///
    pub fn new() -> Self {
        let v: Vec<Vec<T>> = Vec::new();
        Matrix {
            data: v,
            debug: false,
            current: 0,
            max: 0,
        }
    }
}

impl<T: Debug> Matrix<T>
{
    /// サイズ検証 / size matcher
    ///
    /// 行列のサイズを引数行列のサイズと比較し、結果をResult型にくるんで返却する。
    /// 一致する場合にはSelf型、一致しない場合にはエラーメッセージを返却する。
    ///
    pub fn has_same_size_with(&self, other: &Self) -> bool {
        match self {
            _ if self.data.len() == 0 => {
                println!("zero length origin for addition");
                false
            },
            _ if self.data.len() != other.data.len() => {
                println!("column number not matched {}, {}", self.data.len(),other.data.len());
                false
            },
            _ if self.data[0].len() != other.data[0].len() => {
                println!("row number not matched {}, {}", self.data[0].len(), other.data[0].len());
                for i in 0..self.data.len() {
                    for j in 0..self.data[i].len() {
                        print!("d: {:?}, o: {:?}", self.data[i][j], other.data[i][j]);
                    }
                }
                false
            }
            _ => true,
        }
    }
}

impl<T: Copy> Clone for Matrix<T> {
    /// 複製 / Clone
    ///
    /// selfと同一のデータを有する新規インスタンスを生成する。
    /// TがCopyを実装する場合、Matrix構造体のデータはVec<Vec<T>>であるためCloneを実装する。
    /// selfの保持するデータと同一のデータを保持する新規インスタンスを生成することができる。
    ///
    /// ```rust
    /// use matrixa::core::Matrix;
    /// use matrixa::mat;
    ///
    /// let x = mat![i32:[1,2,3],[1,5,6]];
    /// let y = x.clone();
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
    fn clone(&self) -> Matrix<T> {
        let mut res = Matrix::new();

        for i in 0..self.data.len() {
            res.data.push(self.data[i].clone());
        }
        res
    }

}

impl<T: std::fmt::Debug> Matrix<T> {

    /// データ表示関数
    ///
    pub fn print(&self) {
        println!("{:?}", self.data)
    }

    /// 行列データ取得
    ///
    /// データペイロードとしてベクトルベクトルを返却する。
    pub fn dump(&self) -> &Vec<Vec<T>> {
        &self.data
    }

    /// デバッガ
    ///
    /// デバッグフラグを有効化したインスタンスを返す。
    ///
    pub fn debug(&mut self) -> &mut Self {
        self.debug = true;
        println!("debugging for: {:?}", self.data);
        self
    }

    /// データ追加
    ///
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

        self.data.push(data);
        Ok(self)
    }
}

/// [行列一般] 基本メソッド群
///
/// 数値行列および文字行列のいずれにも対応したメソッドを定義。
///
impl<T> Matrix<T>
    where
        T: std::fmt::Debug + Copy,
{

    /// 行抽出関数
    ///
    /// Vec<T>として行を返却
    ///
    pub fn row(&self, num: usize) -> Vec<T> {
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
    ///
    /// Vec<T>として列を返却
    ///
    pub fn col(&self, num: usize) -> Vec<T> {
        self.integrity_check().unwrap();
        let mut res: Vec<T> = Vec::new();
        for i in 0..self.data.len() {
            res.push(self.data[i][num]);
        }
        res
    }

    /// 正方行列判定
    ///
    /// 正方行列であるかどうかを判定し、Result型に格納したオブジェクト参照を返却する
    ///
    /// ```rust
    /// use matrixa::core::Matrix;
    /// use matrixa::mat;
    ///
    /// let m = mat![i32: [1,2,3], [2,3,4],[3,4,5]];
    /// m.is_square().unwrap();
    /// ```
    ///
    pub fn is_square(&self) -> Result<&Self, &str> {
        if self.data.len() != self.data[0].len() {
            Err("not a square matrix")
        } else {
            Ok(&self)
        }
    }

    /// 行数表示関数
    ///
    pub fn rows(&self) -> usize {
        self.data.len()
    }

    /// 列数表示関数
    ///
    pub fn cols(&self) -> usize {
        if self.data.len() == 0 {
            0
        } else {
            self.data[0].len()
        }
    }

    /// 行置換操作
    ///
    pub fn row_replace(&mut self, src: usize, dst: usize) -> Result<&mut Self, &str> {
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
    pub fn col_replace(&mut self, src: usize, dst: usize) -> Result<&mut Self, &str> {
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
    ///
    /// 転置行列でデータを更新し、オブジェクト参照を返却する。
    ///
    ///```rust
    /// use matrixa::core::Matrix;
    /// use matrixa::mat;
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

    pub fn transpose(&mut self) -> &mut Self {
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
    ///
    /// 長さ0もしくは長さの一致しないデータを検出した時点で強制終了
    ///
    pub fn integrity_check(&self) -> Result<&Self, &str> {
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
    ///
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
    ///
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
    ///
    /// 行および列の値をusizeで指定し、行列の幅・高さに収まるかどうかを検証。
    /// 結果をResult型にオブジェクト参照を格納して返却
    ///
    pub fn range_check(&self, row: usize, col: usize) -> Result<&Self, &str> {
        self.row_check(row)?.col_check(col)
    }

    /// 行列セッタ / data setter
    ///
    /// Vec<Vec<T>>への参照として行列データをセットする関数。
    ///
    /// ```rust
    /// use matrixa::core::Matrix;
    /// use matrixa::mat;
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
        self.data = m.clone();
    }

    /// 行列ゲッタ / data getter
    ///
    /// 設定済みの行列データを新規 Vec<Vec<T>> インスタンスとして返却する。
    ///
    /// ```rust
    /// use matrixa::core::Matrix;
    /// use matrixa::mat;
    ///
    /// let mut m = mat![i32];
    /// let v = vec![
    ///     vec![1,2,3],
    ///     vec![1,2,3],
    /// ];
    /// m.data = v.clone();
    /// m.print();
    ///
    /// let e = m.get();
    /// for i in 0..1 {
    ///     for j in 0..2 {
    ///         assert_eq!(e[i][j], v[i][j]);
    ///     }
    /// }
    /// ```
    ///
    pub fn get(&self) -> Vec<Vec<T>> {
        self.data.clone()
    }
}


/// 変換系メソッド群 / conversion methods
///
/// 異なる型を元とする行列への型変換を行う関数群。
///
impl<T: Copy + ToString> Matrix<T> {

    /// 文字列行列への変換 / conversion to String matrix
    ///
    /// ToStringを実装する元を有する行列について、全要素をString型に変換したMatrix<String>を返却。
    ///
    pub fn to_string(&self) -> Matrix<String> {
        let mut res = mat![String];
        for i in 0..self.data.len() {
            res.data.push(Vec::new());
            for j in 0..self.data.len() {
                res.data[i].push(self.data[i][j].to_string());
            }
        }
        res
    }
}




#[cfg(test)]
mod tests_matrix {
    use crate::core::Matrix;
    use crate::mat;

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
    }

    #[test]
    fn test_macro_with_str() {
        let m = mat![
            &str:
                ["abcde","fghij","klmn0"],
                ["bbcde","matched","olmn0"]
        ];
        assert_eq!(m.data.len(), 2);
        assert_eq!(m.data[0].len(), 3);
        assert_eq!(m == m, true);
    }

    #[test]
    fn test_macro_with_string() {
        let m = mat![
            String:
                [String::from("abcde"),String::from("fghij"),String::from("klmn0")],
                [String::from("bbcde"),String::from("matched"),String::from("olmn0")]
        ];
        assert_eq!(m.data.len(), 2);
        assert_eq!(m.data[0].len(), 3);
    }

    #[test]
    fn test_macro_with_bool() {
        let m = mat![
            bool:
                [true,true,false,true,false],
                [true,false,false,true,true],
                [false,false,true,true,true],
                [false,true,true,false,true]
        ];
        assert_eq!(m.data.len(), 4);
        assert_eq!(m.data[0].len(), 5);
    }

    #[test]
    #[should_panic]
    fn test_macro_invalid_len() {
        mat![i32: [1,23],[4,5,6]];
    }

    #[test]
    fn test_eq() {
        let m = mat![i32: [1,2,3,5,5], [3,6,1,4,2], [3,6,0,1,5]];
        let n = mat![i32: [1,2,3,5,5], [3,6,1,4,2], [3,6,0,1,5]];
        assert_eq!(m.has_same_size_with(&n), true);
        assert_eq!(m == m, true);
        assert_eq!(m == n, true);
    }

    #[test]
    fn test_ne() {
        let m = mat![i32: [1,2,3,5,5], [3,6,1,4,2], [3,6,0,1,5]];
        let n = mat![i32: [1,2,3,5,5], [3,6,199293,4,2], [3,6,0,1,5]];
        assert_eq!(m.has_same_size_with(&n), true);
        assert_eq!(m != n, true);
    }

    #[test]
    fn test_eq_str() {
        let m = mat![
            &str:
                ["abcde","fghij","klmn0"],
                ["bbcde","matched","olmn0"]
        ];
        let n = mat![
            &str:
                ["abcde","fghij","klmn0"],
                ["bbcde","matched","olmn0"]
        ];
        let p = mat![
            &str:
                ["abcde","fghij","klmn0"],
                ["bbcde","NOT matched","olmn0"]
        ];        assert_eq!(m.data.len(), 2);
        assert_eq!(m.data[0].len(), 3);
        assert_eq!(m == m, true);

        assert_eq!(m.has_same_size_with(&n), true);
        assert_eq!(m == n, true);
        assert_eq!(m != n, false);
        assert_eq!(m == p, false);
        assert_eq!(m != p, true);
        assert_eq!(n == p, false);
        assert_eq!(n != p, true);
        println!("{}",m.data[0][0].contains("a"))
    }

    #[test]
    fn test_eq_string() {
        let m = mat![
            &str:
                ["abcde","fghij","klmn0"],
                ["bbcde","matched","olmn0"]
        ].to_string();
        let n = mat![
            &str:
                ["abcde","fghij","klmn0"],
                ["bbcde","matched","olmn0"]
        ].to_string();
        let p = mat![
            &str:
                ["abcde","fghij","klmn0"],
                ["bbcde","NOT matched","olmn0"]
        ].to_string();
        assert_eq!(m == m, true);

        assert_eq!(m.has_same_size_with(&n), true);
        assert_eq!(m == n, true);
        assert_eq!(m != n, false);
        assert_eq!(m == p, false);
        assert_eq!(m != p, true);
        assert_eq!(n == p, false);
        assert_eq!(n != p, true);
        println!("{}",m.data[0][0].contains("a"))
    }

    #[test]
    fn test_assign() {
        let m = mat![i32: [1,2,3],[4,5,6],[7,8,9]];
        let n = m.clone();
        let o = n;
        assert_eq!(m == o, true);
    }

    #[test]
    fn test_assign_str() {
        let m = mat![
            &str:
            ["新宿","渋谷","代々木","神田"],
            ["吉祥寺","飯田橋","阿佐ヶ谷","白金"],
            ["保土ヶ谷","荻窪","墨田","北千住"]
        ];
        let n = m.clone();
        let o = n;
        assert_eq!(m == o, true);
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
    //正しい行列(矩形)でOk返却
    fn test_integrity() {
        let m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        m.integrity_check().unwrap();
    }

    #[test]
    #[should_panic]
    //ゼロ値でエラー返却
    fn test_integrity_error_zero() {
        let m = Matrix::<i32>::new();
        m.integrity_check().unwrap();
    }

    #[test]
    #[should_panic]
    //行列でないデータではErrを返却
    fn test_integrity_error_corrupted() {
        let mut m = Matrix::<i32>::new();
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
mod tests_matrix_conversion {
    use crate::core::Matrix;
    use crate::mat;

    #[test]
    fn test_to_string(){
        let m = mat![i32: [2,2,3],[4,5,6],[7,8,9]];
        let res = mat![
            &str:
            ["2","2","3"],
            ["4","5","6"],
            ["7","8","9"]
        ];
        let s = m.to_string();
        for i in 0..s.data.len() {
            for j in 0..s.data[i].len() {
                assert_eq!(s.data[i][j].as_str() == res.data[i][j], true);
            }
        }

        let t = res.to_string();
        for i in 0..t.data.len() {
            for j in 0..t.data[i].len() {
                assert_eq!(t.data[i][j].as_str() == res.data[i][j], true);
            }
        }
    }
}