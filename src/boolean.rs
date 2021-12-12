use crate::core::Matrix;
use crate::mat;
use std::ops::{BitAnd, BitOr, BitXor, Not};

/// 論理積 / logical product
///
/// ２つの論理行列の論理積を計算し、新規Matrixインスタンスとして返却する。
///
impl<T: Copy + std::ops::BitAnd<Output = T>> BitAnd for Matrix<T> {
    type Output = Self;
    fn bitand(self, other: Self) -> Self::Output {
        let mut res = self.clone();
        for i in 0..res.data.len() {
            for j in 0..res.data[i].len() {
                res.data[i][j] = res.data[i][j] & other.data[i][j]
            }
        }
        res
    }
}

/// 論理和 / logical sum
///
/// 2つの論理行列の論理和を計算し、新規Matrixインスタンスとして返却する
///
impl<T: Copy + std::ops::BitOr<Output = T>> BitOr for Matrix<T> {
    type Output = Self;
    fn bitor(self, other: Self) -> Self::Output {
        let mut res = self.clone();
        for i in 0..res.data.len() {
            for j in 0..res.data[i].len() {
                res.data[i][j] = res.data[i][j] | other.data[i][j]
            }
        }
        res
    }
}

/// 排他的論理和 / exclusive logical sum
///
/// 2つの論理行列の排他的論理和を計算し、新規Matrixインスタンスとして返却する
///
impl<T: Copy + std::ops::BitXor<Output = T>> BitXor for Matrix<T> {
    type Output = Self;
    fn bitxor(self, other: Self) -> Self::Output {
        let mut res = self.clone();
        for i in 0..res.data.len() {
            for j in 0..res.data[i].len() {
                res.data[i][j] = res.data[i][j] ^ other.data[i][j]
            }
        }
        res
    }
}

/// 否定 / negation
///
/// 論理行列の各要素について真偽値を反転し、新規Matrixインスタンスとして返却する。
///
impl<T: Copy + std::ops::Not<Output = T>> Not for Matrix<T> {
    type Output = Self;
    fn not(self) -> Self::Output {
        let mut res = self.clone();
        for i in 0..res.data.len() {
            for j in 0..res.data[i].len() {
                res.data[i][j] = !res.data[i][j]
            }
        }
        res
    }
}

#[cfg(test)]
mod tests_matrix_boolean_operator {
    use crate::core::Matrix;
    use crate::mat;

    #[test]
    fn test_bitand(){
        let b = mat![
            bool:
            [true,true],
            [false,true]
        ];
        let v = mat![
            bool:
            [false,true],
            [false,true]
        ];
        let res = mat![
            bool:
            [false,true],
            [false,true]
        ];
        assert_eq!((b & v) == res, true);
    }

    #[test]
    fn test_bitor(){
        let b = mat![
            bool:
            [true,true],
            [false,true]
        ];
        let v = mat![
            bool:
            [false,true],
            [false,true]
        ];
        let res = mat![
            bool:
            [true,true],
            [false,true]
        ];
        assert_eq!((b | v) == res, true);
    }

    #[test]
    fn test_bitxor(){
        let b = mat![
            bool:
            [true,true],
            [false,true]
        ];
        let v = mat![
            bool:
            [false,true],
            [false,true]
        ];
        let res = mat![
            bool:
            [true,false],
            [false,false]
        ];
        assert_eq!((b ^ v) == res, true);
    }

    #[test]
    fn test_not(){
        let b = mat![
            bool:
            [true,true],
            [false,true]
        ];
        let res = mat![
            bool:
            [false,false],
            [true,false]
        ];
        assert_eq!(!b == res, true);
    }
}