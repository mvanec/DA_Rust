// factory.rs
use crate::models::*;

pub fn load_test_data() -> Vec<Trade> {
    let mut trades = Vec::new();

    let mut trade_executions = Vec::new();
    let mut option_details = Vec::new();

    option_details.push(OptionDetail {
        option_id: 1,
        execution_id: 1,
        expiration: "2024-06-30".to_string(),
        strike: 100.0,
        option_type: "CALL".to_string(),
        quantity: 10,
        premium: 10.0,
        opra: "OPRA1".to_string(),
    });

    trade_executions.push(TradeExecution {
        execution_id: 1,
        trade_id: 1,
        execution_date_time: "2024-06-07 12:00:00".to_string(),
        spread: "0.01".to_string(),
        quantity: 10,
        position_effect: "Open".to_string(),
        order_price: 100.0,
        fill_price: 100.0,
        commission: 10.0,
        fees: 10.0,
        reference_number: "Ref1".to_string(),
        options: option_details,
    });

    trades.push(Trade {
        trade_id: 1,
        symbol: "AAPL".to_string(),
        open_date: "2024-06-07".to_string(),
        close_date: None,
        broker_id: 1,
        exchange_id: 1,
        realized_gain: None,
        executions: trade_executions,
    });

    trades
}
