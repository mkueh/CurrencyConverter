use std::collections::HashMap;
use std::ops::Index;

use reqwest::Response;

use Currency::Exchange_History;
use chrono::NaiveDate;
use crate::Currency_Exchange::Currency::Currency_History_Entry;

mod Currency;

const THRESHOLD: i32 = 10;

#[derive(Debug, Clone)]
pub enum Currency_CODE{
    USD,
    EUR,
    NONE,
}

impl Currency_CODE{
    pub fn to_str(&self) -> String {
        match self{
            Currency_CODE::EUR => return "EUR".to_string(),
            Currency_CODE::USD => return "USD".to_string(),
            _ => "NONE".to_string()
        }
    }

    pub fn from_str(code: &str) -> Currency_CODE{
        match code{
            "EUR" => return Currency_CODE::EUR,
            "USD" => return Currency_CODE::USD,
            _ => Currency_CODE::NONE
        }
    }
}


#[derive(Clone, Debug)]
pub struct Exchange {
    exchange_Histories : HashMap<String, Currency::Exchange_History>,
    base_Currency: Currency_CODE,
}

impl Exchange {
    pub fn new_enum(base_Currency: Currency_CODE) -> Exchange {
        let mut ret = Exchange{
            exchange_Histories: HashMap::new(),
            base_Currency
        };
        return ret;
    }

    pub fn init(&mut self) {
        self.load_history();
    }

    fn load_history(&mut self) -> Result<i32, reqwest::Error>{
        let client = reqwest::blocking::Client::new();
        let package = client.get(self.convertEnum2_Code()).header("Accept","text/csv");
        let respons_ret = package.send()?;
        let respons_unwrap = respons_ret.text().unwrap();

        let mut rdr = csv::Reader::from_reader(respons_unwrap.as_bytes());
        let mut deserial_result = rdr.deserialize();

        let mut tmp_Entry_list:Vec<Currency::Currency_History_Entry> = Vec::new();
        for (i,result) in deserial_result.enumerate() {
            let record:Currency::Currency_History_Entry = result.unwrap();
            tmp_Entry_list.push(record);
        }

        for item in tmp_Entry_list.iter_mut(){
            let target_cur = &item.CURRENCY_TARGET;
            let is_in = self.exchange_Histories.contains_key(target_cur);

            if !is_in{
                let mut tmp_history = Exchange_History::new();
                tmp_history.base_CURRENCY = self.convertEnum2_Code();
                tmp_history.target_CURRENCY = target_cur.clone();
                tmp_history.first_date = Option::from(item.TIME_PERIOD);
                let _ = self.exchange_Histories.insert(target_cur.clone(), tmp_history);

            }else{
                let mut tmp_history = self.exchange_Histories.get_mut(target_cur).unwrap();
                tmp_history.exchange_entrys.push(item.clone());
            }
        }
        Ok(-1)
    }

    pub fn get_ExchangeRate(self, target_Currency: Currency_CODE, date: NaiveDate) -> Currency_History_Entry {
        let exchangeHistory:&Exchange_History = self.exchange_Histories.get(&target_Currency.to_str()).unwrap();
        return self.search_exchangeRate(&exchangeHistory.exchange_entrys, date);
    }

    fn search_exchangeRate(self, a: &Vec<Currency_History_Entry>, search_target: NaiveDate) -> Currency_History_Entry {
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
        mid = mid-1;

        loop {
            let mid_index = mid as usize;
            let val: &Currency_History_Entry = a.get(mid_index).unwrap();
            if val.TIME_PERIOD < search_target {
                return a.get(mid_index).unwrap().clone();
            }else {
                mid = mid + 1;
            }
        }
    }

    fn convertEnum2_Code(&self) -> String {
        match self.base_Currency{
            Currency_CODE::EUR => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.EUR..SP00.A".to_string(),
            Currency_CODE::USD => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.USD..SP00.A".to_string(),
            _ => "NONE".to_string()
        }
    }
}
