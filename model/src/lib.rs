#[cfg(test)]
mod tests;


pub struct ModelHParams{
    learning_rate : f32,
    epochs : i32,
    batch_size : i32,
    beta_1 : f32, // commonly 0.9
    beta_2 : f32, // commonly 0.999
}