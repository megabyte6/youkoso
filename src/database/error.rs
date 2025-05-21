/// Error handling for the database module.
///
/// This module provides error types and a result alias for operations
/// related to database functionality throughout the application.
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// Represents errors that can occur in database operations.
///
/// This enum encompasses various error types that might arise during
/// database interactions, such as borrowing errors or Excel file processing issues.
///
/// # Variants
///
/// * `Borrow` - Represents errors that occur when attempting to borrow data.
/// * `Xlsx` - Represents errors related to Excel file operations.
#[derive(Debug, Error)]
pub enum Error {
    /// Represents errors that occur when attempting to borrow already borrowed data.
    ///
    /// This variant wraps `std::cell::BorrowError` and typically occurs when
    /// there's a violation of Rust's borrowing rules in the database module.
    #[error(transparent)]
    Borrow(#[from] std::cell::BorrowError),

    /// Represents errors related to Excel file operations.
    ///
    /// This variant wraps `calamine::XlsxError` and is used when operations
    /// like reading from or parsing Excel files fail.
    #[error(transparent)]
    Xlsx(#[from] calamine::XlsxError),
}
