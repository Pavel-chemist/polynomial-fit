mod row_echelon;

use std::io;
use row_echelon::row_echelon_form;

fn main() {
    let mut rows: String = String::new();
    let mut matrix: Vec<f64> = Vec::with_capacity(0);

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
