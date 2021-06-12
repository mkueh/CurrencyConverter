# CurrencyConverter

My small currencyconvert in Rust, that only support European Central Bank exchange rates. This is my first project in Rust :)

## Example


        let mut Exchanger = Currency_Exchange::Exchange::new();
        let date = NaiveDate::from_ymd(2020, 1, 15);
        Exchanger.init();
        let rate = Exchanger.get_ExchangeRate(Currency_Exchange::Currency_CODE::USD,Currency_Exchange::Currency_CODE::EUR, date).unwrap();
        assert_approx_eq!(rate, 1.1142, 0.00001);
        
Exchange::new() automatically loads the current exchange rates from the ECB during initialization (https://sdw-wsrest.ecb.europa.eu/service/data/EXR/)          
