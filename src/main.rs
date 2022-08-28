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
    let mut matrix: Vec<f64> = Vec::with_capacity(0);

    println!("If you want row echelon form, enter y");
    let answer: String = read_input("");

    if &answer == "y" {
        println!("ROW ECHELON FORM\n\nEnter matrix height:");
    
        let rows: usize = read_input("please, enter positive integer number");
    
        draw_matrix(&matrix, rows);
    
        matrix = populate_matrix(rows);
    
        matrix = row_echelon_form(matrix, rows, rows+1);
    
        println!("matrix that is in reduced row echelon form:");
    
        draw_matrix(&matrix, rows);
    } else {
        println!("\nFitting data with polynomial:\nenter the number of coordinate pairs:");    
        let coord_pairs_num: usize = read_input("");
    
        let mut coord_pairs: Vec<PixCoord> = Vec::with_capacity(coord_pairs_num);
    
        for i in 0..coord_pairs_num {
            println!("pair {}", i+1);
            println!("Enter x coordinate: ");
            let x: f64 = read_input("");
    
            println!("Enter y coordinate: ");
            let y: f64 = read_input("");
    
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

fn read_input<T: std::str::FromStr>(error_message: &str) -> T {
    // generic function that reads input and checks for type (number or text)
    // the error handling should be here
    let mut input: String = String::new();
    let result: T;
    let error: &str;

    if error_message.trim() == "" {
        error = "use correct value type, e.g. number";
    } else {
        error = error_message;
    }

    loop {
        io::stdin()
        .read_line(&mut input)
        .expect("failed to read the line");

        input = input.trim().to_string();

        match input.parse::<T>() {
            Ok(parsed_value) => {
                result = parsed_value;
                break;
            },
            Err(_) => {
                println!("{}", error);
                input = String::new();
                continue;
            },
        };
    }

    return result;
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
