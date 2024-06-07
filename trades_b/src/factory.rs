// factory.rs
use crate::models::*;

// Load test data from CSV
pub fn load_test_data() -> Vec<Trade> {
    let mut trades = Vec::new();

    let trade1_executions = vec![
        TradeExecution {
            execution_id: 1,
            trade_id: 1,
            execution_date_time: "2022-01-01 10:00:00".to_string(),
            spread: "STOCK".to_string(),
            quantity: 100,
            position_effect: "Open".to_string(),
            order_price: 100.0,
            fill_price: 100.0,
            commission: 5.0,
            fees: 0.5,
            reference_number: "ABC123".to_string(),
            options: vec![],
        },
        TradeExecution {
            execution_id: 2,
            trade_id: 1,
            execution_date_time: "2022-01-15 11:00:00".to_string(),
            spread: "STOCK".to_string(),
            quantity: 100,
            position_effect: "Close".to_string(),
            order_price: 120.0,
            fill_price: 120.0,
            commission: 5.0,
            fees: 0.5,
            reference_number: "DEF456".to_string(),
            options: vec![],
        },
    ];

    let trade2_executions = vec![
        TradeExecution {
            execution_id: 3,
            trade_id: 2,
            execution_date_time: "2022-02-01 12:00:00".to_string(),
            spread: "Vertical Debit".to_string(),
            quantity: 1,
            position_effect: "Open".to_string(),
            order_price: 10.0,
            fill_price: 10.0,
            commission: 1.0,
            fees: 0.1,
            reference_number: "GHI789".to_string(),
            options: vec![
                OptionDetail {
                    option_id: 1,
                    execution_id: 3,
                    expiration: "2022-03-18".to_string(),
                    strike: 800.0,
                    option_type: "CALL".to_string(),
                    quantity: 1,
                    premium: 10.0,
                    opra: "TSLA210319C00800000".to_string(),
                },
                OptionDetail {
                    option_id: 2,
                    execution_id: 3,
                    expiration: "2022-03-18".to_string(),
                    strike: 820.0,
                    option_type: "CALL".to_string(),
                    quantity: -1,
                    premium: 5.0,
                    opra: "TSLA210319C00820000".to_string(),
                },
            ],
        },
    ];

    trades.push(Trade {
        trade_id: 1,
        symbol: "AAPL".to_string(),
        open_date: "2022-01-01".to_string(),
        close_date: Some("2022-01-15".to_string()),
        broker_id: 1,
        exchange_id: 1,
        realized_gain: Some(100.0),
        executions: trade1_executions,
    });

    trades.push(Trade {
        trade_id: 2,
        symbol: "TSLA".to_string(),
        open_date: "2022-02-01".to_string(),
        close_date: None,
        broker_id: 2,
        exchange_id: 2,
        realized_gain: None,
        executions: trade2_executions,
    });

    trades
}
