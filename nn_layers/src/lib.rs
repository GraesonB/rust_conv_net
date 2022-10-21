#[cfg(test)]
mod tests;
mod fully_connected;
use num::{Num, Zero};

use matrix_operations::*;
use model::*;

pub trait Forward<T: Num> {
    fn forward(input: Array2D<T>) -> Array2D<T>;
}
pub trait Backward<T: Num> {
    fn backward(input: Array2D<T>) -> Array2D<T>;
}

pub trait UpdateWeights {
    fn update_weights(model_hparams: ModelHParams);
}

