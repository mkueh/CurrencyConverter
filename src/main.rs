mod Currency_Exchange;
use chrono::NaiveDate;

fn test() -> Result<i32, reqwest::Error>{

    let client = reqwest::blocking::Client::new();
    let package = client.get("https://sdw-wsrest.ecb.europa.eu/service/data/EXR/D.USD.EUR.SP00.A?startPeriod=2020").header("Accept","text/csv");
    let respons_ret = package.send()?;
    let respons_unwrap = respons_ret.text().unwrap();

    Ok(-1)
}

fn main() {

    //test();
    //let return_value = hello_world(); // Nothing is printed
    let mut Exchanger = Currency_Exchange::Exchange::new_enum(Currency_Exchange::Currency_CODE::AUD);
    let date = NaiveDate::from_ymd(2020, 1, 15);
    Exchanger.init();
    let rate = Exchanger.get_ExchangeRate(Currency_Exchange::Currency_CODE::EUR, date);

    println!("{:.64}", rate.OBS_VALUE.unwrap());
}
