use crate::Currency_Exchange::Currency::Currency_History;
use std::collections::HashMap;
use reqwest::Response;
use std::ops::Index;

mod Currency;


const THRESHOLD: i32 = 10;

#[derive(Clone, Debug)]
pub enum Currency_CODE{
    USD = 1,
    EUR = 2,
}

#[derive(Clone, Debug)]
pub struct Exchange {
    exchange_Histories : HashMap<String, Currency::Currency_History>,
    base_Currency: Currency_CODE,
}

impl Exchange {
    pub fn new(base_Currency: Currency_CODE) -> Exchange {
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
                let mut tmp_history = Currency_History::new();
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

    fn convertEnum2_Code(&self) -> String {
        match self.base_Currency{
            Currency_CODE::EUR => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.EUR..SP00.A".to_string(),
            Currency_CODE::USD => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.USD..SP00.A".to_string(),
            _ => "NONE".to_string()
        }
    }
}
