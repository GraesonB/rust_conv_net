pub struct Array2D<T> {
    pub shape: (usize, usize),
    pub array: Vec<Vec<T>>
}

impl<T: Copy> Array2D<T> {
    pub fn new(shape: (usize, usize)) -> Self {
        let mut array: Vec<Vec<T>> = Vec::with_capacity(shape.0);
        for _ in 0..shape.0 {
            array.push(Vec::with_capacity(shape.1));
        }
        return Self {
            shape,
            array
        }
    }

    pub fn ones(shape: (usize, usize)) -> Array2D<i32> {
        let mut array: Vec<Vec<i32>> = Vec::with_capacity(shape.0);
        for _ in 0..shape.0 {
            array.push(vec![1; shape.1]);
        }
        return Array2D {
            shape,
            array
        }
    }
 
    pub fn zeroes(shape: (usize, usize)) -> Array2D<i32> {
        let mut array: Vec<Vec<i32>> = Vec::with_capacity(shape.0);
        for _ in 0..shape.0 {
            array.push(vec![0; shape.1]);
        }
        return Array2D {
            shape,
            array
        }
    }

    pub fn from(array: Vec<Vec<T>>) -> Self {
        let shape = (array.len(), array[0].len());
        for vec in &array {
            if vec.len() != shape.1 {
                panic!("[ERROR] Not all rows of 2D array are of equal length");
            }
        }
        return Self {
            shape,
            array
        }
    }

    pub fn transpose(&self) -> Self {
        let shape = (self.shape.1, self.shape.0);
        // initialize output array
        let mut array: Array2D<T> = Array2D::new(shape);

        for row in 0..self.shape.1 { // there will be as many rows as self has columns
            for col in 0..self.shape.0 { // there will be as many columns as self has rows
                // push self's column's as output's rows.
                array.array[row].push(self.array[col][row]);
            }
        }
        array

    }

    pub fn size(&self) -> usize {
        self.shape.0 * self.shape.1
    }

}

