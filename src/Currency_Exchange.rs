use crate::Currency_Exchange::Currency::Currency_History;
use reqwest::Response;

mod Currency;

#[derive(Clone)]
pub struct Exchange {
        pub exchange_entrys : Vec<Currency::Currency_History>,
}


impl Exchange {
    pub fn new() -> Exchange {
        let mut ret = Exchange{
            exchange_entrys: Vec::new(),
        };
        return ret;
    }

    pub fn init(&mut self) -> Result<i32, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let package = client.get("https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.USD.EUR.SP00.A?startPeriod=2020").header("Accept","text/csv");
        let respons_ret = package.send()?;
        let respons_unwrap = respons_ret.text().unwrap();

        self.exchange_entrys.push(Currency_History::new());
        let mut tmp = self.exchange_entrys[self.exchange_entrys.len()-1].clone();
        tmp.init(respons_unwrap);

        Ok(-1)
    }
}
