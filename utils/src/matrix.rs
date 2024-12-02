pub fn transpose<T: Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
  if matrix.is_empty() {
    return vec![];
  }
  let row_len = matrix[0].len();
  (0..row_len)
    .map(|i| matrix.iter().map(|row| row[i].clone()).collect())
    .collect()
}