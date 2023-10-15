// utils/validations.rs

// Function to validate if the provided asset percentage is within a valid range (0-100)
pub fn is_valid_asset_percentage(percentage: f64) -> bool {
  percentage >= 0.0 && percentage <= 100.0
}

// Add other validation functions as needed
