use polars::prelude::*;
use std::fs::File;

fn min_max_normalization(series: &Series) -> PolarsResult<Series> {
  // Cast the series to float type to ensure correct arithmetic
  let series = series.cast(&DataType::Float64)?;

  // Get the min and max values, handling Result<Option<f64>, PolarsError>
  let min: Option<f64> = series.min()?;
  let max: Option<f64> = series.max()?;

  // Debug: Print min and max
  println!("MIN: {:?}", min);
  println!("MAX: {:?}", max);

  // If min or max is None, return an error
  if min.is_none() || max.is_none() {
    return Err(PolarsError::ComputeError("Cannot normalize series with no min or max".into()));
  }

  let min = min.unwrap();
  let max = max.unwrap();

  // Check if max and min are the same
  if min == max {
    println!("All values are the same. Setting normalized values to 1.0.");
    let length = series.len();
    let ones_series = Series::new(series.name().clone(), vec![1.0; length]);
    return Ok(ones_series);
  }

  // Standard Min-Max Normalization
  // println!("Performing standard Min-Max normalization.");
  let normalized_series:Series = (&series - min) / (max - min);
  // Convert the normalized series to a vector of f64
  let rounded_values: Vec<f64> = normalized_series
    .f64()?
    .into_iter()
    .map(|opt_val| opt_val.map(|val| (val * 100.0).round() / 100.0))
    .collect::<Option<Vec<f64>>>()
    .ok_or_else(|| PolarsError::ComputeError("Failed to convert series to vector".into()))?;

  // Create a new series with the rounded values
  let rounded_series = Series::new(series.name().clone(), rounded_values);

  // Debug: Print the first few values of the normalized series
  // println!("First few normalized values: {:?}", normalized_series.head(Some(5)));

  Ok(rounded_series)
}

fn main() {
  let csv_path = "/Volumes/dHDD/Rsys/ml-rust/src_files/heart_attack_prediction_dataset.csv";

  let column = "Heart Rate";

  let mut df = CsvReadOptions::default()
    .try_into_reader_with_file_path(Some(csv_path.into()))
    .unwrap()
    .finish()
    .expect("CSV reader failed");

  // println!("DF: {:#?}", df);

  if df.column(column).is_ok() {
    println!("Found column {}", column);
    let norm_series = min_max_normalization(df.column(column).unwrap());
    let n_series = norm_series.unwrap().rename(PlSmallStr::from(&format!("{} MinMax", column))).clone();
    df.with_column(n_series)
      .expect("PlSmallStr-format: panic message");
    // Display the modified DataFrame
    // println!("{:?}", df);

    let file_out = "/Volumes/dHDD/Rsys/ml-rust/src_files/hap_normalized.csv";

    let mut file = File::create(file_out).unwrap();

    let _ = CsvWriter::new(&mut file)
      .finish(&mut df).expect("CSV file write panic message");

    println!("Normalized data in {}", file_out);

  }

}
