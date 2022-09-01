// declaring modules used
// declaring them in main makes it possible to other modules cross-use structs and functions
mod matrix;
mod matrix_fn;
mod helpers;
mod poly_fit;
mod linear_regression;

// declaring functionality of used modules
use helpers::{PixCoord, read_input};
use matrix::{Matrix, sort_rows};
use matrix_fn::{draw_matrix, populate_matrix};
use poly_fit::polynomial_fit;
use linear_regression::least_squares;

fn main() {
    let mut matrix: Matrix;

    println!("Options:\n row echelon form --- ref,\n polynomial fit --- pf,\n linear regression --- lr,\n matrix sort --- ms");
    let answer: String = read_input("");

    if &answer == "ref" {
        println!("ROW ECHELON FORM\n\nEnter matrix height:");
    
        let rows: usize = read_input("please, enter positive integer number");

        matrix = Matrix::new(rows, rows+1);
    
        draw_matrix(&matrix);
    
        populate_matrix(&mut matrix);
    
        matrix = Matrix::row_echelon_form(&matrix);
    
        println!("matrix that is in reduced row echelon form:");
    
        draw_matrix(&matrix);

    } else if &answer == "ms" {
        println!("MATRIX SORT\n\nEnter matrix height:");
    
        let rows: usize = read_input("please, enter positive integer number");

        matrix = Matrix::new(rows, rows+1);
    
        draw_matrix(&matrix);
    
        populate_matrix(&mut matrix);
    
        sort_rows(&mut matrix);
    
        println!("matrix with rows sorted:");
    
        draw_matrix(&matrix);

    } else if &answer == "pf"{
        println!("\nFitting data with polynomial:\nenter the number of coordinate pairs:");    
        let coord_pairs_num: usize = read_input("please, enter positive integer number");
    
        let mut coord_pairs: Vec<PixCoord> = Vec::with_capacity(coord_pairs_num);
    
        for i in 0..coord_pairs_num {
            println!("pair {}", i+1);
            println!("Enter x coordinate: ");
            let x: f64 = read_input("enter a number");
    
            println!("Enter y coordinate: ");
            let y: f64 = read_input("enter a number");
    
            let coordinate: PixCoord = PixCoord { x: x, y: y };
    
            coord_pairs.push(coordinate);
        }
    
        let coefficients: Vec<f64> = polynomial_fit(coord_pairs);
    
        println!("Coefficients for polynomial:");
        for i in 0..coefficients.len() {
            print!("{}={}, ", (i+97) as u8 as char,coefficients[i]);
        }
    } else if &answer == "lr" {
        least_squares();
    }

    println!("\n\nBYE!");
}
