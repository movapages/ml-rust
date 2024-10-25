use polars::prelude::*;
use std::fs::File;

fn main() {
  let csv_path = "/Volumes/dHDD/Rsys/ml-rust/src_files/heart_attack_prediction_dataset.csv";

  // 1. Read CSV dataset
  let mut df = CsvReadOptions::default()
    .try_into_reader_with_file_path(Some(csv_path.into()))
    .unwrap()
    .finish()
    .expect("CSV reader failed");

  // 2. Extract the column in question and create a polynomial feature (squared)
  //let heart_rate_col = df.column("Heart Rate").expect("Col clone problem").clone(); // PolarsResult<&Series>

  let heart_rate_col = df.column("Heart Rate").unwrap().clone();  // Unwrap and clone the Series
  let mut binding = (&heart_rate_col * &heart_rate_col).unwrap().into_series();
  let heart_rate_squared = binding.rename("Heart Rate^2".into());

  // Add the new polynomial feature column to the original DataFrame
  df = df.hstack(&[heart_rate_squared.clone()]).expect("Hstack problem");

  // println!("DF: {:#?}", df);

  // 4. Put down the results
  let file_out = "/Volumes/dHDD/Rsys/ml-rust/src_files/hap_polinomial_feature.csv";

  let mut file = File::create(file_out).unwrap();

  let _ = CsvWriter::new(&mut file)
    .finish(&mut df).expect("CSV file write panic message");

  println!("Normalized data in {}", file_out);
}
