use super::*;
use matrix_operations::*;

pub struct FCLayer<T> {
    cache: Array2D<T>,
    weights: Array2D<T>,
    biases: Array2D<T>,
    weight_grads: Array2D<T>,
    bias_grads: Array2D<T>
}

impl Forward<T> for FCLayer<T> {
    fn forward<T: Num>(&self, input: Array2D<T>) -> Array2D<T> {
        self.cache = input;
        return dot(input, self.weights) + self.biases;
    }
}

impl Backward<T> for FCLayer<T> {
    fn backward<T: Num>(&self, input: Array2D<T>) -> Array2D<T> {
        let m = self.cache.shape.0; // number of training examples
        self.weight_grads = (1/m) * dot(self.cache.transpose(), input);

    }
}
