// factory.rs
use models::{Trade, TradeExecution, OptionDetails};
use std::collections::VecDeque;

pub fn load_test_data() -> VecDeque<Trade> {
    let mut trades = VecDeque::new();

    let mut trade = Trade {
        trade_id: 1,
        symbol: "AAPL".to_string(),
        open_date: "2022-01-01".to_string(),
        close_date: Some("2022-01-31".to_string()),
        broker_id: 1,
        exchange_id: 1,
        realized_gain: Some(100.0),
        executions: VecDeque::new(),
    };

    let mut execution = TradeExecution {
        execution_id: 1,
        execution_date_time: "2022-01-01 10:00:00".to_string(),
        spread: "Market Order".to_string(),
        quantity: 100,
        position_effect: "Open".to_string(),
        order_price: 100.0,
        fill_price: 100.0,
        commission: 5.0,
        fees: 1.0,
        reference_number: "ABC123".to_string(),
        options: VecDeque::new(),
    };

    let option = OptionDetails {
        option_id: 1,
        expiration: "2022-02-01".to_string(),
        strike: 100.0,
        option_type: "CALL".to_string(),
        quantity: 100,
        premium: 5.0,
        opra: "ABC123".to_string(),
    };

    execution.options.push_back(option);
    trade.executions.push_back(execution);
    trades.push_back(trade);

    trades
}
