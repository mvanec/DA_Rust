// factory.rs
use crate::data_loader::DataLoader;
use crate::models::*;

pub fn load_and_transform_data(data_loader: &dyn DataLoader) -> Result<Vec<Trade>, Box<dyn Error>> {
    let trades = data_loader.load_trades()?;
    let trade_executions = data_loader.load_trade_executions()?;
    let option_details = data_loader.load_option_details()?;

    let mut trades_map: std::collections::HashMap<i32, &mut Trade> = trades.iter_mut().map(|t| (t.trade_id, t)).collect();

    for mut execution in trade_executions {
        if let Some(trade) = trades_map.get_mut(&execution.trade_id) {
            trade.executions.push(execution);
        }
    }

    for option in option_details {
        for trade in trades_map.values_mut() {
            for execution in &mut trade.executions {
                if execution.execution_id == option.execution_id {
                    execution.options.push(option);
                }
            }
        }
    }

    Ok(trades)
}
