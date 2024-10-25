use polars::prelude::*;
use std::fs::File;

fn main() {
  let csv_path = "/Volumes/dHDD/Rsys/ml-rust/src_files/heart_attack_prediction_dataset.csv";

  // 1. Read CSV dataset using CsvReadOptions
  let df = CsvReadOptions::default()
    .try_into_reader_with_file_path(Some(csv_path.into()))
    .unwrap()
    .finish()
    .expect("CSV reader failed");

  // 2. Access the "Gender" column
  let gender_series = df.column("Sex").expect("Column 'Gender' not found");

  // 3. Create a new vector for binary values
  let mut binary_values = Vec::new();

  // 4. Iterate over the values in the "Gender" column
  for i in 0..gender_series.len() {
    match gender_series.get(i) {
      Ok(value) => {
        let value_str = value.to_string(); // Convert AnyValue to string
        if value_str == "Male" {
          binary_values.push(Some(1));
        } else if value_str == "Female" {
          binary_values.push(Some(0));
        } else {
          binary_values.push(None); // Handle unknown or missing values
        }
      }
      Err(_) => binary_values.push(None), // Handle errors
    }
  }

  // 5. Convert to a Series and rename
  let gender_binary_series = Int32Chunked::from_slice_options(PlSmallStr::from("gender_binary"), &binary_values).into_series();

  // 6. Add the gender binary column to the DataFrame using hstack
  let mut df_binary = df.hstack(&[gender_binary_series]).expect("Failed to add binary column");

  // 7. Write the results to a new CSV file
  let file_out = "/Volumes/dHDD/Rsys/ml-rust/src_files/hap_categorical_feature.csv";
  let mut file = File::create(file_out).expect("Failed to create output file");

  CsvWriter::new(&mut file)
    .finish(&mut df_binary)
    .expect("Failed to write to CSV file");

  println!("CSV data with categorical variable is in {}", file_out);
}
