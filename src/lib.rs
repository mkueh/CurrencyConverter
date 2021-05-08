mod Currency_Exchange;
use chrono::NaiveDate;

#[cfg(test)]
mod tests {
    use crate::Currency_Exchange;
    use assert_approx_eq::assert_approx_eq;
    use chrono::NaiveDate;

    #[test]
    fn it_works() {

        let mut Exchanger = Currency_Exchange::Exchange::new_enum(Currency_Exchange::Currency_CODE::AUD);
        let date = NaiveDate::from_ymd(2020, 1, 15);
        Exchanger.init();
        let rate = Exchanger.get_ExchangeRate(Currency_Exchange::Currency_CODE::EUR, date).unwrap();

        assert_approx_eq!(rate.OBS_VALUE.unwrap(), 1.61880, 0.00001);
        println!("{:.64}", rate.OBS_VALUE.unwrap());

    }
}
