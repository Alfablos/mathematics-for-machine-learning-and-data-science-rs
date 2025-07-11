use ndarray::{array, s, Array, Array1, Array2, ArrayBase, Dim, Ix1, OwnedRepr};

// https://docs.rs/ndarray/latest/ndarray/doc/ndarray_for_numpy_users/index.html

fn main() {
  let onedim: ArrayBase<OwnedRepr<u8>, Ix1> = array![1, 2, 3];
  println!("onedim = {onedim}");

  // 0, 1, 2, 3 as f64
  let onedim_range: ArrayBase<OwnedRepr<f64>, Ix1> = Array::range(0., 4., 1.);
  println!("onedim_range = {onedim_range}");

  // 1, 2, 3 as f64
  let onedim_range_from1: ArrayBase<OwnedRepr<f64>, Ix1> = Array::range(1., 4., 1.);
  println!("onedim_range_from1 = {onedim_range_from1}");

  let onedim_range_with_step = Array::range(1., 30., 2.); // step is 2; all odds < 30
  println!("onedim_range_with_step = {onedim_range_with_step}");

  let onedim_equally_spaced = Array::linspace(0., 20., 5);
  println!("onedim_equally_spaced = {onedim_equally_spaced}");
  
  let onedim_zeros: ArrayBase<OwnedRepr<u8>, Ix1> = Array::<u8, Ix1>::zeros((5)); // MUST provide the generic types
  println!("onedim_zeros = {onedim_zeros}");
  
  // Multidimensional //
  let arr = array![1., 2., 3., 4., 5., 6.];
  
  // reshape-like
  let multi = Array::from_shape_vec((2, 2), arr.to_vec());
  
  // Math operations //
  let vec1 = array![2,3];
  let vec2 = array![3,4];
  
  println!("{} + {} = {}", &vec1, &vec2, vec1.clone() + vec2.clone());
  println!("{} - {} = {}", &vec1, &vec2, vec1.clone() - vec2.clone());
  
  let pow = |vec: ArrayBase<OwnedRepr<i32>, Ix1>,n: u32|
      vec.mapv(|v| v.pow(n));
  let n = 3;
  println!("{vec1}^{n} = {}", pow(vec1.clone(), n));
  
  println!("{vec1} dot product {vec2} = {}", vec1.dot(&vec2));
  println!("Scalar by vector: {vec1} * 2 = {}", vec1.clone() * 2);
  
  
  // Indexing in 1D //
  let arr = array![[1., 2., 3.], [4., 5., 6.,], [7., 8., 9.]]; // 3x2
  let last_member_of_last_arr = arr[[2, 2]];
  println!("last_member_of_last_arr = {last_member_of_last_arr}");
  
  let first_group = arr.slice(s![0, ..]);
  println!("first_group = {}", first_group);
  
}
