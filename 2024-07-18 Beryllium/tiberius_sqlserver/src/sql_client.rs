use anyhow::Ok;
use async_std::net::TcpStream;
use tiberius::{Client, Config, AuthMethod};
use tiberius::SqlBrowser;


/// Connect to a named instance of SQL Server through SQL Server Browser.
/// Make sure that SQL Server Browser is installed and running, otherwise
/// the connection will fail.
pub async fn connect_through_sql_browser() -> anyhow::Result<()> {
    let mut config = Config::new();

    config.host("127.0.0.1");
    config.port(1433);
    config.authentication(AuthMethod::sql_server("SA", "Mssql_da_1"));
    config.trust_cert();

    // This will create a new `TcpStream` from `async-std`, connected to the
    // right port of the named instance.
    let tcp = TcpStream::connect_named(&config).await?;

    // Perform the real connection to the SQL Server
    let client = Client::connect(config, tcp).await?;
    println!("Successfully connected to server.");

    // And then close the connection.
    client.close().await?;
    Ok(())
}

/// Connect to an SQL Server instance using the hostname and port number.
pub async fn connect_through_port() -> anyhow::Result<()> {

    let config = Config::from_ado_string(&CONN_STR_PORT)?;

    // Create a `TCPStream` from the `async-std` library with
    // a address that contains the hostname/IP and port number.
    let tcp = TcpStream::connect(config.get_addr()).await?;

    tcp.set_nodelay(true)?;

    // Connect to SQL Server
    let client = Client::connect(config, tcp).await?;
    eprintln!("Successfully connected to server.");

    client.close().await?;

    Ok(())
}

