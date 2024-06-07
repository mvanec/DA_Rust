// printer.rs
use models::{Trade, TradeExecution, OptionDetails};
use std::collections::VecDeque;

pub fn print_trades(trades: &VecDeque<Trade>) {
    let mut trade_symbol_width = 0;
    let mut execution_spread_width = 0;
    let mut option_strike_width = 0;

    for trade in trades {
        trade_symbol_width = trade_symbol_width.max(trade.symbol.len());
        for execution in &trade.executions {
            execution_spread_width = execution_spread_width.max(execution.spread.len());
            for option in &execution.options {
                option_strike_width = option_strike_width.max(format!("{}", option.strike).len());
            }
        }
    }

    for trade in trades {
        println!("| {:width$} | {} | {} | {} | {} | {} | {} |",
                 trade.symbol, trade.open_date, trade.close_date.as_ref().unwrap_or(&"".to_string()), trade.broker_id, trade.exchange_id, trade.realized_gain.as_ref().unwrap_or(&0.0), "|",
                 width = trade_symbol_width);

        for execution in &trade.executions {
            println!("|     | {:width$} | {} | {} | {} | {} | {} | {} | {} | {} |",
                     execution.spread, execution.execution_date_time, execution.quantity, execution.position_effect, execution.order_price, execution.fill_price, execution.commission, execution.fees, "|",
                     width = execution_spread_width);

            for option in &execution.options {
                println!("|         | {:width$} | {} | {} | {} | {} | {} |",
                         format!("{}", option.strike), option.expiration, option.option_type, option.quantity, option.premium, option.opra, "|",
                         width = option_strike_width);
            }
        }
    }
}
