use csv::ReaderBuilder;
use std::error::Error;

pub fn load_test_data() -> Result<Vec<Trade>, Box<dyn Error>> {
    let mut trades = Vec::new();

    let mut rdr = ReaderBuilder::new().from_path("trades.csv")?;
    for result in rdr.deserialize() {
        let (trade_id, symbol, open_date, close_date, broker_id, exchange_id, realized_gain): (i32, String, String, Option<String>, i32, i32, Option<f64>) = result?;
        trades.push(Trade {
            trade_id,
            symbol,
            open_date,
            close_date,
            broker_id,
            exchange_id,
            realized_gain,
            executions: Vec::new(),
        });
    }

    let mut rdr = ReaderBuilder::new().from_path("trade_executions.csv")?;
    for result in rdr.deserialize() {
        let (execution_id, trade_id, execution_date_time, spread, quantity, position_effect, order_price, fill_price, commission, fees, reference_number): (i32, i32, String, String, i32, String, f64, f64, f64, f64, String) = result?;
        let trade = trades.iter_mut().find(|trade| trade.trade_id == trade_id).unwrap();
        trade.executions.push(TradeExecution {
            execution_id,
            trade_id,
            execution_date_time,
            spread,
            quantity,
            position_effect,
            order_price,
            fill_price,
            commission,
            fees,
            reference_number,
            options: Vec::new(),
        });
    }

    let mut rdr = ReaderBuilder::new().from_path("options_details.csv")?;
    for result in rdr.deserialize() {
        let (option_id, execution_id, expiration, strike, option_type, quantity, premium, opra): (i32, i32, String, f64, String, i32, f64, String) = result?;
        let trade = trades.iter_mut().find(|trade| trade.executions.iter().any(|execution| execution.execution_id == execution_id)).unwrap();
        let execution = trade.executions.iter_mut().find(|execution| execution.execution_id == execution_id).unwrap();
        execution.options.push(OptionDetail {
            option_id,
            execution_id,
            expiration,
            strike,
            option_type,
            quantity,
            premium,
            opra,
        });
    }

    Ok(trades)
}
