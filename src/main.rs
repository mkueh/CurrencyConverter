mod Currency_Exchange;

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
    let mut Exchanger = Currency_Exchange::Exchange::new(Currency_Exchange::Currency_CODE::USD);
    Exchanger.init();

    //println!("return value= {:#?}", return_value)
}