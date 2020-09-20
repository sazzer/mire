use std::str::FromStr;
use strum::VariantNames;

/// Enumeration of possible directions to sort in
#[derive(Debug, PartialEq)]
pub enum SortDirection {
    /// Sort in strict ascending order
    Ascending,
    /// Sort in strict descending order
    Descending,
    /// Sort in whatever order is natural for this field
    Natural,
}

/// Representation of a single field on which to sort
#[derive(Debug)]
pub struct SortField<F> {
    /// The field to sort on
    pub field: F,
    /// The direction in which to sort
    pub direction: SortDirection,
}

/// Representation of a set of fields on which to sort
#[derive(Debug)]
pub struct SortFields<F>(Vec<SortField<F>>);

impl<F> std::ops::Deref for SortFields<F> {
    type Target = Vec<SortField<F>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Errors that can occur when parsing a sort fields
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum SortFieldParseError {
    /// A field was unknown
    #[error("A field was unknown: {0}")]
    UnknownField(String),

    #[error("A field was blank")]
    BlankField,
}

impl<F> FromStr for SortField<F>
where
    F: VariantNames + FromStr,
{
    type Err = SortFieldParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (direction, field_name) = match input.get(0..1) {
            None => return Err(SortFieldParseError::BlankField),
            Some("+") => (SortDirection::Ascending, &input[1..]),
            Some("-") => (SortDirection::Descending, &input[1..]),
            Some(_) => (SortDirection::Natural, input),
        };

        let found_name = &F::VARIANTS
            .iter()
            .find(|field| field.to_lowercase() == field_name.to_lowercase())
            .ok_or_else(|| SortFieldParseError::UnknownField(input.to_owned()))?;

        let field = F::from_str(found_name)
            .map_err(|_| SortFieldParseError::UnknownField(input.to_owned()))?;

        Ok(Self { field, direction })
    }
}

impl<F> FromStr for SortFields<F>
where
    F: VariantNames + FromStr + std::fmt::Debug,
{
    type Err = Vec<SortFieldParseError>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (fields, errors): (Vec<_>, Vec<_>) = input
            .split(',')
            .map(str::trim)
            .filter_map(|part| {
                if part.is_empty() {
                    None
                } else {
                    Some(part.parse())
                }
            })
            .partition(Result::is_ok);

        if errors.is_empty() {
            let fields: Vec<_> = fields.into_iter().map(Result::unwrap).collect();
            Ok(Self(fields))
        } else {
            let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{check, let_assert};
    use strum_macros::{EnumString, EnumVariantNames};

    #[derive(Debug, PartialEq, EnumVariantNames, EnumString)]
    enum TestSorts {
        Name,
        Age,
        Created,
    }

    #[test]
    fn test_parse_blank() {
        let sorts: SortFields<TestSorts> = "".parse().unwrap();

        check!(sorts.is_empty());
    }

    #[test]
    fn test_parse_single() {
        let sorts: SortFields<TestSorts> = "name".parse().unwrap();

        check!(sorts.len() == 1);
        check!(sorts.get(0).unwrap().field == TestSorts::Name);
        check!(sorts.get(0).unwrap().direction == SortDirection::Natural);
    }

    #[test]
    fn test_parse_single_padded() {
        let sorts: SortFields<TestSorts> = "  name  ".parse().unwrap();

        check!(sorts.len() == 1);
        check!(sorts.get(0).unwrap().field == TestSorts::Name);
        check!(sorts.get(0).unwrap().direction == SortDirection::Natural);
    }

    #[test]
    fn test_parse_single_ascending() {
        let sorts: SortFields<TestSorts> = "+name".parse().unwrap();

        check!(sorts.len() == 1);
        check!(sorts.get(0).unwrap().field == TestSorts::Name);
        check!(sorts.get(0).unwrap().direction == SortDirection::Ascending);
    }

    #[test]
    fn test_parse_single_descending() {
        let sorts: SortFields<TestSorts> = "-name".parse().unwrap();

        check!(sorts.len() == 1);
        check!(sorts.get(0).unwrap().field == TestSorts::Name);
        check!(sorts.get(0).unwrap().direction == SortDirection::Descending);
    }

    #[test]
    fn test_parse_two() {
        let sorts: SortFields<TestSorts> = "name,age".parse().unwrap();

        check!(sorts.len() == 2);

        check!(sorts.get(0).unwrap().field == TestSorts::Name);
        check!(sorts.get(0).unwrap().direction == SortDirection::Natural);

        check!(sorts.get(1).unwrap().field == TestSorts::Age);
        check!(sorts.get(1).unwrap().direction == SortDirection::Natural);
    }

    #[test]
    fn test_parse_two_blanks() {
        let sorts: SortFields<TestSorts> = "name,,age".parse().unwrap();

        check!(sorts.len() == 2);

        check!(sorts.get(0).unwrap().field == TestSorts::Name);
        check!(sorts.get(0).unwrap().direction == SortDirection::Natural);

        check!(sorts.get(1).unwrap().field == TestSorts::Age);
        check!(sorts.get(1).unwrap().direction == SortDirection::Natural);
    }

    #[test]
    fn test_parse_two_padded() {
        let sorts: SortFields<TestSorts> = "name , age".parse().unwrap();

        check!(sorts.len() == 2);

        check!(sorts.get(0).unwrap().field == TestSorts::Name);
        check!(sorts.get(0).unwrap().direction == SortDirection::Natural);

        check!(sorts.get(1).unwrap().field == TestSorts::Age);
        check!(sorts.get(1).unwrap().direction == SortDirection::Natural);
    }

    #[test]
    fn test_parse_single_unknown() {
        let sorts = "unknown".parse::<SortFields<TestSorts>>().unwrap_err();

        check!(sorts.len() == 1);
        let_assert!(SortFieldParseError::UnknownField(field) = sorts.get(0).unwrap());
        check!(field == "unknown");
    }

    #[test]
    fn test_parse_single_unknown_ascending() {
        let sorts = "+unknown".parse::<SortFields<TestSorts>>().unwrap_err();

        check!(sorts.len() == 1);
        let_assert!(SortFieldParseError::UnknownField(field) = sorts.get(0).unwrap());
        check!(field == "+unknown");
    }

    #[test]
    fn test_parse_one_unknown() {
        let sorts = "name,unknown,age"
            .parse::<SortFields<TestSorts>>()
            .unwrap_err();

        check!(sorts.len() == 1);
        let_assert!(SortFieldParseError::UnknownField(field) = sorts.get(0).unwrap());
        check!(field == "unknown");
    }

    #[test]
    fn test_parse_two_unknown() {
        let sorts = "name,unknown,other"
            .parse::<SortFields<TestSorts>>()
            .unwrap_err();

        check!(sorts.len() == 2);
        
        let_assert!(SortFieldParseError::UnknownField(field) = sorts.get(0).unwrap());
        check!(field == "unknown");
        
        let_assert!(SortFieldParseError::UnknownField(field) = sorts.get(1).unwrap());
        check!(field == "other");
    }
}
