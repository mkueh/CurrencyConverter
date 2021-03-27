use chrono::{NaiveDate,};
use serde::{Deserialize, Serializer, Deserializer};
use std::collections::HashMap;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use crate::Currency_Exchange::Currency;

#[derive(Deserialize, Debug, Clone)]
pub struct Currency_History_Entry {
    CURRENCY: String,
    CURRENCY_DENOM: String,
    #[serde(with = "my_date_format")]
    TIME_PERIOD: NaiveDate,
    OBS_VALUE: f64,
}

mod my_date_format {
    use chrono::{DateTime, Utc, TimeZone, FixedOffset, NaiveDate};
    use serde::{self, Deserialize, Deserializer};
    use chrono::format::Fixed;

    pub fn deserialize<'de, D>(deserializer: D,) -> Result<NaiveDate, D::Error> where D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let utc = NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom);
        return utc;
    }
}

#[derive(Clone)]
pub struct Currency_History {
    exchange_entry : Vec<Currency_History_Entry>,
    base_CURRENCY: String,
    target_CURRENCY: String,
    first_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
}

impl Currency_History {
    pub fn new() -> Currency_History {
        let mut ret = Currency_History{
            exchange_entry: Vec::new(),
            base_CURRENCY: "".to_string(),
            target_CURRENCY: "".to_string(),
            first_date: None,
            end_date: None
        };
        return ret;
    }

    pub fn init(&mut self, ezb_response:String){
        let mut rdr = csv::Reader::from_reader(ezb_response.as_bytes());
        let mut deserial_result = rdr.deserialize();

        for (i,result) in deserial_result.enumerate() {
            let record:Currency_History_Entry = result.unwrap();
            self.exchange_entry.push(record);
        }

        self.base_CURRENCY = self.exchange_entry[0].CURRENCY.clone();
        self.target_CURRENCY = self.exchange_entry[0].CURRENCY_DENOM.clone();
        self.first_date = Option::from(self.exchange_entry[0].TIME_PERIOD.clone());
        self.end_date = Option::from(self.exchange_entry[self.exchange_entry.len() - 1].TIME_PERIOD.clone());

        }
}


