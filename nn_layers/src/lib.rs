#[cfg(test)]
mod tests;

use mat_ops::*;

pub trait Forward {
    fn forward(input: Array2D) -> Array2D;
}
pub trait Backward {
    fn backward(input: Array2D) -> Array2D;
}

pub trait UpdateWeights {
    fn update_weights(model_hparams: ModelHParams);
}

