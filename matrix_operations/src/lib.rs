use std::ops::AddAssign;

#[cfg(test)] // cfg is conditional compilation
mod tests;
mod hdarray;
use num::{Num, Zero};

pub use self::hdarray::Array2D;


pub fn dot<T>(a: Array2D<T>, b:Array2D<T>) -> Array2D<T> 
    where T: Num + AddAssign + Copy + Zero
{
    if a.shape.1 != b.shape.0 {
        panic!("[ERROR] Incomaptible matrix shapes")
    }
    // initialize array to be returned
    let mut product: Array2D<T> = Array2D::new((a.shape.0, b.shape.1));
    // i and j are for looping through each position in the product array
    for row in 0..product.shape.0 {
        for col in 0..product.shape.1 {
            // initialize sum and loop through a's row and b's column adding each product to sum
            let mut sum: T = T::zero();
            for i in 0..a.shape.1 {
                // i moves along a's row and b's column. once finished, it moves over one column for b and repeats the process
                // until the next row, starting at the first column again.
                sum += a.array[row][i] * b.array[i][col];
            }
            // append sum to the current row of the array. "col" isn't needed because we always just push it to the next value
            // if we initialized product using Array2D::zeroes we could probably do product.array[row][col] = sum;
            product.array[row].push(sum);
        }
    }
    product
}