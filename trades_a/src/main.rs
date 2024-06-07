// main.rs
mod models;
mod factory;
mod printer;

fn main() {
    let trades = factory::load_test_data();
    printer::print_trades(&trades);
}
