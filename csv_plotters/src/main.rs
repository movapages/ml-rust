use polars::prelude::*;
use plotters::prelude::*;

fn main() {
  let csv_path = "/Volumes/dHDD/Rsys/ml-rust/src_files/heart_attack_prediction_dataset.csv";

  // 1. Read CSV dataset
  let df = CsvReadOptions::default()
    .try_into_reader_with_file_path(Some(csv_path.into()))
    .unwrap()
    .finish()
    .expect("CSV reader failed");

  // println!("DF: {:#?}", df);

  // 2. Get ready 'Age' and 'Heart rate' cols
  // let age_series = df.column("Age").unwrap().f64().unwrap();
  // let hr_series = df.column("Heart Rate").unwrap().f64().unwrap();

  // Cast 'Age' and 'Heart rate' columns to f64 if needed
  let age_series = df.column("Age").unwrap().cast(&DataType::Float64).unwrap();
  let hr_series = df.column("Heart Rate").unwrap().cast(&DataType::Float64).unwrap();

  // Extract the columns as f64 series
  let age_series = age_series.f64().unwrap();
  let hr_series = hr_series.f64().unwrap();

  // 3. Prepare data set for scatter plot
  let age_data: Vec<f64> = age_series.into_iter().filter_map(|x| x).collect();
  let hr_data: Vec<f64> = hr_series.into_iter().filter_map(|x| x).collect();

  // 4. Chk data lengths
  if age_data.len() != hr_data.len() {
    panic!("Mismatched number of values in CSV Age and/or Heart Rate");
  }

  // Calculate the range for X and Y axes, ignoring NaN values
  let age_min = age_data.iter().cloned().fold(f64::INFINITY, f64::min);
  let age_max = age_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
  let hr_min = hr_data.iter().cloned().fold(f64::INFINITY, f64::min);
  let hr_max = hr_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

  // 5. Plot it
  let plot_path = "/Volumes/dHDD/Rsys/ml-rust/src_files/age_vs_heart_rate_plot.png";
  let root = BitMapBackend::new(plot_path, (1024, 768)).into_drawing_area();
  root.fill(&WHITE).unwrap();

  let mut chart = ChartBuilder::on(&root)
    .caption("Age vs Heart Rate", ("sans-serif", 50).into_font())
    .margin(10)
    .x_label_area_size(40)
    .y_label_area_size(40)
    .build_cartesian_2d(age_min..age_max, hr_min..hr_max).unwrap();

  chart.configure_mesh().draw().unwrap();

  chart.draw_series(
    age_data.iter().zip(hr_data.iter()).map(|(&age, &hr)| {
      Circle::new((age, hr), 5, ShapeStyle::from(&RED).filled())
    }),
  ).unwrap();

  // Ensure the drawing is saved
  root.present().expect("Unable to write result to file");

  println!("Scatter plot saved to {}", plot_path);
}
