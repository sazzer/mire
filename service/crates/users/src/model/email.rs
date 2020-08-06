use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};

/// The email address for a User record
#[derive(Debug, PartialEq, FromSql)]
pub struct Email(String);

impl<S> From<S> for Email
where
    S: Into<String>,
{
    fn from(email: S) -> Self {
        Self(email.into())
    }
}

impl ToSql for Email {
    accepts!(TEXT, VARCHAR);

    to_sql_checked!();

    fn to_sql(
        &self,
        t: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.0.to_sql(t, w)
    }
}
