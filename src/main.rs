mod Currency_Exchange;

fn main() {
    //let return_value = hello_world(); // Nothing is printed
    let mut Exchanger = Currency_Exchange::Exchange{ exchange_entrys: vec![] };
    Exchanger.init();

    //println!("return value= {:#?}", return_value)
}