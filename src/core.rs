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
    fn row_len(&self) -> usize;
    fn col_len(&self) -> usize;
    fn row_replace(&mut self, src: usize, dst: usize) -> &mut Self;
    fn col_replace(&mut self, src: usize, dst: usize) -> &mut Self;
    fn transpose(&mut self) -> &mut Self;
    fn integrity_check(&self);
    fn debug(&mut self) -> &mut Self;
}

pub struct Numbers<T> {
    data: Vec<Vec<T>>,
    current: usize,
    max: usize,
    debug: bool,
}

pub trait TensorProcessor<T> {
    fn zero(&mut self) -> &mut Self;
    fn add(&mut self, val: T) -> &mut Self;
    fn sub(&mut self, val: T) -> &mut Self;
    fn mul(&mut self, val: T) -> &mut Self;
    fn div(&mut self, val: T) -> &mut Self;
    fn prod(&mut self, val: Numbers<T>) -> &mut Self;
    fn determinant(&self) -> T;
    fn adjugate(&self, p: usize, q: usize) -> Result<Numbers<T>,&str>;
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
///  let mut fm = mat![f32: [1.0,2.0,3.0]];
///  assert_eq!(im.row_len(), 0);
///  assert_eq!(fm.col_len(), 3);
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
    fn print(&self){
        println!("{:?}",self.data)
    }

    //行列データへのアクセサ
    //
    fn get(&self) -> &Vec<Vec<T>>{
        &self.data
    }

    //行列データのセッター
    //
    fn set(&mut self, m: Vec<Vec<T>>){
        if m.len() == 0 {
            panic!("argument has zero length");
        }
        if self.debug {
            println!("new data set: {:?}", m);
        }
        self.data = m;
    }

    //デバッガ
    //デバッグフラグを有効化したインスタンスを返す。
    //
    fn debug(&mut self) -> &mut Self {
        self.debug = true;
        println!("debuging for: {:?}",self.data);
        self
    }

    // 行列データ整合性検証
    // 長さ0もしくは長さの一致しないデータを検出した時点で強制終了
    //
    fn integrity_check(&self) {
        if self.data.len() == 0 {
            panic!("zero matrix length detected");
        }
        for i in 0..self.data.len() {
            let len = self.data[0].len(); 
            if self.data[i].len() != len {
                panic!("matrix corrupted at column {} (data: {:?}, length: {}, expected {})", i, self.data[i], self.data[i].len(), len);
            }
        }

    }

    //行抽出関数
    //Vec<T>として行を返却
    //
    fn row(&self, num: usize) -> Vec<T> {
        self.integrity_check();
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

    //列抽出関数
    //Vec<T>として列を返却
    //
    fn col(&self, num: usize) -> Vec<T> {
        self.integrity_check();
        if num >= self.data[0].len() {
            panic!("colum number {} is out of order: must be less than {}",num, self.data[0].len());
        }
        let mut res: Vec<T> = Vec::new();
        for i in 0..self.data.len() {
            res.push(self.data[i][num]);
        }
        res
    }

    //行数表示関数
    //
    fn row_len(&self) -> usize {
        self.data.len()
    }

    //列数表示関数
    //
    fn col_len(&self) -> usize {
        self.integrity_check();
        self.data[0].len()
    }

    //正方行列判定
    //オブジェクト参照をResultにくるんで返却する
    //
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
        self.integrity_check();

        self
    }

    //列置換操作
    //
    fn col_replace(&mut self, src: usize, dst: usize) -> &mut Self {
        self.integrity_check();

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

    // 転置
    //
    fn transpose (&mut self) -> &mut Self {
        self.integrity_check();

        let mut res: Vec<Vec<T>> = Vec::new();
        let col_len = self.data[0].len();
        for i in 0..col_len {
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
        for i in 0..col_len {
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
        self.integrity_check();
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
    fn sub(&mut self, val: T) -> &mut Self {
        self.integrity_check();
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
    fn mul(&mut self, val: T) -> &mut Self {
        self.integrity_check();
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
    // (i32では端数切捨て)
    fn div(&mut self, val: T) -> &mut Self {
        self.integrity_check();
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

    //行列乗算 
    //引数として与えられた同一型の行列インスタンスを用いて乗算を行い、
    //データ変更後のインスタンスを返却する。
    //
    fn prod (&mut self, m: Numbers<T>) -> &mut Self {
        self.integrity_check();
        m.integrity_check();
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

        if self.debug {
            println!("matrix multiple for m: {:?}", m.data);
            println!("{:?}",self.data);
        }
        self
    }

    //余因子行列
    //行p, 列q についての余因子行列を取得し、Resultでくるんで返却する
    //
    fn adjugate(&self, p: usize, q: usize) -> Result<Numbers<T>,&str> {
        self.integrity_check();
        if p >= self.data.len() {
            return Err("row is out of order")
        } else if q >= self.data[0].len() {
            return Err("col is out of order")
        }
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

    //行列式
    //行列式を計算し、型Tで結果を返却する
    //
    fn determinant(&self) -> T {
        self.integrity_check();
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
    fn test_is_square(){
        let m = mat![i32: [1,2,3], [2,3,4],[3,4,5]];
        m.is_square().unwrap();
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
        m.integrity_check();
    }

    #[test]
    #[should_panic]
    //ゼロ値でパニック
    fn test_integrity_error_zero(){
        let mut m = Numbers::<i32>::new();
        m.integrity_check();
    }

    #[test]
    #[should_panic]
    //行列でないデータではパニック
    fn test_integrity_error_corrupted(){
        let mut m = Numbers::<i32>::new();
        //privateフィールドに手動でベクトル代入
        m.data.push(vec![1,2,3]);
        m.data.push(vec![1,2,3,4,5]);
        m.integrity_check();
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
    fn test_row_len(){
        let m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        assert_eq!(m.row_len(),3);
    }

    #[test]
    fn test_col_len(){
        let m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        assert_eq!(m.col_len(),5);
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


    #[test]
    fn test_transpose_rect(){
        let mut m = mat![i32: [1,2,3,4,5], [2,3,4,5,6],[3,4,5,6,7]];
        assert_eq!(m.data.len(),3);
        assert_eq!(m.data[0].len(),5);
        let res = mat![
            i32:
                [1,2,3],
                [2,3,4],
                [3,4,5],
                [4,5,6],
                [5,6,7]
        ];
        m.transpose();
        assert_eq!(m.data.len(),5);
        assert_eq!(m.data[0].len(),3);
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
    fn test_prod(){
        let mut m = mat![
            i32:
                [1,2,3],
                [4,5,7]
        ];
        let n = mat![
            i32:
                [1,3],
                [5,7],
                [10,10]
        ];
        let res = mat![
            i32:
                [41,47],
                [99,117]
        ];
        m.prod(n);
        assert_eq!(m.data.len(),2);
        assert_eq!(m.data[0].len(),2);
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
        m.prod(n);
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
        m.prod(n);
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
        assert_eq!(adj.data.len(), 2);
        assert_eq!(adj.data[0].len(), 2);
        assert_eq!(adj.data.len(), res.data.len());
        assert_eq!(adj.data[0].len(), res.data[0].len());
        //panic!("adj {:?}",adj.data);
        for i in 0..adj.data.len() {
            for j in 0..adj.data[0].len() {
                assert_eq!(adj.data[i][j], res.data[i][j]);
            }
        }
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
    fn test_determinant_4x4(){
        let m = mat![
            i32:
                [2,3,4,1],
                [1,2,3,4],
                [4,1,2,3],
                [3,2,1,4]
        ];
        assert_eq!(m.determinant(),80);
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
