use serde_json::{Value};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, FixedOffset, Utc};
use serde::{Deserialize, Serializer, Deserializer};
use std::collections::HashMap;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

#[derive(Deserialize, Debug)]
struct Record {
    CURRENCY: Value,
    CURRENCY_DENOM: Value,
    #[serde(with = "my_date_format")]
    TIME_PERIOD: DateTime<Utc>,
    OBS_VALUE: f64,
    UNIT: String,
    TITLE: String,
}

mod my_date_format {
    use chrono::{DateTime, Utc, TimeZone, FixedOffset, NaiveDate};
    use serde::{self, Deserialize, Deserializer};
    use chrono::format::Fixed;

    pub fn deserialize<'de, D>(deserializer: D,) -> Result<DateTime<Utc>, D::Error> where D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        println!("s= {:?}", s);
        let utc = Utc.datetime_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom);
        return utc;
    }
}


fn hello_world() -> Result<i32, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let package = client.get("https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.USD.EUR.SP00.A?startPeriod=2020").header("Accept","text/csv");
    let respons = package.send()?.text().unwrap();

    let mut rdr = csv::Reader::from_reader(respons.as_bytes());
    for result in rdr.deserialize() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record:Record = result?;
        println!("{:?}", record);
    }
    Ok(-1)
}

fn main() {
    let return_value = hello_world(); // Nothing is printed
    println!("return value= {:#?}", return_value)
}