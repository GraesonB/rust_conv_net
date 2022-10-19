use super::*;

#[test]
fn ones_test() {
   let array: Array2D<i32> = Array2D::<i32>::ones((2,2));
   let test: Vec<Vec<i32>> = vec![vec![1,1], vec![1,1]];
   assert_eq!(array.array, test);
}

#[test]
fn zeroes_test() {
    let array: Array2D<i32> = Array2D::<i32>::zeroes((2,2));
    let test: Vec<Vec<i32>> = vec![vec![0,0], vec![0,0]];
    assert_eq!(array.array, test);
 }

 #[test]
fn array_from_test() {
    let array: Array2D<i32> = Array2D::from(vec![
        vec![1,2,3],
        vec![1,2,3]
    ]);
    let test: Vec<Vec<i32>> = vec![vec![1,2,3], vec![1,2,3]];
    assert_eq!(array.array, test);
 }

 #[test]
 fn dot_test() {
    let array1 = Array2D::from(vec![
        vec![2,3],
        vec![3,4]
    ]);
    let array2 = Array2D::from(vec![
        vec![1,3],
        vec![3,4]
    ]);
    let product = dot(array1, array2);
    assert_eq!(product.array, vec![vec![11,18], vec![15,25]]);
 }

 #[test]
 #[should_panic]
 fn incompatible_dot_test() {
    let array1 = Array2D::from(vec![
        vec![2,3],
        vec![3,4],
    ]);
    let array2 = Array2D::from(vec![
        vec![1,3],
        vec![3,4],
        vec![4,3]
    ]);
    dot(array1, array2);
 }

 #[test] 
 fn compatible_dot_test() {
    let array1 = Array2D::<i32>::ones((77,3));
    let array2 = Array2D::<i32>::ones((3,34));
    dot(array1, array2);
 }

 #[test]
 fn transpose_test() {
    let array = Array2D::from(vec![
        vec![1,2],
        vec![3,4],
        vec![5,6]
    ]);
    let array_transpose = Array2D::from(vec![
        vec![1,3,5],
        vec![2,4,6]
    ]);
    assert_eq!(array.transpose().array, array_transpose.array);
 }

 #[test]
 fn sum_vertical_test() {
    let array: Array2D<i32> = Array2D::<i32>::ones((3,3));
    let test: Vec<Vec<i32>> = vec![vec![3,3,3]];
    assert_eq!(array.sum_vertical().array, test);
 }

 #[test]
 fn sum_horizontal_test() {
    let array: Array2D<i32> = Array2D::<i32>::ones((3,3));
    let test: Vec<Vec<i32>> = vec![
        vec![3],
        vec![3],
        vec![3]];
    assert_eq!(array.sum_horizontal().array, test);
 }
