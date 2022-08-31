// receives list of coordinate pairs, returns coefficients for best fit linear equation
// later add polynomial

use crate::matrix::Matrix;
use crate::helpers::{PixCoord, read_input};
use crate::matrix_fn::draw_matrix;

pub fn least_squares(/* coord_pairs: Vec<PixCoord> */) /* -> Vec<f64> */ {
  let mut coordinates: Vec<PixCoord> = Vec::with_capacity(16);
  let mut result: Vec<f64> = vec![0.0; 2];
  let mut answer: String = String::from("y");

  println!("Enter the coordinates of point:");

  while answer == "y" {
    println!("Enter x coordinate: ");
    let x: f64 = read_input("Enter the number!");

    println!("Enter y coordinate: ");
    let y: f64 = read_input("Enter the number!");

    let point: PixCoord = PixCoord { x: x, y: y };

    coordinates.push(point);

    println!("Add another point? (y/n):");

    answer =read_input("Enter a string.");
  }

  println!("Entered coordinate pairs:");
  for i in 0..coordinates.len() {
    println!("x: {},\ty: {}", coordinates[i].x, coordinates[i].y);
  }

  let mut matrix_a = Matrix::new(coordinates.len(), 2);
  let mut vector_b: Matrix = Matrix::new(coordinates.len(), 1);

  for i in 0..coordinates.len() {
    matrix_a.set_value_at(0, i, coordinates[i].x);
    matrix_a.set_value_at(1, i, 1.0);
    vector_b.set_value_at(0, i, coordinates[i].y);
  }

  println!("matrix_a:");
  draw_matrix(&matrix_a);

  println!("vector_b:");
  draw_matrix(&vector_b);

  let matrix_a_t: Matrix = Matrix::transpose(&matrix_a);

  println!("matrix_a_t:");
  draw_matrix(&matrix_a_t);

  let normal_m: Matrix = Matrix::multiply(&matrix_a_t, &matrix_a);

  println!("normal_m = matrix_a_t * matrix_a");
  draw_matrix(&normal_m);

  let moment_m: Matrix = Matrix::multiply(&matrix_a_t, &vector_b);

  println!("moment_m = matrix_a_t * vector_b");
  draw_matrix(&moment_m);

  let mut eq_system: Matrix = Matrix::new(normal_m.rows(), normal_m.columns() + 1);

  println!("empty eq_system:");
  draw_matrix(&eq_system);

  for j in 0..normal_m.rows() {
    println!("setting row {} of eq_system", j+1);
    for i in 0..normal_m.columns() {
      eq_system.set_value_at(i, j, normal_m.get_value_at(i, j));
    }

    println!("setting last column of row {} of eq_system", j+1);
    eq_system.set_value_at(normal_m.columns(), j, moment_m.get_value_at(0, j));
  }

  println!("filled eq_system:");
  draw_matrix(&eq_system);

  eq_system = Matrix::row_echelon_form(&eq_system);

  println!("eq_system in reduced row echelon form:");
  draw_matrix(&eq_system);

  for i in 0..eq_system.rows() {
    result[i] = eq_system.get_value_at(eq_system.columns() - 1, i);
  }


  for i in 0..result.len() {
    println!("{} = {}", (i+97) as u8 as char, result[i]);
  }
  // return result;
}
