use super::{ Backward };

pub struct FCLayer {
    cache: Array2D,
    weights: Array2D,
    biases: Array2D,
    weight_grads: Array2D,
    bias_grads: Array2D
}

