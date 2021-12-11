use std::ops::Rem;
use crate::core::Matrix;
use crate::mat;
use PartialEq;
use std::ops::{Add, Mul, Sub};

/// 行列の加算
///
/// 行列の要素ごとの加算を行い、新規インスタンスとして結果を返却する。
/// 行および列の数が一致しない行列が指定された場合はパニックする。
///
/// ```rust
/// use matrixa::core::Matrix;
/// use matrixa::mat;
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

impl<T: std::ops::Add<Output = T>> Add for Matrix<T>
    where
        T: Copy + std::ops::Add<Output = T> + std::fmt::Debug + From<u8>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        if !self.has_same_size_with(other.get()) {
            panic!("abort");
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
///
/// 行列の要素ごとの減算を行い、新規インスタンスとして結果を返却する。
/// 行および列の数が一致しない行列が指定された場合はパニックする。
///
/// ```rust
/// use matrixa::core::Matrix;
/// use matrixa::mat;
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

impl<T: std::ops::Sub<Output = T>> Sub for Matrix<T>
    where
        T: Copy + std::ops::Sub<Output = T> + std::fmt::Debug + From<u8>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {

        if !self.has_same_size_with(other.get()) {
            panic!("abort");
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
/// use matrixa::core::Matrix;
/// use matrixa::mat;
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



/// 数値計算用共通メソッド群
///
/// 整数型、浮動小数点型、虚数型に対する演算処理
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
    /// use matrixa::core::Matrix;
    /// use matrixa::mat;
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

    /// 積
    ///
    /// 引数として与えられた同一型の行列インスタンスを用いて
    /// self.data を元とする行列の積を計算し、Result型に格納した新規インスタンスを返却する。
    /// std::ops::Mul を実装する場合 * 演算子を用いて同様の計算が可能だが、Result型を通じたエラー制御が可能。
    ///
    /// ```rust
    /// use matrixa::core::Matrix;
    /// use matrixa::mat;
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
    ///
    /// 行列の要素ごとの積(element-wize or pointwise product)を求め、
    /// Result型に格納した新規インスタンスを返却する。
    ///
    /// ```rust
    /// use matrixa::core::Matrix;
    /// use matrixa::mat;
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
    ///
    /// 行p, 列q についての余因子行列を取得し、Result型に
    /// くるんだ Matrix<T>型として返却する。オブジェクト本体の
    /// 変更は行わないイミュータブルな実装。
    ///
    /// ```rust
    /// use matrixa::core::Matrix;
    /// use matrixa::mat;
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
    ///
    /// 行列式を計算し、型Tで結果を返却する
    ///
    /// ```rust
    /// use matrixa::core::Matrix;
    /// use matrixa::mat;
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
    ///
    /// 正則行列であるかどうか調べ、Result型にくるんで
    /// オブジェクト参照を返却する。
    ///
    /// ```rust
    /// use matrixa::core::Matrix;
    /// use matrixa::mat;
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
    /// use matrixa::core::Matrix;
    /// use matrixa::mat;
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

impl<T: std::ops::Rem<Output = T>> Matrix<T>
    where
        T: Copy + std::ops::Rem<Output = T> + std::fmt::Display + std::fmt::Debug,
{
    /// スカラー剰余計算
    /// int系の型のみサポート
    ///
    pub fn residue(&mut self, val: T) -> &mut Self {
        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                self.data[i][j] = self.data[i][j] % val;
            }
        }
        self
    }
}

impl<T: std::ops::Rem<Output = T>> Rem for Matrix<T>
where
    T: Copy + std::ops::Rem<Output = T> + std::fmt::Display + std::fmt::Debug,
{
    type Output = Self;
    /// 行列の要素ごとの剰余
    ///
    /// 整数型の行列のみサポート
    ///
    fn rem(self, other: Self) -> Self {

        let mut res = Matrix::<T>::new();

        for i in 0..self.data.len() {
            res.data.push(Vec::new());
            for j in 0..self.data[0].len() {
                res.data[i].push(self.data[i][j] % other.data[i][j]);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests_matrix_numeric_operator {
    use crate::core::Matrix;
    use crate::mat;

    #[test]
    fn test_plus() {
        let m = mat![i32: [1,2,3],[4,5,6],[7,8,9]];
        let n = mat![i32: [2,3,4],[5,6,7],[8,9,10]];
        let res = mat![i32: [3,5,7],[9,11,13],[15,17,19]];
        let e = m + n;
        assert_eq!(e == res, true)
    }

    #[test]
    fn test_minus() {
        let m = mat![i32: [1,2,3],[4,5,6],[7,8,9]];
        let n = mat![i32: [2,3,4],[5,6,7],[8,9,10]];
        let res = mat![i32: [-1,-1,-1],[-1,-1,-1],[-1,-1,-1]];
        let e = m - n;
        assert_eq!(e == res, true)
    }

    #[test]
    fn test_astar() {
        let m = mat![i32: [1,2,3],[4,5,6],[7,8,9]];
        let n = mat![i32: [2,3,4],[5,6,7],[8,9,10]];
        let res = mat![i32: [36,42,48],[81,96,111],[126,150,174]];
        let e = m * n;
        assert_eq!(e == res, true)
    }

}

#[cfg(test)]
mod tests_matrix_manipulation {
    use crate::core::Matrix;
    use crate::mat;

    #[test]
    fn test_add() {
        let mut m = mat![i32: [1,2,3],[4,5,6],[7,8,9]];
        let res = mat![i32: [2,3,4],[5,6,7],[8,9,10]];
        m.add(1);
        assert_eq!(m == res, true)
    }

    #[test]
    fn test_sub() {
        let mut m = mat![i32: [1,2,3],[4,5,6],[7,8,9]];
        let res = mat![i32: [-4,-3,-2],[-1,0,1],[2,3,4]];
        m.sub(5);
        assert_eq!(m == res, true)
    }

    #[test]
    fn test_mul() {
        let mut m = mat![i32: [1,2,3],[4,5,6],[7,8,9]];
        let res = mat![i32: [2,4,6],[8,10,12],[14,16,18]];
        m.mul(2);
        assert_eq!(m == res, true)
    }

    #[test]
    fn test_div() {
        let mut m = mat![i32: [2,4,6],[8,10,12],[14,16,18]];
        let res = mat![i32: [1,2,3],[4,5,6],[7,8,9]];
        m.div(2);
        assert_eq!(m == res, true)
    }

    #[test]
    fn test_residue(){
        let mut m = mat![i32: [1,2,3],[4,5,6],[-7,-8,-9]];
        let result = mat![i32: [1,0,1],[0,1,0],[-1,0,-1]];
        m.residue(2);
        assert_eq!(m == result, true);
    }

    #[test]
    fn test_rem(){
        let m = mat![i32: [1,2,3],[4,5,6],[-7,-8,-9]];
        let other = mat![i32: [-7,-8,-9],[6,5,4],[3,2,1]];
        let result = mat![i32: [1,2,3],[4,0,2],[-1,0,0]];
        let m = m % other;
        assert_eq!(m == result, true)
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

    #[test]
    #[should_panic]
    fn test_is_not_regular(){
        let m = mat![
            i32:
                [1,1],
                [1,1]
        ];
        m.is_regular().unwrap();
    }

    #[test]
    fn test_adjugate(){
        let m = mat![
            i32:
                [1,2,3],
                [4,5,7],
                [9,17,13289]
        ];
        let res = mat![
            i32:
                [1,-3],
                [-9,13289]
        ];
        let adj = m.adjugate(1,1).unwrap();
        assert_eq!(adj.rows(), 2);
        assert_eq!(adj.cols(), 2);
        for i in 0..adj.rows() {
            for j in 0..adj.cols() {
                assert_eq!(adj.dump()[i][j], res.dump()[i][j]);
            }
        }
    }

    #[test]
    fn test_inverse(){
        let d = mat![
           f64:
               [1.0,2.0,0.0],
               [3.0,1.0,2.0],
               [-1.0,3.0,1.0]
        ];
        let result = d.inverse().unwrap().get();
        let result_cmp = mat![
           f64:
               [0.3333333333333333, 0.13333333333333333, -0.26666666666666666],
               [0.3333333333333333, -0.06666666666666667, 0.13333333333333333],
               [-0.6666666666666666, 0.3333333333333333, 0.3333333333333333]
        ];

        for i in 0..result.rows() {
            println!("{:?}",result.col(i));
            for j in 0..result.cols() {
                assert_eq!(result.dump()[i][j], result_cmp.dump()[i][j]);
            }
        }
    }
}
