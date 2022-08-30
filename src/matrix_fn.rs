// module with functions for calculating reduced row echelon form for matrices
use crate::helpers::read_input;
use crate::matrix::Matrix;

pub fn draw_matrix(matrix: &Matrix) {
    println!();

    for j in 0..matrix.rows() {
        for i in 0..matrix.columns() {
            if j * matrix.columns() + i < matrix.size() {
                let value = matrix.get_value_at(i, j);

                print!("{value}\t")
            } else {
                print!("0 \t");
            }
        }

        println!();
    }
}

pub fn populate_matrix(matrix: &mut Matrix) {
    let mut value: f64;

    println!("Enter the {} values for the matrix:", matrix.size());

    for j in 0..matrix.rows() {
        for i in 0..matrix.columns() {
            println!("value at i={}, j={}:", i, j);
        
            value = read_input("enter number");

            matrix.set_value_at(i,j, value);

            draw_matrix(&matrix);
        }
    }

    println!("matrix is populated!");
}
/* 
pub fn row_echelon_form(matrix: &Matrix) -> Matrix {
    let mut result: Matrix = Matrix::copy(matrix);

    // make all rows in matrix start with 1.0
    normalize_rows(&mut result);

    // convert matrix to row echelon form
    for i in 0..(result.rows() - 1) {
        subtract_row_down ( &mut result, i );
        normalize_rows ( &mut result );
    }
  
    // convert echelon matrix to reduced row echelon form
    for i in (0..result.rows()).rev() {
        subtract_row_up (&mut result, i );
    }

    // dealing with -0.0  values
    for j in 0..result.rows() {
        for i in 0..result.columns() {    
            if result.get_value_at(i, j) == -0.0 {
                result.set_value_at(i, j, 0.0);
            }
        }
    }

    return result;
}
 */
