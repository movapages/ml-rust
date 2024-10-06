use polars::prelude::*;
use std::fs;

fn main() {
  let csv_path = "/Volumes/dHDD/Rsys/ml-rust/src_files/hap_with_zeros.csv";
  let data = fs::read_to_string(csv_path).unwrap();
  let proc_data = data.replace(';', ",");
  let proc_csv_path = "/Volumes/dHDD/Rsys/ml-rust/src_files/proc_hap_with_zeros.csv";
  fs::write(proc_csv_path, &proc_data).expect("unable to write the file");
  let csv_df = CsvReadOptions::default()
    .try_into_reader_with_file_path(Some(proc_csv_path.into()))
    .unwrap()
    .finish()
    .unwrap(); // !!! trailing unwrap() !!!

  // let binding = csv_df;
  let cols = &csv_df.get_column_names();

  println!("Columns to check {:?}", cols);

  // for col in cols.into_iter() {
  // for col in cols.iter() {
  for col in cols {
    // let series = &csv_df.column(col.deref()).unwrap();
    let series = &csv_df.column(col).unwrap();
    let missing_count = series.null_count();
    println!("Column '{}' has {} missing values", col, missing_count);
  }

}
