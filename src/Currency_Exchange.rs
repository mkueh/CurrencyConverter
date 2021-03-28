use crate::Currency_Exchange::Currency::Currency_History;
use reqwest::Response;

mod Currency;


const THRESHOLD: i32 = 10;

enum Currency_CODE{
    USD = 1,
    EUR = 2,
}

#[derive(Clone, Debug)]
pub struct Exchange {
    exchange_entrys : Vec<Currency::Currency_History>,
    base_Currency: Currency_CODE,
}

impl Exchange {
    pub fn new(base_Currency: Currency_CODE) -> Exchange {
        let mut ret = Exchange{
            exchange_entrys: Vec::new(),
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

        for (i,result) in deserial_result.enumerate() {
            let record:Currency::Currency_History_Entry = result.unwrap();
            self.exchange_entry.push(record);
        }

        Ok(-1)

    }

    fn convertEnum2_Code(&self) -> &str {
        match self.base_Currency{
            Currency_CODE::EUR => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.EUR..SP00.A",
            Currency_CODE::USD => return "https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.USD..SP00.A",
            _ => "NONE"
        }
    }
}
