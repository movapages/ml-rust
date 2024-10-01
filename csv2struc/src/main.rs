use csv::{Reader, StringRecord};
use std::{fs::File, io::{BufRead, BufReader}};

#[derive(Default, Debug, Clone)]
pub struct HeartAttackPredictionDataset {
  pub patient_id: String,
  pub age: i32,
  pub sex: String,
  pub cholesterol: i32,
  pub blood_pressure: String,
  pub heart_rate: i32,
  pub diabetes: i32,
  pub family_history: i32,
  pub smoking: i32,
  pub obesity: i32,
  pub alcohol_consumption: i32,
  pub exercise_hours_per_week: f64,
  pub diet: String,
  pub previous_heart_problems: i32,
  pub medication_use: i32,
  pub stress_level: i32,
  pub sedentary_hours_per_day: f64,
  pub income: i32,
  pub bmi: f64,
  pub triglycerides: i32,
  pub physical_activity_days_per_week: i32,
  pub sleep_hours_per_day: i32,
  pub country: String,
  pub continent: String,
  pub hemisphere: String,
  pub heart_attack_risk: i32,
}

impl From<Vec<String>> for HeartAttackPredictionDataset{
  fn from(value: Vec<String>) -> Self {
    Self{
      patient_id: value[0].parse().unwrap(),
      age: value[1].trim().parse::<i32>().unwrap(),
      sex: value[2].parse().unwrap(),
      cholesterol: value[3].parse().unwrap(),
      blood_pressure: value[4].parse().unwrap(),
      heart_rate: value[5].parse().unwrap(),
      diabetes: value[6].parse().unwrap(),
      family_history: value[7].parse().unwrap(),
      smoking: value[8].parse().unwrap(),
      obesity: value[9].parse().unwrap(),
      alcohol_consumption: value[10].parse().unwrap(),
      exercise_hours_per_week: value[11].parse().unwrap(),
      diet: value[12].parse().unwrap(),
      previous_heart_problems: value[13].parse().unwrap(),
      medication_use: value[14].parse().unwrap(),
      stress_level: value[15].parse().unwrap(),
      sedentary_hours_per_day: value[16].parse().unwrap(),
      income: value[17].parse().unwrap(),
      bmi: value[18].parse().unwrap(),
      triglycerides: value[19].parse().unwrap(),
      physical_activity_days_per_week: value[20].parse().unwrap(),
      sleep_hours_per_day: value[21].parse().unwrap(),
      country: value[22].parse().unwrap(),
      continent: value[23].parse().unwrap(),
      hemisphere: value[24].parse().unwrap(),
      heart_attack_risk: value[25].parse().unwrap(),
    }
  }
}

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
