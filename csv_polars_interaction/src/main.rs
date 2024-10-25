use polars::prelude::*;
use polars_core::datatypes::Schema;
use std::fs::File;

fn main() {
  let csv_path = "/Volumes/dHDD/Rsys/ml-rust/src_files/heart_attack_prediction_dataset.csv";

  // 1. Read CSV dataset
  let df = CsvReadOptions::default()
    .try_into_reader_with_file_path(Some(csv_path.into()))
    .unwrap()
    .finish()
    .expect("CSV reader failed");

  // 2. Calculate the interaction column as a Series
  let age_series = df.column("Age").expect("Column 'age' not found");
  let cholesterol_series = df.column("Cholesterol").expect("Column 'cholesterol_level' not found");

  let mut binding = (age_series * cholesterol_series).expect("Failed to calculate interaction");

  let interaction_series = binding.rename(PlSmallStr::from("age_cholesterol_interaction"));

  // 3. Add the interaction column to the DataFrame using hstack
  let mut df_interact = df.hstack(&[interaction_series.clone()]).expect("Failed to add interaction column");


  // println!("DF: {:#?}", df);

  // 4. Put down the results
  let file_out = "/Volumes/dHDD/Rsys/ml-rust/src_files/hap_heart_attack_interaction.csv";

  let mut file = File::create(file_out).unwrap();

  let _ = CsvWriter::new(&mut file)
    .finish(&mut df_interact).expect("CSV file write panic message");

  println!("CSV data with interaction feature is in {}", file_out);
}
