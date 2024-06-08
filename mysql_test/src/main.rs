use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://data:dataannotationd@localhost:3306/dataannotation")
        .await?;

    let trades = sqlx::query_as!(
        Trade,
        "SELECT TradeID, Symbol, OpenDate, CloseDate, BrokerID, ExchangeID, RealizedGain FROM trades",
    )
    .fetch_all(&pool)
    .await?;

    for trade in trades {
        println!("{:?}", trade);
    }

    Ok(())
}

#[derive(Debug, sqlx::FromRow)]
struct Trade {
    TradeID: i32,
    Symbol: String,
    OpenDate: String,
    CloseDate: Option<String>,
    BrokerID: i32,
    ExchangeID: i32,
    RealizedGain: Option<f64>,
}
