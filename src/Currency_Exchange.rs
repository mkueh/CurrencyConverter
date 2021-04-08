use std::collections::HashMap;
use std::ops::Index;
use reqwest::Response;
use chrono::NaiveDate;
use Currency::Exchange_History;
use Currency::Currency_History_Entry;
use Error::CurrencyCodeNotFound;
use Error::DateIsOutOfRange;
use futures::future::err;

mod Currency;
mod Error;

#[derive(Debug, Clone)]
pub enum Currency_CODE {
    EUR,
    USD,
    JPY,
    BGN,
    CZK,
    DKK,
    GBP,
    HUF,
    PLN,
    RON,
    SEK,
    CHF,
    ISK,
    NOK,
    HRK,
    RUB,
    TRY,
    AUD,
    BRL,
    CAD,
    CNY,
    HKD,
    IDR,
    ILS,
    KRW,
    MXN,
    MYR,
    NZD,
    PHP,
    SGD,
    THB,
    ZAR,
}

impl Currency_CODE {
    pub fn to_str(&self) -> Result<String, CurrencyCodeNotFound> {
        match self {
            Currency_CODE::EUR => return Ok("EUR".to_string()),
            Currency_CODE::USD => return Ok("USD".to_string()),
            Currency_CODE::JPY => return Ok("JPY".to_string()),
            Currency_CODE::BGN => return Ok("BGN".to_string()),
            Currency_CODE::CZK => return Ok("CZK".to_string()),
            Currency_CODE::DKK => return Ok("DKK".to_string()),
            Currency_CODE::GBP => return Ok("GBP".to_string()),
            Currency_CODE::HUF => return Ok("HUF".to_string()),
            Currency_CODE::PLN => return Ok("PLN".to_string()),
            Currency_CODE::RON => return Ok("RON".to_string()),
            Currency_CODE::SEK => return Ok("SEK".to_string()),
            Currency_CODE::CHF => return Ok("CHF".to_string()),
            Currency_CODE::ISK => return Ok("ISK".to_string()),
            Currency_CODE::NOK => return Ok("NOK".to_string()),
            Currency_CODE::HRK => return Ok("HRK".to_string()),
            Currency_CODE::RUB => return Ok("RUB".to_string()),
            Currency_CODE::TRY => return Ok("TRY".to_string()),
            Currency_CODE::AUD => return Ok("AUD".to_string()),
            Currency_CODE::BRL => return Ok("BRL".to_string()),
            Currency_CODE::CAD => return Ok("CAD".to_string()),
            Currency_CODE::CNY => return Ok("CNY".to_string()),
            Currency_CODE::HKD => return Ok("HKD".to_string()),
            Currency_CODE::IDR => return Ok("IDR".to_string()),
            Currency_CODE::ILS => return Ok("ILS".to_string()),
            Currency_CODE::KRW => return Ok("KRW".to_string()),
            Currency_CODE::MXN => return Ok("MXN".to_string()),
            Currency_CODE::MYR => return Ok("MYR".to_string()),
            Currency_CODE::NZD => return Ok("NZD".to_string()),
            Currency_CODE::PHP => return Ok("PHP".to_string()),
            Currency_CODE::SGD => return Ok("SGD".to_string()),
            Currency_CODE::THB => return Ok("THB".to_string()),
            Currency_CODE::ZAR => return Ok("ZAR".to_string()),
            _ => Err(CurrencyCodeNotFound)
        }
    }

    pub fn from_str(code: &str) -> Result<Currency_CODE, CurrencyCodeNotFound> {
        match code {
            "EUR" => return Ok(Currency_CODE::EUR),
            "USD" => return Ok(Currency_CODE::USD),
            "JPY" => return Ok(Currency_CODE::JPY),
            "BGN" => return Ok(Currency_CODE::BGN),
            "CZK" => return Ok(Currency_CODE::CZK),
            "DKK" => return Ok(Currency_CODE::DKK),
            "GBP" => return Ok(Currency_CODE::GBP),
            "HUF" => return Ok(Currency_CODE::HUF),
            "PLN" => return Ok(Currency_CODE::PLN),
            "RON" => return Ok(Currency_CODE::RON),
            "SEK" => return Ok(Currency_CODE::SEK),
            "CHF" => return Ok(Currency_CODE::CHF),
            "ISK" => return Ok(Currency_CODE::ISK),
            "NOK" => return Ok(Currency_CODE::NOK),
            "HRK" => return Ok(Currency_CODE::HRK),
            "RUB" => return Ok(Currency_CODE::RUB),
            "TRY" => return Ok(Currency_CODE::TRY),
            "AUD" => return Ok(Currency_CODE::AUD),
            "BRL" => return Ok(Currency_CODE::BRL),
            "CAD" => return Ok(Currency_CODE::CAD),
            "CNY" => return Ok(Currency_CODE::CNY),
            "HKD" => return Ok(Currency_CODE::HKD),
            "IDR" => return Ok(Currency_CODE::IDR),
            "ILS" => return Ok(Currency_CODE::ILS),
            "KRW" => return Ok(Currency_CODE::KRW),
            "MXN" => return Ok(Currency_CODE::MXN),
            "MYR" => return Ok(Currency_CODE::MYR),
            "NZD" => return Ok(Currency_CODE::NZD),
            "PHP" => return Ok(Currency_CODE::PHP),
            "SGD" => return Ok(Currency_CODE::SGD),
            "THB" => return Ok(Currency_CODE::THB),
            "ZAR" => return Ok(Currency_CODE::ZAR),
            _ => Err(CurrencyCodeNotFound)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Exchange {
    exchange_Histories: HashMap<String, Currency::Exchange_History>,
    base_Currency: Currency_CODE,
}

impl Exchange {
    pub fn new_enum(base_Currency: Currency_CODE) -> Exchange {
        let mut ret = Exchange {
            exchange_Histories: HashMap::new(),
            base_Currency,
        };
        return ret;
    }

    pub fn init(&mut self) {
        self.load_history();
    }

    fn load_history(&mut self){
        let client = reqwest::blocking::Client::new();
        let package = client
            .get(self.convertEnum2_Code())
            .header("Accept", "text/csv");
        let respons_ret = package.send().unwrap();
        let respons_unwrap = respons_ret.text().unwrap();

        let mut rdr = csv::Reader::from_reader(respons_unwrap.as_bytes());
        let mut deserial_result = rdr.deserialize();

        let mut tmp_Entry_list: Vec<Currency::Currency_History_Entry> = Vec::new();
        for (i, result) in deserial_result.enumerate() {
            let record: Currency::Currency_History_Entry = result.unwrap();
            tmp_Entry_list.push(record);
        }

        for item in tmp_Entry_list.iter_mut() {
            let target_cur = &item.CURRENCY_TARGET;
            let is_in = self.exchange_Histories.contains_key(target_cur);

            if !is_in {
                let mut tmp_history = Exchange_History::new();
                tmp_history.base_CURRENCY = self.convertEnum2_Code();
                tmp_history.target_CURRENCY = target_cur.clone();
                tmp_history.first_date = Option::from(item.TIME_PERIOD);
                let _ = self
                    .exchange_Histories
                    .insert(target_cur.clone(), tmp_history);
            } else {
                let mut tmp_history = self.exchange_Histories.get_mut(target_cur).unwrap();
                tmp_history.exchange_entrys.push(item.clone());
            }
        }
    }

    pub fn get_ExchangeRate(&self, target_Currency: Currency_CODE, date: NaiveDate) -> Result<Currency_History_Entry, DateIsOutOfRange> {
        let target_str = &target_Currency.to_str().unwrap();
        let exchangeHistory= self.exchange_Histories.get(target_str).unwrap();
        return self.search_exchangeRate(&exchangeHistory.exchange_entrys, date);
    }

    fn search_exchangeRate(&self, a: &Vec<Currency_History_Entry>, search_target: NaiveDate) -> Result<Currency_History_Entry, DateIsOutOfRange> {
        //range check
        let first_date = a.first().unwrap().TIME_PERIOD;
        let last_date = a.last().unwrap().TIME_PERIOD;

        if !(search_target <= first_date && first_date >= last_date){
            return Err(DateIsOutOfRange);
        }


        let mut low: i64 = 0;
        let mut high: i64 = a.len() as i64;
        let mut mid = ((high - low) / 2) + low;

        while low <= high {
            mid = ((high - low) / 2) + low;
            let mid_index = mid as usize;
            let val: &Currency_History_Entry = a.get(mid_index).unwrap();

            if val.TIME_PERIOD == search_target {
                return Ok(a.get(mid_index).unwrap().clone());
            }

            // Search values that are greater than val - to right of current mid_index
            if val.TIME_PERIOD < search_target {
                low = mid + 1;
            }

            // Search values that are less than val - to the left of current mid_index
            if val.TIME_PERIOD > search_target {
                high = mid - 1;
            }
        }
        mid = mid - 1;

        loop {
            let mid_index = mid as usize;
            let val: &Currency_History_Entry = a.get(mid_index).unwrap();
            if val.TIME_PERIOD < search_target {
                return Ok(a.get(mid_index).unwrap().clone());
            } else {
                mid = mid + 1;
            }
        }
    }

    fn convertEnum2_Code(&self) -> String {
        match self.base_Currency{
            Currency_CODE::EUR => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.EUR..SP00.A".to_string(),
            Currency_CODE::USD => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.USD..SP00.A".to_string(),
            Currency_CODE::JPY => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.JPY..SP00.A".to_string(),
            Currency_CODE::BGN => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.BGN..SP00.A".to_string(),
            Currency_CODE::CZK => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.CZK..SP00.A".to_string(),
            Currency_CODE::DKK => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.DKK..SP00.A".to_string(),
            Currency_CODE::GBP => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.GBP..SP00.A".to_string(),
            Currency_CODE::HUF => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.HUF..SP00.A".to_string(),
            Currency_CODE::PLN => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.PLN..SP00.A".to_string(),
            Currency_CODE::RON => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.RON..SP00.A".to_string(),
            Currency_CODE::SEK => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.SEK..SP00.A".to_string(),
            Currency_CODE::CHF => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.CHF..SP00.A".to_string(),
            Currency_CODE::ISK => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.ISK..SP00.A".to_string(),
            Currency_CODE::NOK => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.NOK..SP00.A".to_string(),
            Currency_CODE::HRK => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.HRK..SP00.A".to_string(),
            Currency_CODE::RUB => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.RUB..SP00.A".to_string(),
            Currency_CODE::TRY => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.TRY..SP00.A".to_string(),
            Currency_CODE::AUD => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.AUD..SP00.A".to_string(),
            Currency_CODE::BRL => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.BRL..SP00.A".to_string(),
            Currency_CODE::CAD => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.CAD..SP00.A".to_string(),
            Currency_CODE::CNY => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.CNY..SP00.A".to_string(),
            Currency_CODE::HKD => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.HKD..SP00.A".to_string(),
            Currency_CODE::IDR => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.IDR..SP00.A".to_string(),
            Currency_CODE::ILS => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.ILS..SP00.A".to_string(),
            Currency_CODE::KRW => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.KRW..SP00.A".to_string(),
            Currency_CODE::MXN => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.MXN..SP00.A".to_string(),
            Currency_CODE::MYR => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.MYR..SP00.A".to_string(),
            Currency_CODE::NZD => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.NZD..SP00.A".to_string(),
            Currency_CODE::PHP => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.PHP..SP00.A".to_string(),
            Currency_CODE::SGD => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.SGD..SP00.A".to_string(),
            Currency_CODE::THB => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.THB..SP00.A".to_string(),
            Currency_CODE::ZAR => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.ZAR..SP00.A".to_string(),
            _ => "NONE".to_string()
        }
    }
}
