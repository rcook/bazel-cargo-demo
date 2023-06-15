use crate::error::WidgetParseError;
use anyhow::anyhow;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;
use uuid::Uuid;

pub struct Widget(Uuid);

impl Display for Widget {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Widget {
    type Err = WidgetParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.parse::<Uuid>()
                .map_err(|e| WidgetParseError::Other(anyhow!(e)))?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::Widget;
    use anyhow::Result;

    #[test]
    fn basics() -> Result<()> {
        let widget = "66e11bc5-fb59-4406-bdef-b7cff4dd98f8".parse::<Widget>()?;
        assert_eq!("66e11bc5-fb59-4406-bdef-b7cff4dd98f8", widget.to_string());
        Ok(())
    }
}
