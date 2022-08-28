// module with functions for calculating reduced row echelon form for matrices
use crate::helpers::read_input;

pub struct Matrix {
    rows: usize,
    columns: usize,
    data: Vec<f64>,
  }
  
  impl Matrix {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            rows,
            columns,
            data: vec![0.0; rows * columns],
        }
    }
  
    pub fn copy(matrix_to_be_copied: &Matrix) -> Matrix {
        let mut copied_matrix: Matrix = Matrix::new(matrix_to_be_copied.rows, matrix_to_be_copied.columns);
  
        for i in 0..matrix_to_be_copied.data.len() {
            copied_matrix.data[i] = matrix_to_be_copied.data[i];
        }
  
        return copied_matrix;
    }

    pub fn get_value_at(&self, column: usize, row: usize) -> f64 {
        if column < self.columns && row < self.rows {
            return self.data[row * self.columns + column];
        } else {
            println!("Get value Error! Indices are outside of bounds!");
            if column >= self.columns {
                println!("Trying to get at column {} while there are {} columns.", column+1, self.columns);
            }
            
            if row >= self.rows {
                println!("Trying to get at row {} while there are {} rows.", row+1, self.rows);
            }
            return 0.0;
        }
    }

    pub fn set_value_at(&mut self, column: usize, row: usize, value: f64) {
        if column < self.columns && row < self.rows {
            self.data[row * self.columns + column] = value;
        } else {
            println!("Set value Error! Indices are outside of bounds!");
            if column >= self.columns {
                println!("Trying to set at column {} while there are {} columns.", column+1, self.columns);
            }
            
            if row >= self.rows {
                println!("Trying to set at row {} while there are {} rows.", row+1, self.rows);
            }
        }
    }

    pub fn size(&self) -> usize {
        return self.data.len();
    }

    pub fn rows(&self) -> usize {
        return self.rows;
    }

    pub fn columns(&self) -> usize {
        return self.columns;
    }
  }

pub fn draw_matrix(matrix: &Matrix) {
    println!();

    for j in 0..matrix.rows {
        for i in 0..matrix.columns {
            if j*matrix.columns + i < matrix.size() {
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

    for j in 0..matrix.rows {
        for i in 0..matrix.columns {
            println!("value at i={}, j={}:", i, j);
        
            value = read_input("enter number");

            matrix.set_value_at(i,j, value);

            draw_matrix(&matrix);
        }
    }

    println!("matrix is populated!");
}

pub fn row_echelon_form(matrix: &Matrix) -> Matrix {
    let mut result: Matrix = Matrix::copy(matrix);

    // make all rows in matrix start with 1.0
    normalize_rows(&mut result);

    // convert matrix to row echelon form
    for i in 0..(result.rows - 1) {
        subtract_row_down ( &mut result, i );
        normalize_rows ( &mut result );
    }
  
    // convert echelon matrix to reduced row echelon form
    for i in (0..result.rows).rev() {
        subtract_row_up (&mut result, i );
    }
  
    for i in 0..result.size() {
        if result.data[i] == -0.0 {
            result.data[i] = 0.0;
        }
    }

    return result;
}

fn normalize_rows(matrix: &mut Matrix) {
    // normalized rows are those which have first non-zero element = 1.0
    // rows are normalized by dividing all elements by leading non-zero value

    let mut leading_value: f64;
    let mut count: usize;
  
    for j in 0..matrix.rows { 
        // for each row...
        count = 0;
        leading_value = 0.0;

        // find first non-zero value in row
        while leading_value == 0.0 {
            leading_value = matrix.data[j * matrix.columns + count];
            count = count + 1;
        }

        // normalize the row
        for i in 0..matrix.columns {
            matrix.data[j * matrix.columns + i] = matrix.data[j * matrix.columns + i] / leading_value;
        }
    }
}

fn subtract_row_down (matrix: &mut Matrix, row_to_subtract: usize)
{
    // substracting copy of row from all rows underneath it
    if row_to_subtract < matrix.rows {
        for j in (row_to_subtract + 1)..matrix.rows {
            for i in 0..matrix.columns {
                matrix.data[j * matrix.columns + i] = matrix.data[j * matrix.columns + i] - matrix.data[row_to_subtract * matrix.columns + i];
            }
        }
    } else {
        println!("Error! Cannot subtract unexistent row!");
        println!("Row number is {}, while there are only {} rows.", row_to_subtract + 1, matrix.rows);
    }
}

fn subtract_row_up (matrix: &mut Matrix, row_to_subtract: usize)
{
    // subtracting a weighted copy of row from all the rows above it

    let mut leading_value_index: usize = 0;
    let mut coefficient: f64;

    // find first non-zero value index
    for i in 0..matrix.columns {
        if matrix.data[row_to_subtract * matrix.columns + i] != 0.0 {
            leading_value_index = i;
            break;
        }
    }
  
    //subtracting rows such that on leading_value_index result become zero
    for j in (0..row_to_subtract).rev() {
        coefficient = matrix.data[j * matrix.columns + leading_value_index] / matrix.data[row_to_subtract * matrix.columns + leading_value_index];

        for i in 0..matrix.columns {
            matrix.data[j * matrix.columns + i] = matrix.data[j * matrix.columns + i] - coefficient * matrix.data[row_to_subtract * matrix.columns + i];
        }
    } 
}