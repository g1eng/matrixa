//! 基本的な使い方
//!
//! ```rust
//! use matrixa::core::Matrix;
//! use matrixa::mat;
//!
//! // i32
//! let mut im = Matrix::<i32>::new();
//! im.push(vec![1,2,3,4,5]).unwrap().push(vec![5,6,7,8,9]).unwrap();
//! im.add(1).print();
//! im.mul(3).print();
//!
//! // f32
//! let fm1 = mat![
//!   f32:
//!   [1.0,2.0,3.0],
//!   [2.0,3.0,4.0]
//! ];
//! let fm2 = mat![
//!   f32:
//!   [1.1,2.2,3.3],
//!   [2.2,3.3,4.4]
//! ];
//! let fm = fm1 + fm2;
//! fm.print()
//! ```