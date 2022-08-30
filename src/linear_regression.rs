// receives list of coordinate pairs, returns coefficients for best fit linear equation
// later add polynomial

use crate::matrix::Matrix;
use crate::helpers::{PixCoord, read_input};

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

  let mut matrix_a = Matrix::new(coordinates.len(), 2);
  let mut vector_b: Matrix = Matrix::new(coordinates.len(), 1);

  for i in 0..coordinates.len() {
    matrix_a.set_value_at(0, i, coordinates[i].x);
    matrix_a.set_value_at(0, i, 1.0);
    vector_b.set_value_at(0, i, coordinates[i].y);
  }

  let matrix_a_t: Matrix = Matrix::transpose(&matrix_a);

  let normal_m: Matrix = Matrix::multiply(&matrix_a, &matrix_a_t);

  let mut eq_system: Matrix = Matrix::new(normal_m.rows(), normal_m.columns() + 1);

  for j in 0..normal_m.rows() {
    for i in 0..normal_m.columns() {
      eq_system.set_value_at(i, j, normal_m.get_value_at(i, j));
    }

    eq_system.set_value_at(normal_m.columns(), j, vector_b.get_value_at(0, j));
  }

  eq_system = Matrix::row_echelon_form(&eq_system);

  for i in 0..eq_system.rows() {
    result[i] = eq_system.get_value_at(0, i);
  }


  for i in 0..result.len() {
    println!("{}: {}", i, result[i]);
  }
  // return result;
}
