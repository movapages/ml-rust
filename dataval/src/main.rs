use csv::{Reader, StringRecord};
use std::{fs::File, io::{BufRead, BufReader}};

use hap_dataset::HeartAttackPredictionDataset;

fn main() {

  let file_path = "/Volumes/dHDD/Rsys/ml-rust/src_files/heart_attack_prediction_dataset.csv";
  println!("Source file: {}", &file_path);
  // all records w/o column titles
  let counter = BufReader::new(File::open(&file_path).unwrap()).lines().count() - 1;

  let file = File::open(&file_path).unwrap();
  let mut reader = Reader::from_reader(file.try_clone().unwrap());
  let _headers = reader.headers();
  let mut res: Vec<HeartAttackPredictionDataset> = Vec::with_capacity(counter);
  println!("res LEN {}", res.capacity());

  for result in reader.records() {
    let record:StringRecord = result.unwrap();
    res.push(HeartAttackPredictionDataset::from(record.iter().map(|s| s.to_string()).collect::<Vec<String>>()));
  }

  println!("the 1st record {:#?}", res[0]);

}