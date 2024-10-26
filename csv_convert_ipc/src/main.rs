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

  // println!("DF: {:#?}", df);

  // Define the output IPC file path
  let ipc_path = "/Volumes/dHDD/Rsys/ml-rust/src_files/heart_attack_prediction_dataset.ipc";

  // Open a file for writing IPC data
  let mut file = File::create(ipc_path).expect("CSV file creation failed");

  // Write the DataFrame using IPC format
  IpcWriter::new(&mut file).finish(&mut df).expect("IPC writer failed");

  println!("Data successfully written to IPC format at {}", ipc_path);
}
