use polars::prelude::*;

fn main() {
  let csv_path = "/Volumes/dHDD/Rsys/ml-rust/src_files/hap_with_zeros.csv";

  let df = CsvReadOptions::default()
    .with_has_header(true)
    .try_into_reader_with_file_path(Some(csv_path.parse().unwrap())).unwrap()
    .finish();
  let binding = df.unwrap();
  let cols = &binding.get_column_names();

  println!("Columns to check {:?}", cols);

  // for col in cols.into_iter() {
  // for col in cols.iter() {
  for col in cols {
    // let series = &binding.column(col.deref()).unwrap();
    let series = &binding.column(col).unwrap();
    let missing_count = series.null_count();
    println!("Column '{}' has {} missing values", col, missing_count);
  }
}
