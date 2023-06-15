use crate::{error::DataAccessError, WidgetParseError};

pub type DataAccessResult<T> = std::result::Result<T, DataAccessError>;

#[allow(unused)]
pub type WidgetParseResult<T> = std::result::Result<T, WidgetParseError>;
