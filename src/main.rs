// declaring modules used
mod matrix;
mod helpers;
mod poly_fit;

// declaring functionality of used modules
use std::io;
use matrix::row_echelon_form;
use crate::helpers::PixCoord;
use poly_fit::polynomial_fit;

fn main() {
    let mut rows: String = String::new();
    let mut coord_pairs_num: String = String::new();
    let mut matrix: Vec<f64> = Vec::with_capacity(0);
    let mut answer: String = String::new();

    println!("If you want row echelon form, enter y");

    io::stdin()
        .read_line(&mut answer)
        .expect("failed to read the line");

    if &answer == "y\n" {
        println!("ROW ECHELON FORM\n\nEnter matrix height:");

        io::stdin()
            .read_line(&mut rows)
            .expect("failed to read the line");
    
        let rows: usize = rows.trim().parse().expect("Please type a number!");
    
        draw_matrix(&matrix, rows);
    
        matrix = populate_matrix(rows);
    
        matrix = row_echelon_form(matrix, rows, rows+1);
    
        println!("matrix that is in reduced row echelon form:");
    
        draw_matrix(&matrix, rows);
    } else {
        println!("\nFitting data with polynomial:\nenter the number of coordinate pairs:");

        io::stdin()
            .read_line(&mut coord_pairs_num)
            .expect("failed to read the line");
    
        let coord_pairs_num: usize = coord_pairs_num.trim().parse().expect("Please type a number!");
    
        let mut coord_pairs: Vec<PixCoord> = Vec::with_capacity(coord_pairs_num);
    
        for i in 0..coord_pairs_num {
            let mut x: String = String::new();
            let mut y: String = String::new();
    
            println!("pair {}", i+1);
            println!("Enter x coordinate: ");
            io::stdin()
            .read_line(&mut x)
            .expect("failed to read the line");
    
            let x: f64 = x.trim().parse().expect("Please type a number!");
    
            println!("Enter y coordinate: ");
            io::stdin()
            .read_line(&mut y)
            .expect("failed to read the line");
    
            
            let y: f64 = y.trim().parse().expect("Please type a number!");
    
            let coordinate: PixCoord = PixCoord { x: x, y: y };
    
            coord_pairs.push(coordinate);
        }
    
        let coefficients: Vec<f64> = polynomial_fit(coord_pairs);
    
        println!("Coefficients for polynomial:");
        for i in 0..coefficients.len() {
            print!("{}={}, ", (i+97) as u8 as char,coefficients[i]);
        }
    }

    println!("\n\nBYE!");
}




fn draw_matrix(data: &Vec<f64>, rows: usize) {
    let columns: usize = rows + 1;
    let mut value: f64;

    println!();

    for j in 0..rows {
        for i in 0..columns {
            if j*columns + i < data.len() {
                value = data[j*columns + i];

                print!("{value}\t")
            } else {
                print!("0 \t");
            }
        }

        println!();
    }
}

fn populate_matrix(rows: usize) -> Vec<f64>{
    let capacity: usize = rows * (rows + 1);

    let mut data: Vec<f64> = Vec::with_capacity(capacity);
    let mut input_value: String = String::new();
    let mut value: f64;

    for index in 0..capacity {
        println!("value #{index}");

        io::stdin()
            .read_line(&mut input_value)
            .expect("failed to read the line");
        
        value = input_value.trim().parse().expect("Please type a number!");

        data.push(value);

        draw_matrix(&data, rows);

        input_value = String::from("");
    }

    let array_size = data.len();
    println!("matrix is populated!\nThe number of values in matrix is {array_size}");

    return data;
}
