use polars::prelude::*;

fn main() {
  let csv_path = "/Volumes/dHDD/Rsys/ml-rust/src_files/heart_attack_prediction_dataset.csv";
  let csv_df = CsvReadOptions::default()
    .try_into_reader_with_file_path(Some(csv_path.into()))
    .unwrap()
    .finish()
    .unwrap();

  println!("DF: {:#?}", csv_df);
}
