use bytes::BytesMut;
use mire_authorization::PrincipalId;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

/// The unique identifier for a User record
#[derive(Debug, PartialEq, FromSql, Clone, Deserialize, Serialize)]
pub struct UserId(Uuid);

impl Default for UserId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl From<Uuid> for UserId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl ToString for UserId {
    fn to_string(&self) -> String {
        format!("{}", self.0)
    }
}

/// Errors that can occur when parsing a User ID
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum UserIdParseError {
    /// The User ID was malformed
    #[error("The User ID was malformed")]
    Malformed,
}

impl FromStr for UserId {
    type Err = UserIdParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let id_value = Uuid::parse_str(input).map_err(|_| UserIdParseError::Malformed)?;
        Ok(Self(id_value))
    }
}

impl ToSql for UserId {
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

impl From<UserId> for PrincipalId {
    fn from(user_id: UserId) -> Self {
        Self::User(user_id.0.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn parse_blank() {
        let err = UserId::from_str("").unwrap_err();
        check!(err == UserIdParseError::Malformed);
    }

    #[test]
    fn parse_not_uuid() {
        let err = UserId::from_str("thisIsntAUUID").unwrap_err();
        check!(err == UserIdParseError::Malformed);
    }

    #[test]
    fn parse_not_hex() {
        let err = UserId::from_str("2ac84068-1664-4f41-8e98-a464c78c5e4h").unwrap_err();
        check!(err == UserIdParseError::Malformed);
    }

    #[test]
    fn parse_valid() {
        let user_id = UserId::from_str("2ac84068-1664-4f41-8e98-a464c78c5e40").unwrap();

        check!(user_id.0.to_string() == "2ac84068-1664-4f41-8e98-a464c78c5e40");
    }

    #[test]
    fn to_principal_id() {
        let user_id = UserId::from_str("2ac84068-1664-4f41-8e98-a464c78c5e40").unwrap();
        let principal_id = PrincipalId::from(user_id);
        check!(
            principal_id == PrincipalId::User("2ac84068-1664-4f41-8e98-a464c78c5e40".to_owned())
        );
    }
}
