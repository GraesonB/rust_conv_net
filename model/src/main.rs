use matrix_operations::*;
use std::time::{Duration, Instant};


fn main() {
    let now = Instant::now();
    let array1 = Array2D::<i32>::ones((7500,15));
    let array2 = array1.transpose();
    let product = dot(array1, array2);
    println!("{:?}", &product.array);
}
