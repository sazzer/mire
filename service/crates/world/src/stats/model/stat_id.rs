use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

/// The unique identifier for an Stat record
#[derive(Debug, PartialEq, FromSql, Clone, Deserialize, Serialize)]
pub struct StatId(Uuid);

impl Default for StatId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl From<Uuid> for StatId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl ToString for StatId {
    fn to_string(&self) -> String {
        format!("{}", self.0)
    }
}

/// Errors that can occur when parsing a Stat ID
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum StatIdParseError {
    /// The Stat ID was malformed
    #[error("The Stat ID was malformed")]
    Malformed,
}

impl FromStr for StatId {
    type Err = StatIdParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let id_value = Uuid::parse_str(input).map_err(|_| StatIdParseError::Malformed)?;
        Ok(Self(id_value))
    }
}

impl ToSql for StatId {
    accepts!(UUID);

    to_sql_checked!();

    fn to_sql(
        &self,
        t: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.0.to_sql(t, w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn parse_blank() {
        let err = StatId::from_str("").unwrap_err();
        check!(err == StatIdParseError::Malformed);
    }

    #[test]
    fn parse_not_uuid() {
        let err = StatId::from_str("thisIsntAUUID").unwrap_err();
        check!(err == StatIdParseError::Malformed);
    }

    #[test]
    fn parse_not_hex() {
        let err = StatId::from_str("2ac84068-1664-4f41-8e98-a464c78c5e4h").unwrap_err();
        check!(err == StatIdParseError::Malformed);
    }

    #[test]
    fn parse_valid() {
        let stat_id = StatId::from_str("2ac84068-1664-4f41-8e98-a464c78c5e40").unwrap();

        check!(stat_id.0.to_string() == "2ac84068-1664-4f41-8e98-a464c78c5e40");
    }
}
