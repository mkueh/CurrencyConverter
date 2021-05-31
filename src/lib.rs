mod Currency_Exchange;
use chrono::NaiveDate;

#[cfg(test)]
mod tests {
    use crate::Currency_Exchange;
    use assert_approx_eq::assert_approx_eq;
    use chrono::NaiveDate;
    use tokio::task::JoinHandle;

    #[test]
    fn USD_EUR_2020_1_15() {

        let mut Exchanger = Currency_Exchange::Exchange::new();
        let date = NaiveDate::from_ymd(2020, 1, 15);
        Exchanger.init();
        let rate = Exchanger.get_ExchangeRate(Currency_Exchange::Currency_CODE::USD,Currency_Exchange::Currency_CODE::EUR, date).unwrap();
        assert_approx_eq!(rate, 1.1142, 0.00001);
    }

    #[test]
    fn JPY_EUR_2020_1_15() {

        let mut Exchanger = Currency_Exchange::Exchange::new();
        let date = NaiveDate::from_ymd(2020, 1, 15);
        Exchanger.init();
        let rate = Exchanger.get_ExchangeRate(Currency_Exchange::Currency_CODE::JPY,Currency_Exchange::Currency_CODE::EUR, date).unwrap();
        assert_approx_eq!(rate, 122.43, 0.00001);
    }

    #[test]
    fn CAD_EUR_2020_1_15() {

        let mut Exchanger = Currency_Exchange::Exchange::new();
        let date = NaiveDate::from_ymd(2020, 1, 15);
        Exchanger.init();
        let rate = Exchanger.get_ExchangeRate(Currency_Exchange::Currency_CODE::CAD,Currency_Exchange::Currency_CODE::EUR, date).unwrap();
        assert_approx_eq!(rate, 1.4565, 0.00001);
    }

    #[test]
    fn ZAR_EUR_2020_1_15() {

        let mut Exchanger = Currency_Exchange::Exchange::new();
        let date = NaiveDate::from_ymd(2020, 1, 15);
        Exchanger.init();
        let rate = Exchanger.get_ExchangeRate(Currency_Exchange::Currency_CODE::ZAR,Currency_Exchange::Currency_CODE::EUR, date).unwrap();
        assert_approx_eq!(rate, 16.0218, 0.00001);
    }

    #[test]
    fn EUR_USD_2020_1_15() {

        let mut Exchanger = Currency_Exchange::Exchange::new();
        let date = NaiveDate::from_ymd(2020, 1, 15);
        Exchanger.init();
        let rate = Exchanger.get_ExchangeRate(Currency_Exchange::Currency_CODE::EUR, Currency_Exchange::Currency_CODE::USD, date).unwrap();
        assert_approx_eq!(rate, 0.89750, 0.00001);
    }

    #[test]
    fn JPY_USD_2020_1_15() {

        let mut Exchanger = Currency_Exchange::Exchange::new();
        let date = NaiveDate::from_ymd(2020, 1, 15);
        Exchanger.init();
        let rate = Exchanger.get_ExchangeRate(Currency_Exchange::Currency_CODE::JPY, Currency_Exchange::Currency_CODE::USD, date).unwrap();
        assert_approx_eq!(rate, 109.90374, 0.03);
    }
}
