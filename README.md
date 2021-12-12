# About

matrixa.rs is an experimental linear algebra library, especially featuring on matrix, list manipulation on Rust.

It supports mathematical and string manipulation for data, within the type of `Matrix<T>` which holds the payload in vector-in-vector with dynamic length of row and column.
You can declare and manipulate two-dimensional matrices as an object like this:

```rust
use matrixa::core::Matrix;
use matrixa::mat;

// i32
let mut im = Matrix::<i32>::new();
im.push(vec![1,2,3,4,5]).unwrap().push(vec![5,6,7,8,9]).unwrap();
im.add(1).print();
im.mul(3).print();

// f32
let fm1 = mat![
  f32:
  [1.0,2.0,3.0],
  [2.0,3.0,4.0]
];
let fm2 = mat![
  f32:
  [1.1,2.2,3.3],
  [2.2,3.3,4.4]
];
let fm = fm1 + fm2;
fm.print()
```

# Concepts

This is a dog food that probably tastes bad for you.

The project aims to be:

* easy-to-use and easy-to-understand its usage and behavior
* well-documented and maintainable.
* human-friendly, with syntax sugars like +, - operators or `mat![T]`.

# Features

* You can create Matrix<T> instance for matrix declaration and its manipulation.
* `Matrix::<T>::new()` or easy-to-use `!mat[T]` macro for the constructor.
* A matrix instance can be typed with integer such as i32, floating point such as f32, bool, or reference to string literal (&str).
* Builtin integrity checker and push or merge mechanism for panic-less append of rows or columns
* It implements Clone. You can assign a matrix to another using `=` operator or generate clone instance with `clone()`.
* It implements Iterator. You can iterate data with a representation, such as `for d in matrix` 
* Almost all manipulation below results a new instance which can be mutable to the next operation.


## Core functionalities

### Core manipulator or formatters:
* new
* clone
* row_replace
* col_replace
* transpose
* fill_zero (for number matrices)
* resize

### Matchers
  - equal `=`
  - not equal `!=`
  
## Numerical operations

### Supported numerical operations

| category | operator | scalar | matrix | 
| --- | --- | --- | --- | 
| addition | + | O* | O |
| subtraction | - | O* | O |
| product | * | O* | O |
| hadamard product |  | | O* |
| division | / | O* | O |
| rem | % | O* | O |

* O: supported
* O*: supported via method

### supported matrices to generate for:
  - inverse matrix
  - identity matrix
  - adjugate matrix

### supported calculators:
  - determinant
  - regular matrix detector
  - trace

### supported string operation

WIP

