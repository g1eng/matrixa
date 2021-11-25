# About

matrixa.rs is a casual numeric calculation library, especially featuring on matrix and list manipulation.
At now it is experimental and it will support various mathematical and string manipulation for data, within the type of Matrix<T> which is implemented in vector-in-vector with dynamic length of row and column.
You can declare and manipulate two-dimensional matrices as an object like this:

```rust
// for i32
use matrixa::core::{List,Matrix};
use matrixa::mat;

let mut im = Matrix::<i32>::new();
im.push(vec![1,2,3,4,5]).unwrap().push(vec![5,6,7,8,9]).unwrap();
im.add(1).print();
im.mul(3).print();

// for f32
  
let mut fm = mat![f32: 
  [1.0,2.0,3.0],
  [2.0,3.0,4.0],
];
fm.debug()
    .push(vec![1.23,4.56,7.89])
    .unwrap();
fm.print();
```

# Concepts

This project is an experimental dog food that probably tastes bad for you.

The project aims to be:

* casual, easy-to-use and easy-to-understand its usage and behavior
* well-documented and maintainable for human after 10 years ahead. (just joking)
* human-friendly, with syntax sugars like +, - operators or `mat![T]`.

# Features

* You can create Matrix<T> instance for matrix declaration and its manipulation.
* `Matrix::<T>::new()` or easy-to-use `!mat[T]` macro for the constructor.
* A matrix instance can be typed with i32 (or other integer types), f32 (or other float types), and sized string (&str) or String.
* Builtin integrity checker and push or merge mechanism for panic-less append of rows or columns
* It implements Iterator. You can iterate data with a representation, such as `for d in matrix` 
* Almost all manipulation below results a new instance which can be mutable to the next operation.

### supported numerical operations
  - addition (with scalar or matrix)
  - subtract (with scalar or matrix)
  - product  (with scalar or matrix)
  - hadamard product (with matrix)
  - divide  (with scalar)
  - residue  (with scalar integer)

### supported mathematical operators
  - addition(+) 
  - subtract(-) 
  - product(\*)

### supported matrices to generate for:
  - inverse matrix
  - identity matrix
  - adjugate matrix

### supported calculators:
  - determinant
  - regular matrix detector
  - trace

### basic manipulator or formatters:
  - row_replace
  - col_replace
  - transpose
  - fill_zero (for number matrices)
  - resize
