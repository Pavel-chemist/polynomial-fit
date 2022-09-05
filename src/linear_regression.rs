// receives list of coordinate pairs, returns coefficients for best fit linear or polynomial equation

use crate::matrix::Matrix;
use crate::helpers::{PixCoord, read_input, exp};

pub fn least_squares(/* coord_pairs: Vec<PixCoord> */) /* -> Vec<f64> */ {
  let mut coordinates: Vec<PixCoord> = Vec::with_capacity(16);
  let mut answer: String = String::from("y");

  let result: Vec<f64>;
  let power: usize;

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

  println!("Enter the power of polynomial (1-{})", coordinates.len() - 1);
  power = read_input("enter non-negative integer");

  result = least_squares_auto(coordinates, power);

  for i in 0..result.len() {
    println!("{} = {}", (i+97) as u8 as char, result[i]);
  }
  // return result;
}

fn least_squares_auto(coordinates: Vec<PixCoord>, polynomial_order: usize) -> Vec<f64>{
  let mut result: Vec<f64> = Vec::with_capacity(16);
  let mut matrix_a = Matrix::new(coordinates.len(), polynomial_order + 1);
  let mut vector_b: Matrix = Matrix::new(coordinates.len(), 1);

  for i in 0..coordinates.len() {
    for p in 0..=polynomial_order {
      let value = exp(coordinates[i].x, p as i32);
      matrix_a.set_value_at(matrix_a.columns() - p - 1, i, value);
    }

    vector_b.set_value_at(0, i, coordinates[i].y);
  }

  let matrix_a_t: Matrix = Matrix::transpose(&matrix_a);
  let normal_m: Matrix = Matrix::multiply(&matrix_a_t, &matrix_a);
  let moment_m: Matrix = Matrix::multiply(&matrix_a_t, &vector_b);

  let mut eq_system: Matrix = Matrix::new(normal_m.rows(), normal_m.columns() + 1);

  for j in 0..normal_m.rows() {
    // setting row (j) of eq_system
    for i in 0..normal_m.columns() {
      eq_system.set_value_at(i, j, normal_m.get_value_at(i, j));
    }

    // setting last column (i) of row of eq_system
    eq_system.set_value_at(normal_m.columns(), j, moment_m.get_value_at(0, j));
  }

  eq_system = Matrix::row_echelon_form(&eq_system);

  for i in 0..eq_system.rows() {
    result.push(eq_system.get_value_at(eq_system.columns() - 1, i));
  }

  return result;
}
