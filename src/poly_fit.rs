// fit the data with polynomial of order (num_points-1)
use crate::matrix::Matrix;
use crate::matrix_fn::{row_echelon_form};
use crate::helpers::{PixCoord, exp};

pub fn polynomial_fit (coordinates: Vec<PixCoord> ) -> Vec<f64> {
//	given number of coordinate pairs, output list of coefficients corresponding to coord pair number
//length should not be less than 2 (?)
    let length: usize = coordinates.len();


    let mut coefficients: Vec<f64> = vec![0.0; length]; //number of coefficients equals number of coordinate pairs given
    
    //create matrix to be processed to Row Echelon Form
    //length+1 by length
    let mut matrix: Matrix = Matrix::new(length, length + 1);
    /*
    
    Matrix to solve:
    
    1		X1		X1^2	Y1			1	0	0	C
    1		X2		X2^2	Y2	===>	0	1	0	B
    1		X3		X3^2	Y3			0	0	1	A
    
    */

    println!("Matrix has {} rows and {} columns", matrix.rows(), matrix.columns());

    for j in 0..matrix.rows() { //matrix height
        for i in 0..(matrix.columns() - 1) {
            let exponent: i32 = i as i32;
            matrix.set_value_at(i, j, exp( coordinates[j].x, exponent ));
        }

        matrix.set_value_at(matrix.columns() - 1, j, coordinates[j].y);
    }
    
    matrix = row_echelon_form(&matrix);
    
    for i in 0..matrix.rows() {
        coefficients[length - 1 - i] = matrix.get_value_at(matrix.columns() - 1, i);
    }
    
    return coefficients;
}