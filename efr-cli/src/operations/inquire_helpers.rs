use chrono::{DateTime, NaiveTime, Utc};
use inquire::Text;

use crate::operations::error::OperationsError;

pub trait InquireEmptyIsNone {
    type Output;

    fn prompt_empty_is_none(self) -> Result<Option<Self::Output>, inquire::InquireError>;
}

impl<'a, 'b> InquireEmptyIsNone for Text<'a, 'b> {
    type Output = String;

    fn prompt_empty_is_none(self) -> Result<Option<Self::Output>, inquire::InquireError> {
        match self.prompt_skippable()? {
            Some(output) if output.is_empty() => Ok(None),
            Some(output) => Ok(Some(output)),
            None => Ok(None),
        }
    }
}

pub struct DateRange {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

impl DateRange {
    pub fn prompt(start_message: &str, end_message: &str) -> Result<Self, OperationsError> {
        let start_date_naive = inquire::DateSelect::new(start_message).prompt()?;
        let naive_time = NaiveTime::default();
        let start_date = start_date_naive.and_time(naive_time).and_utc();

        let min_date = start_date_naive.max(Utc::now().date_naive());

        let naive = inquire::DateSelect::new(end_message)
            .with_min_date(start_date_naive)
            .with_starting_date(min_date)
            .prompt()?;
        let end_date = naive.and_time(naive_time).and_utc();

        Ok(Self {
            start_date,
            end_date,
        })
    }
}
