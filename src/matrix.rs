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
}