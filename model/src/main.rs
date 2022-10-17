use matrix_operations::*;
use std::time::{Duration, Instant};


fn main() {
    let now = Instant::now();
    let array1 = Array2D::<i32>::ones((7500,15));
    let array2 = Array2D::<i32>::ones((15,10000));
    let product = dot(array1, array2);
    println!("{}", now.elapsed().as_millis());
}
