// fit the data with polynomial of order (num_points-1)

#[path = "matrix.rs"] mod matrix;
#[path = "helpers.rs"] mod helpers;

use matrix::row_echelon_form;
use crate::helpers::{PixCoord, exp};

pub fn polynomial_fit (coordinates: Vec<PixCoord> ) -> Vec<f64> {
//	given number of coordinate pairs, output list of coefficients corresponding to coord pair number
//length should not be less than 2 (?)
    let length: usize = coordinates.len();

    let matrix_size: usize = (length + 1) * length;
    
    let mut coefficients: Vec<f64> = vec![0.0; length]; //number of coefficients equals number of coordinate pairs given
    
    //create matrix to be processed to Row Echelon Form
    //length+1 by length
    let mut matrix: Vec<f64> = vec![0.0; matrix_size];
    /*
    
    Matrix to solve:
    
    1		X1		X1^2	Y1			1	0	0	C
    1		X2		X2^2	Y2	===>	0	1	0	B
    1		X3		X3^2	Y3			0	0	1	A
    
    */

    for j in 0..length { //matrix height
        for i in 0..length {
            let exponent: i32 = i as i32;
            matrix[j*(length+1) + i] = exp( coordinates[j].x, exponent );
        }
        matrix[j*(length+1) + length] = coordinates[j].y;
    }
    
    matrix = row_echelon_form(matrix, length, length+1);
    
    for i in 0..length {
        coefficients[length - 1 - i] = matrix[i*(length+1) + length];
    }
    
    return coefficients;
}