use chrono::{NaiveDate,};
use serde::{Deserialize, Serializer, Deserializer};
use std::collections::HashMap;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use crate::Currency_Exchange::Currency;

#[derive(Deserialize, Debug, Clone)]
pub struct Currency_History_Entry {
    pub CURRENCY: String,
    #[serde(rename = "CURRENCY_DENOM")]
    pub CURRENCY_TARGET: String,
    #[serde(with = "my_date_format")]
    pub TIME_PERIOD: NaiveDate,
    pub OBS_VALUE: Option<f64>,
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

#[derive(Clone, Debug)]
pub struct Exchange_History {
    pub exchange_entrys : Vec<Currency_History_Entry>,
    pub base_CURRENCY: String,
    pub target_CURRENCY: String,
    pub first_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

impl Exchange_History {
    pub fn new() -> Exchange_History {
        let mut ret = Exchange_History {
            exchange_entrys: Vec::new(),
            base_CURRENCY: "".to_string(),
            target_CURRENCY: "".to_string(),
            first_date: None,
            end_date: None
        };
        return ret;
    }

    pub fn init(&mut self, ezb_response:String){
        self.base_CURRENCY = self.exchange_entrys[0].CURRENCY.clone();
        self.target_CURRENCY = self.exchange_entrys[0].CURRENCY_TARGET.clone();
        self.first_date = Option::from(self.exchange_entrys[0].TIME_PERIOD.clone());
        self.end_date = Option::from(self.exchange_entrys[self.exchange_entrys.len() - 1].TIME_PERIOD.clone());

        }
}


