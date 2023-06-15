mod data_access;
mod error;
mod object_model;
mod proto;
mod result;

pub use self::data_access::WidgetRepo;
pub use self::error::{DataAccessError, WidgetParseError};
pub use self::object_model::Widget;
pub use self::proto::adder;
pub use self::result::DataAccessResult;
