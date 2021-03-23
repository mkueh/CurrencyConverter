use serde_json::{Value};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, FixedOffset, Utc};
use serde::{Deserialize, Serializer, Deserializer};
use std::collections::HashMap;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

#[derive(Deserialize, Debug, Clone)]
struct Currency_Exchange_Entry {
    CURRENCY: String,
    CURRENCY_DENOM: String,
    #[serde(with = "my_date_format")]
    TIME_PERIOD: NaiveDate,
    OBS_VALUE: f64,
}

#[derive(Clone)]
struct Currency_Exchange {
    exchange_entry : Vec<Currency_Exchange_Entry>,
    base_CURRENCY: String,
    target_CURRENCY: String,
    first_date: NaiveDate,
    end_date: NaiveDate,
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


fn hello_world() -> Result<i32, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let package = client.get("https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.USD.EUR.SP00.A?startPeriod=2020").header("Accept","text/csv");
    let respons = package.send()?.text().unwrap();

    let mut rdr = csv::Reader::from_reader(respons.as_bytes());
    let mut deserial_result = rdr.deserialize();

    let mut vec_of_entrys: Vec<Currency_Exchange_Entry> = Vec::new();
    for (i,result) in deserial_result.enumerate() {
        let record:Currency_Exchange_Entry = result?;
        vec_of_entrys.push(record);
    }

    let base_CURRENCY = vec_of_entrys[0].CURRENCY.clone();
    let target_CURRENCY= vec_of_entrys[0].CURRENCY_DENOM.clone();
    let first_date= vec_of_entrys[0].TIME_PERIOD.clone();
    let end_date= vec_of_entrys[vec_of_entrys.len()-1].TIME_PERIOD.clone();


    let USD_Exchange: Currency_Exchange = Currency_Exchange{exchange_entry: vec_of_entrys, base_CURRENCY, target_CURRENCY,
        first_date, end_date
    };

    Ok(-1)
}

fn main() {
    let return_value = hello_world(); // Nothing is printed
    println!("return value= {:#?}", return_value)
}