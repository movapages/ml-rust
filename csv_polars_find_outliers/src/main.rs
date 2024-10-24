use polars::prelude::*;
use std::fs::File;

fn main() {
  let csv_path = "/Volumes/dHDD/Rsys/ml-rust/src_files/heart_attack_prediction_dataset.csv";

  // let column = "Heart Rate";

  let csv_df = CsvReadOptions::default()
    .try_into_reader_with_file_path(Some(csv_path.into()))
    .unwrap()
    .finish()
    .unwrap();

  // println!("DF: {:#?}", csv_df);
  let fdf = csv_df.lazy();
  let mut outliers = fdf.filter(col("Heart Rate").lt(41).or(col("Heart Rate").gt(200)))
    .sort(vec!["Heart Rate"], SortMultipleOptions {
        ..Default::default()
      })
    .collect()
    .unwrap();
  // println!("fdf {:#?}", outliers[5]);
  // println!("fdf {:#?}", outliers.get_row(1));
  // println!("fdf {:#?}", outliers.slice(0,12));

  let file_out = "/Volumes/dHDD/Rsys/ml-rust/src_files/outlier_count.csv";

  let mut file = File::create(file_out).unwrap();

  let _ = CsvWriter::new(&mut file)
    .finish(&mut outliers).expect("finish panic message");

  println!("Outliers in {}", file_out);

}
