pub struct Array2D<T> {
    pub shape: (u32, u32),
    pub array: Vec<Vec<T>>
}

impl<T> Array2D<T> {
    pub fn new(shape: (u32, u32)) -> Self {
        let mut array: Vec<Vec<T>> = Vec::with_capacity(shape.0 as usize);
        for _ in 0..shape.0 {
            array.push(Vec::with_capacity(shape.1 as usize));
        }
        return Self {
            shape,
            array
        }
    }

    pub fn ones(shape: (u32, u32)) -> Array2D<i32> {
        let mut array: Vec<Vec<i32>> = Vec::with_capacity(shape.0 as usize);
        for _ in 0..shape.0 {
            array.push(vec![1; shape.1 as usize]);
        }
        return Array2D {
            shape,
            array
        }
    }
 
    pub fn zeroes(shape: (u32, u32)) -> Array2D<i32> {
        let mut array: Vec<Vec<i32>> = Vec::with_capacity(shape.0 as usize);
        for _ in 0..shape.0 {
            array.push(vec![0; shape.1 as usize]);
        }
        return Array2D {
            shape,
            array
        }
    }

    pub fn from(array: Vec<Vec<T>>) -> Self {
        let shape = (array.len() as u32, array[0].len() as u32);
        for vec in &array {
            if vec.len() != shape.1 as usize {
                panic!("[ERROR] Not all rows of 2D array are of equal length");
            }
        }
        return Self {
            shape,
            array
        }
    }

    pub fn size(&self) -> u32 {
        self.shape.0 * self.shape.1
    }
}

