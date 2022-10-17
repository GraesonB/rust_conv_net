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
    let product = dot(array1, array2);
 }

 #[test] 
 fn compatible_dot_test() {
    let array1 = Array2D::<i32>::ones((7267,3));
    let array2 = Array2D::<i32>::ones((3,4534));
    let product = dot(array1, array2);

 }

