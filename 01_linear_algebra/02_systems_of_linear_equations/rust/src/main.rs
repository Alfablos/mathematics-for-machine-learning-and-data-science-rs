


use ndarray::{array, Array, Array1, Array2, ArrayBase, Axis, Ix2, OwnedRepr};

use ndarray_linalg::{Determinant, Solve};

#[allow(non_snake_case)]


fn main() {
  one_solution();
  println!();
  println!();
  no_solutions();
  println!();
  println!();
  infinite_solutions();
}

fn infinite_solutions() {
  println!("Solve:\n{{\n  -x + 3y = 7;\n  3x - 9y = 21;\n}}");

  let A: Array2<f64> = array![[-1., 3.], [3., -9.]];
  let b = array![7., -21.];
  if A.is_square() {
    let A_det = A.det().unwrap().round();
    println!("A determinant: {}", A_det);
    if A_det != 0_f64 {
      let x = A.solve_into(b.clone()).unwrap().round();
      println!("The system has a unique solution: {x}")
    }
  } else {
    println!("A determinant: not square")
  }
}

fn no_solutions() {
  println!("Solve:\n{{\n  -x + 3y = 7;\n  3x - 9y = 1;\n}}");
  
  let A: Array2<f64> = array![[-1., 3.], [3., -9.]];
  let b = array![7., 9.];
  if A.is_square() {
    let A_det = A.det().unwrap().round();
    println!("A determinant: {}", A_det);
    if A_det != 0_f64 {
      let x = A.solve_into(b.clone()).unwrap().round();
      println!("The system has a unique solution: {x}")
    }
  } else {
    println!("A determinant: not square")
  }
}


fn one_solution() {
  println!("Solve:\n{{\n  -x + 3y = 7;\n  3x + 2y = 1;\n}}");

  let A: Array2<f64> = array![[-1., 3.], [3., 2.]];
  println!("A:\n{}", &A);

  let b: Array1<f64> = array![7., 1.];
  println!("B:\n{}", &b);


  if A.is_square() {
    let A_det = A.det().unwrap().round();
    println!("A determinant: {}", A_det);
    if A_det != 0_f64 {
      let x = A.solve_into(b.clone()).unwrap().round();
      println!("The system has a unique solution: {x}")
    }
  } else {
    println!("A determinant: not square")
  }

  println!();
  println!();
  println!();

  let reshaped_b = b.insert_axis(Axis(1));
  println!("reshaped_b:\n{}", &reshaped_b);

  let A_augmented = ndarray::concatenate(Axis(1), &[A.view(), reshaped_b.view()]).unwrap();

  println!("A_augmented:\n{}", &A_augmented);

  println!();
  println!("Gauss elimination!");
  let mut A_augmented = A_augmented.clone();  // I need to mutate this one


  // Must send mutable borrows out of scope if I want to print!
  println!("1. R2 = 3 * R1 + R2\n");
  {
    let A_augmented_view = A_augmented.view_mut();
    let (r1_slice, mut r2_slice) = A_augmented_view.split_at(Axis(0), 1);

    let r1 = r1_slice.row(0);
    let mut r2 = r2_slice.row_mut(0);   // Requires the matrix to be mut
    r2.scaled_add(3f64, &r1);
  }

  println!("A_augmented:\n{}\n", &A_augmented);

  println!("2. R2 = 1/11 * R2");
  {
    let mut A_augmented_view = A_augmented.view_mut();
    let mut r2 = A_augmented_view.row_mut(1);

    r2 *= (1f64/11f64) as f64;
  }

  println!("A_augmented:\n{}\n", &A_augmented);

  println!("3. R1 = -1 * R1");
  {
    let mut A_augmented_view = A_augmented.view_mut();
    let mut r1 = A_augmented_view.row_mut(0);
    r1 *= -1f64;
  }

  println!("A_augmented (Row Echelon Form):\n{}\n", &A_augmented);

  println!("4. R1 = R2 + 7/2 * R2");
  {
    let A_augmented_view = A_augmented.view_mut();
    let (mut r1_slice, r2_slice) = A_augmented_view.split_at(Axis(0), 1);
    r1_slice.scaled_add(7f64/2f64, &r2_slice);
  }
  println!("A_augmented:\n{}\n", &A_augmented);

  println!("R1 = R1 + (-1/2) * R2");
  {
    let A_augmented_view = A_augmented.view_mut();
    let (mut r1_slice, r2_slice) = A_augmented_view.split_at(Axis(0), 1);
    r1_slice.scaled_add(-1f64/2f64, &r2_slice);
  }

  println!("A_augmented:\n{}\n", &A_augmented);

  println!("x1 = {}", A_augmented.get((0,2)).unwrap());
  println!("x2 = {}", A_augmented.get((1,2)).unwrap());
}