use crate::errors::error_impl;

#[derive(Debug)]
pub enum UserControllerError {
    UserNotFound,
    UsernameAlreadyExists,
    /// A value fetched from the DB as NULL when it shouldn't.
    ///
    /// `table_name` is the name of the DB table that the NULL value was fetched from.
    /// `column_name` is the name of the column, as stored in the DB table, that the NULL value was
    /// fetched from.
    /// `explanation` is an explanation of why this value is not supposed to be NULL, used for logging.
    ///
    /// # Notes
    ///
    /// Ideally, this should never happen,
    /// which means this error variant can be replaced with `unwrap`/`expect`. But instead, this
    /// error variant is used as a safety net.
    UnexpectedNullValueFetchedFromDb {
        table_name: &'static str,
        column_name: &'static str,
        explanation: &'static str,
    },
    Sqlx(sqlx::Error),
}

error_impl!(UserControllerError);
