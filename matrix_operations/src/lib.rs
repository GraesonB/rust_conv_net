use std::{fmt::Debug, ops::{Mul, Add, AddAssign}};

#[cfg(test)] //cfg is conditional compilation
mod tests;
mod hdarray;
use num::{Num, Zero};

pub use self::hdarray::Array2D;


pub fn dot<T: Num + AddAssign + Copy + Zero>(a: Array2D<T>, b:Array2D<T>) -> Array2D<T> {
    if a.shape.1 != b.shape.0 {
        panic!("[ERROR] Incomaptible matrix shapes")
    }
    let mut product: Array2D<T> = Array2D::new((a.shape.0, b.shape.1));
    for i in 0..product.shape.0 as usize {
        for j in 0..product.shape.1 as usize {
            let mut sum: T = T::zero();
            for w in 0..a.shape.1 as usize {
                sum += a.array[i][w] * b.array[w][j];
            }
            product.array[i].push(sum);
        }
    }
    product
}