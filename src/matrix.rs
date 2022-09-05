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

    pub fn swap_rows(&mut self, row1: usize, row2: usize) {
        if row1 < self.rows && row2 < self.rows {
            let mut value_hold: f64;
            for i in 0..self.columns {
                value_hold = self.get_value_at(i, row1);
                self.set_value_at(i, row1, self.get_value_at(i, row2));
                self.set_value_at(i, row2, value_hold);
            }
        } else {
            panic!("swapping non-existent rows!");
        }
    }

    pub fn swap_columns(&mut self, column1: usize, column2: usize) {
        if column1 < self.columns && column2 < self.columns {
            let mut value_hold: f64;
            for j in 0..self.rows {
                value_hold = self.get_value_at(column1, j);
                self.set_value_at(column1, j, self.get_value_at(column2, j));
                self.set_value_at(column2, j, value_hold);
            }
        } else {
            panic!("swapping non-existent rows!");
        }
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
}

fn is_second_row_starts_later(matrix: &Matrix, row1: usize, row2: usize) -> bool {
    if row1 < matrix.rows && row2 < matrix.rows {
        // find first column where at least one value is not zero
        for i in 0..matrix.columns {
            if matrix.get_value_at(i, row1) == 0.0 && matrix.get_value_at(i, row2) == 0.0 {
                // continue -- zero, above zero --> going to check next column
            } else if matrix.get_value_at(i, row1) != 0.0 && matrix.get_value_at(i, row2) == 0.0 {
                return true;
            } else {
                return false;
            };
        }

        // in case both rows are zeros:
        return false;
    } else {
        panic!("Trying to compare non-existent rows!");
    }
}

fn normalize_rows(matrix: &mut Matrix) {
    // normalized rows are those which have first non-zero element = 1.0
    // rows are normalized by dividing all elements by leading non-zero value

    let mut leading_value: f64;
    let mut count: usize;
  
    sort_rows(matrix);

    for j in 0..matrix.rows() { 
        // for each row...
        count = 0;
        leading_value = 0.0;

        // find first non-zero value in row
        while leading_value == 0.0 {
            if count == matrix.columns {
                leading_value = 1.0;

                break;
            }

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

pub fn sort_rows(matrix: &mut Matrix) {
    let mut lead_val_index: usize = 0;
    let mut lead_val_index_next: usize = 0;
    let mut has_rows_swapped: bool = false;

    for sweep in 0..matrix.rows - 1 {
        has_rows_swapped = false;

        for j in 0..matrix.rows - 1 {
            for i in 0..matrix.columns {
                if matrix.get_value_at(i, j) != 0.0 {
                    lead_val_index = i;
    
                    break;
                }
            }
            
            for  i in 0..matrix.columns {
                if matrix.get_value_at(i, j + 1) != 0.0 {
                    lead_val_index_next = i;
    
                    break;
                }
            }
    
            if lead_val_index_next < lead_val_index {
                matrix.swap_rows(j, j + 1);

                has_rows_swapped = true;
            }
        }

        // if no rows been swapped, then matrix is sorted already
        if !has_rows_swapped {
            break;
        }
    } 
}

fn subtract_row_down(matrix: &mut Matrix, row_to_subtract: usize) {
    // substracting copy of row from all rows underneath it
    if row_to_subtract < matrix.rows() {
        for j in (row_to_subtract + 1)..matrix.rows() {            
            if !is_second_row_starts_later(matrix, row_to_subtract, j) {
                for i in 0..matrix.columns() {
                    let low_val: f64 = matrix.get_value_at(i, j);
                    let subtr_val: f64 = matrix.get_value_at(i, row_to_subtract);
                    let subtracted_value: f64 = low_val - subtr_val;
    
                    matrix.set_value_at(i, j, subtracted_value);
                }
            }
        }
    } else {
        println!("Row number is {}, while there are only {} rows.", row_to_subtract + 1, matrix.rows());
        panic!("Error! Cannot subtract unexistent row!");
    }
}

fn subtract_row_up(matrix: &mut Matrix, row_to_subtract: usize) {
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
        if matrix.get_value_at(leading_value_index, row_to_subtract) == 0.0 {
            coefficient = 1.0;
        } else {
            coefficient = matrix.get_value_at(leading_value_index, j) / matrix.get_value_at(leading_value_index, row_to_subtract);
        }

        for i in 0..matrix.columns() {
            let subtracted_value: f64 = matrix.get_value_at(i, j) - coefficient * matrix.get_value_at(i, row_to_subtract);

            matrix.set_value_at(i, j, subtracted_value);
        }
    } 
}