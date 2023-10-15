// utils/calculations.rs

// Function to calculate the total portfolio value
pub fn calculate_portfolio_value(crypto_value: u64, bonds_value: u64, stocks_value: u64) -> u64 {
  crypto_value + bonds_value + stocks_value
}

// Function to calculate the percentage of each asset in the portfolio
pub fn calculate_asset_percentage(asset_value: u64, total_portfolio_value: u64) -> f64 {
  (asset_value as f64 / total_portfolio_value as f64) * 100.0
}

// Add other calculation functions as needed
