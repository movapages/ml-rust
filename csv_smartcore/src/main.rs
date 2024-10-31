use csv::ReaderBuilder;
use ndarray::Array2;
// use smartcore::linalg::basic::dense_matrix::DenseMatrix;
use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::linear::linear_regression::LinearRegression;
use smartcore::model_selection::train_test_split;
use smartcore::metrics::mean_squared_error;

fn main() {
  // Define the path to your CSV file
  let csv_path = "../src_files/heart_attack_prediction_dataset.csv";

  // Read CSV data into vectors
  let mut reader = ReaderBuilder::new()
      .has_headers(true)
      .from_path(csv_path)
      .expect("Failed to open CSV file");

  let mut records: Vec<Vec<f64>> = Vec::new();
  let mut labels: Vec<f64> = Vec::new();

  // Extract Age (index 1), Cholesterol (index 5), and Heart Attack Risk (index 25)
  for result in reader.records() {
    let record = result.expect("Error reading CSV record");

    // Parse Age and Cholesterol as features
    let age: f64 = record[1].parse().expect("Failed to parse Age");
    let cholesterol: f64 = record[5].parse().expect("Failed to parse Cholesterol");
    records.push(vec![age, cholesterol]);

    // Parse Heart Attack Risk as target label
    let risk: f64 = record[25].parse().expect("Failed to parse Heart Attack Risk");
    labels.push(risk);
  }

  // Convert records to ndarray
  let n_rows = &records.len();
  let n_cols = &records[0].len();
  // let flat_data: Vec<f64> = records.into_iter().flatten().collect();
  // let features = Array2::from_shape_vec((n_rows, n_cols), flat_data)
  //     .expect("Failed to create ndarray from CSV data");

  // Convert records directly to DenseMatrix
  let features = DenseMatrix::from_2d_vec(&records);


  // Split data into training and test sets
  let (x_train, x_test, y_train, y_test) = train_test_split(&features, &labels, 0.3, true, None);

  // Train a linear regression model
  let lin_reg = LinearRegression::fit(&x_train, &y_train, Default::default())
      .expect("Failed to train linear regression model");

  // Make predictions and calculate mean squared error
  let y_pred = lin_reg.predict(&x_test).expect("Failed to predict test data");
  let mse = mean_squared_error(&y_test, &y_pred);
  println!("Model Mean Squared Error: {:.2}", mse);
  
  // let features = DenseMatrix::from_2d_array(
  //   &features.outer_iter().map(|row| row.as_slice().unwrap()).collect::<Vec<_>>()
  // );

  // let features = Array2::from_shape_vec((n_rows, n_cols), flat_data)
      // .expect("Failed to create ndarray from CSV data");

  // Split data into training and test sets
  // let (x_train, x_test, y_train, y_test) = train_test_split(&features, &labels, 0.3, true, None);
  // 
  // // Convert ndarray to DenseMatrix for smartcore
  // // **Convert ndarray to DenseMatrix using from_2d_array**
  // let x_train = DenseMatrix::from_2d_array(
  //   &x_train.outer_iter().map(|row| row.as_slice().unwrap()).collect::<Vec<_>>()
  // );
  // let x_test = DenseMatrix::from_2d_array(
  //   &x_test.outer_iter().map(|row| row.as_slice().unwrap()).collect::<Vec<_>>()
  // );
  // 
  // // Train a linear regression model
  // let lin_reg = LinearRegression::fit(&x_train, &y_train, Default::default())
  //     .expect("Failed to train linear regression model");
  // 
  // // Make predictions and calculate mean squared error
  // let y_pred = lin_reg.predict(&x_test).expect("Failed to predict test data");
  // let mse = mean_squared_error(&y_test, &y_pred);
  // println!("Model Mean Squared Error: {:.2}", mse);
}
