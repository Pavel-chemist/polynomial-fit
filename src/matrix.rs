use crate::matrix_fn::draw_matrix;

pub struct Matrix {
  rows: usize,
  columns: usize,
  data: Vec<f64>,
}

impl Matrix {
    // constructor
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            rows,
            columns,
            data: vec![0.0; rows * columns],
        }
    }

    // getters and setters
    pub fn get_value_at(&self, column: usize, row: usize) -> f64 {
        if column < self.columns && row < self.rows {
            return self.data[row * self.columns + column];
        } else {
            if column >= self.columns {
                println!("Trying to get at column {} while there are {} columns.", column+1, self.columns);
            }
            
            if row >= self.rows {
                println!("Trying to get at row {} while there are {} rows.", row+1, self.rows);
            }
            
            panic!("Get value Error! Indices are outside of bounds!");
        }
    }

    pub fn set_value_at(&mut self, column: usize, row: usize, value: f64) {
        if column < self.columns && row < self.rows {
            self.data[row * self.columns + column] = value;
        } else {
            if column >= self.columns {
                println!("Trying to set at column {} while there are {} columns.", column+1, self.columns);
            }
            
            if row >= self.rows {
                println!("Trying to set at row {} while there are {} rows.", row+1, self.rows);
            }

            panic!("Set value Error! Indices are outside of bounds!");
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

    // matrix related
    pub fn copy(matrix_to_be_copied: &Matrix) -> Matrix {
        let mut copied_matrix: Matrix = Matrix::new(matrix_to_be_copied.rows, matrix_to_be_copied.columns);

        for i in 0..matrix_to_be_copied.data.len() {
            copied_matrix.data[i] = matrix_to_be_copied.data[i];
        }

        return copied_matrix;
    }

    pub fn transpose (matrix_to_be_transposed: &Matrix) -> Matrix {
        let mut transposed_matrix: Matrix = Matrix::new(matrix_to_be_transposed.columns, matrix_to_be_transposed.rows);

        for j in 0..transposed_matrix.rows {
            for i in 0..transposed_matrix.columns {
                transposed_matrix.set_value_at(i, j, matrix_to_be_transposed.get_value_at(j, i));
            }
        }

        return transposed_matrix;
    }

    pub fn sum(addend1: &Matrix, addend2: &Matrix) -> Matrix {
        let mut sum: Matrix = Matrix::new(addend1.rows, addend1.columns);

        if addend1.rows == addend2.rows && addend1.columns == addend2.columns {
            for i in 0..sum.size() {
                sum.data[i] = addend1.data[i] + addend2.data[i];
            }
        } else {
            panic!("Non-equal matrices cannot be summed!");
        }

        return sum;
    }

    pub fn scalar_multiply(matrix: &Matrix, multiplier: f64) -> Matrix {
        let mut scaled_matrix: Matrix = Matrix::new(matrix.rows, matrix.columns);

        for i in 0..matrix.size() {
            scaled_matrix.data[i] = matrix.data[i] * multiplier;
        }

        return scaled_matrix;
    }

    pub fn multiply(left: &Matrix, right: &Matrix) -> Matrix {
        if left.columns == right.rows {
            let common = left.columns;
            let mut product: Matrix = Matrix::new(left.rows, right.columns);

            for j in 0..product.rows {
                for i in 0..product.columns {
                    let mut running_sum: f64 = 0.0;

                    for c in 0..common {
                        running_sum += left.get_value_at(c, j) * right.get_value_at(i, c);
                    }

                    product.set_value_at(i, j, running_sum);
                }
            }

            return product;
        } else {
            panic!("left matrix should have the same number of columns as right rows!");
        }
    }

    pub fn row_echelon_form(matrix: &Matrix) -> Matrix {
        println!("Converting to reduced row_echelon_form");

        let mut result: Matrix = Matrix::copy(matrix);

        println!("copied matrix:");
        draw_matrix(&result);  
    
        // make all rows in matrix start with 1.0
        normalize_rows(&mut result);

        println!("normalized matrix:");
        draw_matrix(&result);  
    
        // convert matrix to row echelon form
        for i in 0..(result.rows() - 1) {
            subtract_row_down ( &mut result, i );
            normalize_rows ( &mut result );
        }

        println!("matrix in row echelon form:");
        draw_matrix(&result);  
      
        // convert echelon matrix to reduced row echelon form
        for i in (0..result.rows()).rev() {
            subtract_row_up (&mut result, i );
        }

        println!("matrix in reduced row echelon form:");
        draw_matrix(&result);
    
        println!("dealing with -0.0 values...");
        // dealing with -0.0  values
        for j in 0..result.rows() {
            for i in 0..result.columns() {    
                if result.get_value_at(i, j) == -0.0 {
                    result.set_value_at(i, j, 0.0);
                }
            }
        }

        println!("reduced row echelon form result:");
        draw_matrix(&result);
    
        return result;
    }
}

fn normalize_rows(matrix: &mut Matrix) {
    // normalized rows are those which have first non-zero element = 1.0
    // rows are normalized by dividing all elements by leading non-zero value

    let mut leading_value: f64;
    let mut count: usize;
  
    for j in 0..matrix.rows() { 
        // for each row...
        count = 0;
        leading_value = 0.0;

        // find first non-zero value in row
        while leading_value == 0.0 {
            leading_value = matrix.get_value_at(count, j);

            count = count + 1;
        }

        // normalize the row
        for i in 0..matrix.columns() {
            let normalized_value: f64 = matrix.get_value_at(i, j) / leading_value;

            matrix.set_value_at(i, j, normalized_value);
        }
    }
}

fn subtract_row_down (matrix: &mut Matrix, row_to_subtract: usize)
{
    // substracting copy of row from all rows underneath it
    if row_to_subtract < matrix.rows() {
        for j in (row_to_subtract + 1)..matrix.rows() {
            println!("subtracting row {} from row {}...", row_to_subtract+1, j+1);

            for i in 0..matrix.columns() {
                let subtracted_value: f64 = matrix.get_value_at(i, j) - matrix.get_value_at(i, row_to_subtract);

                matrix.set_value_at(i, j, subtracted_value);
            }
        }
    } else {
        println!("Error! Cannot subtract unexistent row!");
        println!("Row number is {}, while there are only {} rows.", row_to_subtract + 1, matrix.rows());
    }
}

fn subtract_row_up (matrix: &mut Matrix, row_to_subtract: usize)
{
    // subtracting a weighted copy of row from all the rows above it

    let mut leading_value_index: usize = 0;
    let mut coefficient: f64;

    // find first non-zero value index
    for i in 0..matrix.columns() {
        if matrix.get_value_at(i, row_to_subtract) != 0.0 {
            leading_value_index = i;
            break;
        }
    }
  
    //subtracting rows such that on leading_value_index result become zero
    for j in (0..row_to_subtract).rev() {
        coefficient = matrix.get_value_at(leading_value_index, j) / matrix.get_value_at(leading_value_index, row_to_subtract);

        for i in 0..matrix.columns() {
            let subtracted_value: f64 = matrix.get_value_at(i, j) - coefficient * matrix.get_value_at(i, row_to_subtract);

            matrix.set_value_at(i, j, subtracted_value);
        }
    } 
}