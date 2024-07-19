use anyhow::Ok;
use async_std::net::TcpStream;
use futures_util::stream::TryStreamExt;
use once_cell::sync::Lazy;
use tiberius::numeric::Decimal;
use std::env;
use std::fmt;
use tiberius::{Client, Config, Query, QueryItem};

#[derive(Debug)]
struct Book {
    title: String,
    publication_date: chrono::NaiveDate,
    edition: String,
    retail_price: Decimal,
}

impl fmt::Display for Book {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let disp_str = format!(
            "{:30} | {:10} | {:10} |$ {:0.2}",
            self.title,
            self.publication_date.format("%Y-%m-%d"),
            self.edition,
            self.retail_price
        );
        write!(f, "{}", disp_str)
    }
}

static CONN_STR_PORT: Lazy<String> = Lazy::new(|| {
    env::var("TIBERIUS_TEST_CONNECTION_STRING").unwrap_or_else(|_| {
        "server=tcp:localhost,1433;IntegratedSecurity=false;TrustServerCertificate=true;user=SA;password=SuperSecretPassword;database=DataAnnotation".to_owned()
    })
});

pub async fn read_table() -> anyhow::Result<()> {
    let config = Config::from_ado_string(&CONN_STR_PORT)?;
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;
    let select =
        Query::new("SELECT Title, PublicationDate, Edition, RetailPrice FROM Bookstore.Books");
    let mut stream = select.query(&mut client).await?;

    // Read each row as long as ther arrive from the stream
    while let Some(row) = stream.try_next().await? {
        match row {
            // Metadata of the result set
            QueryItem::Metadata(meta) => {
                eprintln!("{:?}", meta);
            }

            // Actual data rows
            QueryItem::Row(r) => {
                // Break line to separate each row
                println!();

                // Create variables with an explicit type annotation.
                let title_column: Option<&str> = r.get(0);
                let pub_date_column: Option<chrono::NaiveDate> = r.get(1);
                let edition_column: Option<&str> = r.get(2);
                let price_column: Option<Decimal> = r.get(3);

                // The complete list of SQL Server data types
                // matching with Rust data types can be found in:
                // https://docs.rs/tiberius/latest/tiberius/trait.FromSql.html#tymethod.from_sql

                let mut book: Book = Book {
                    title: "".to_string(),
                    publication_date: chrono::NaiveDate::parse_from_str("1900-01-01", "%Y-%m-%d")
                        .unwrap(),
                    edition: "".to_string(),
                    retail_price: Decimal::MIN,
                };

                if let Some(value) = title_column {
                    book.title = value.to_string();
                }

                if let Some(value) = edition_column {
                    book.edition = value.to_string();
                }

                if let Some(value) = pub_date_column {
                    book.publication_date = value;
                }

                if let Some(value) = price_column {
                    book.retail_price = value;
                }

                println!("{book}");
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn test_read_table() {
        let result = read_table().await;
        eprintln!("Error is {:?}", result);
        assert_eq!(result.is_ok(), true);
    }
}
