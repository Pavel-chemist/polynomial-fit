use std::io;

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


fn row_echelon_form(mut data: Vec<f64>, rows: usize, columns: usize) -> Vec<f64>{
    normalize_rows(&mut data, rows, columns);

    for i in 0..(rows-1) {
        subtract_row_down ( &mut data, rows, columns, i );
        normalize_rows ( &mut data, rows, columns );
    }
    
    for i in (0..rows).rev() {
        subtract_row_up (&mut data, columns, i );
    }
    
    for i in 0..(rows*columns) {
        if data[i] == -0.0 {
            data[i] = 0.0;
        }
    }

    return data;
}

fn normalize_rows(data: &mut Vec<f64>, rows: usize, columns: usize) {
    let mut leading_value: f64;
    let mut count: usize;
    
    for j in 0..rows {
        count = 0;
        leading_value = 0.0;

        while leading_value == 0.0 {
            leading_value = data[j * columns + count];
            count = count + 1;
        }

        for i in 0..columns {
            data[j*columns + i] = data[j*columns+i] / leading_value;
        }
    }
}

fn subtract_row_down (data: &mut Vec<f64>, rows: usize, columns: usize, row_to_subtract: usize)
{
    for j in (row_to_subtract + 1)..rows {
        for i in 0..columns {
            data[j*columns+i] = data[j*columns+i] - data[row_to_subtract*columns+i];
        }
    }
}

fn subtract_row_up (data: &mut Vec<f64>, columns: usize, row_to_subtract: usize)
{
    //find first non-zero value index
    let mut leading_value_index: usize = 0;
    let mut coefficient: f64;
    
    for i in 0..columns {
        if data[row_to_subtract*columns+i] != 0.0 {
            leading_value_index = i;
            break;
        }
    }
    
    //subtracting rows such that on leading_value_index result become zero
    for j in (0..row_to_subtract).rev() {
        coefficient = data[j*columns+leading_value_index] / data[row_to_subtract*columns+leading_value_index];

        for i in 0..columns {
            data[j*columns+i] = data[j*columns+i] - coefficient * data[row_to_subtract*columns+i];
        }
    }
    
}