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
            let mut product: Matrix = Matrix::new(left.rows, right.columns);

            for j in 0..product.rows {
                for i in 0..product.columns {
                    let mut running_sum: f64 = 0.0;

                    for a in 0..left.columns {
                        running_sum += left.get_value_at(a, j) * right.get_value_at(i, a);
                    }

                    product.set_value_at(i, j, running_sum);
                }
            }

            return product;
        } else {
            panic!("left matrix should have the same number of columns as right rows!");
        }
    }
}
