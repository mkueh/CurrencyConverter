use std::collections::HashMap;
use std::ops::Index;
use reqwest::Response;
use Currency::Exchange_History;

mod Currency;

#[derive(Debug, Clone)]
pub enum Currency_CODE {
    USD,
    EUR,
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
    INR,
    KRW,
    MXN,
    MYR,
    NZD,
    PHP,
    SGD,
    THB,
    ZAR,
    NONE,
}

impl Currency_CODE {
    pub fn to_str(&self) -> String {
        match self {
            Currency_CODE::EUR => return "EUR".to_string(),
            Currency_CODE::USD => return "USD".to_string(),
            Currency_CODE::AUD => return "JPY".to_string(),
            Currency_CODE::EUR => return "BGN".to_string(),
            Currency_CODE::USD => return "CZK".to_string(),
            Currency_CODE::AUD => return "DKK".to_string(),
            Currency_CODE::EUR => return "GBP".to_string(),
            Currency_CODE::USD => return "HUF".to_string(),
            Currency_CODE::AUD => return "PLN".to_string(),
            Currency_CODE::EUR => return "RON".to_string(),
            Currency_CODE::USD => return "SEK".to_string(),
            Currency_CODE::AUD => return "CHF".to_string(),
            Currency_CODE::EUR => return "ISK".to_string(),
            Currency_CODE::USD => return "NOK".to_string(),
            Currency_CODE::AUD => return "HRK".to_string(),
            Currency_CODE::EUR => return "RUB".to_string(),
            Currency_CODE::USD => return "TRY".to_string(),
            Currency_CODE::AUD => return "AUD".to_string(),
            Currency_CODE::EUR => return "BRL".to_string(),
            Currency_CODE::USD => return "CAD".to_string(),
            Currency_CODE::AUD => return "CNY".to_string(),
            Currency_CODE::EUR => return "HKD".to_string(),
            Currency_CODE::USD => return "IDR".to_string(),
            Currency_CODE::AUD => return "ILS".to_string(),
            Currency_CODE::EUR => return "KRW".to_string(),
            Currency_CODE::USD => return "MXN".to_string(),
            Currency_CODE::AUD => return "MYR".to_string(),
            Currency_CODE::EUR => return "NZD".to_string(),
            Currency_CODE::USD => return "PHP".to_string(),
            Currency_CODE::AUD => return "SGD".to_string(),
            Currency_CODE::USD => return "THB".to_string(),
            Currency_CODE::AUD => return "ZAR".to_string(),
            _ => "NONE".to_string()
        }
    }

    pub fn from_str(code: &str) -> Currency_CODE {
        match code {
            "EUR" => return Currency_CODE::EUR,
            "USD" => return Currency_CODE::USD,
            "JPY" => return Currency_CODE::JPY,
            "BGN" => return Currency_CODE::BGN,
            "CZK" => return Currency_CODE::CZK,
            "DKK" => return Currency_CODE::DKK,
            "GBP" => return Currency_CODE::GBP,
            "HUF" => return Currency_CODE::HUF,
            "PLN" => return Currency_CODE::PLN,
            "RON" => return Currency_CODE::RON,
            "SEK" => return Currency_CODE::SEK,
            "CHF" => return Currency_CODE::CHF,
            "ISK" => return Currency_CODE::ISK,
            "NOK" => return Currency_CODE::NOK,
            "HRK" => return Currency_CODE::HRK,
            "RUB" => return Currency_CODE::RUB,
            "TRY" => return Currency_CODE::TRY,
            "AUD" => return Currency_CODE::AUD,
            "BRL" => return Currency_CODE::BRL,
            "CAD" => return Currency_CODE::CAD,
            "CNY" => return Currency_CODE::CNY,
            "HKD" => return Currency_CODE::HKD,
            "IDR" => return Currency_CODE::IDR,
            "ILS" => return Currency_CODE::ILS,
            "KRW" => return Currency_CODE::KRW,
            "MXN" => return Currency_CODE::MXN,
            "MYR" => return Currency_CODE::MYR,
            "NZD" => return Currency_CODE::NZD,
            "PHP" => return Currency_CODE::PHP,
            "SGD" => return Currency_CODE::SGD,
            "THB" => return Currency_CODE::THB,
            "ZAR" => return Currency_CODE::ZAR,
            _ => Currency_CODE::NONE
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

    fn load_history(&mut self) -> Result<i32, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let package = client
            .get(self.convertEnum2_Code())
            .header("Accept", "text/csv");
        let respons_ret = package.send()?;
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
        Ok(-1)
    }

    pub fn get_ExchangeRate(&self, target_Currency: Currency_CODE, date: NaiveDate) -> Currency_History_Entry {
        let exchangeHistory: &Exchange_History = self.exchange_Histories.get(&target_Currency.to_str()).unwrap();

        

        return self.search_exchangeRate(&exchangeHistory.exchange_entrys, date);
    }

    fn search_exchangeRate(&self, a: &Vec<Currency_History_Entry>, search_target: NaiveDate) -> Currency_History_Entry {
        let mut low: i64 = 0;
        let mut high: i64 = a.len() as i64;
        let mut mid = ((high - low) / 2) + low;

        while low <= high {
            mid = ((high - low) / 2) + low;
            let mid_index = mid as usize;
            let val: &Currency_History_Entry = a.get(mid_index).unwrap();

            if val.TIME_PERIOD == search_target {
                return a.get(mid_index).unwrap().clone();
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
                return a.get(mid_index).unwrap().clone();
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
