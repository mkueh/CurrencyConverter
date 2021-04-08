use std::fmt;

#[derive(Debug, Clone)]
pub struct DateIsOutOfRange;

impl fmt::Display for DateIsOutOfRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The date is out of the dateset range")
    }
}

#[derive(Debug, Clone)]
pub struct CurrencyCodeNotFound;

impl fmt::Display for CurrencyCodeNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No information found for this CurrencyCode")
    }
}