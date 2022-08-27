// module with functions wor calculating reduced row echelon form for matrices

pub fn row_echelon_form(mut data: Vec<f64>, rows: usize, columns: usize) -> Vec<f64>{
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