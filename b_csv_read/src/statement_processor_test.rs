// statement_processor_test.rs
use crate::statement_processor::process_statement_file;

#[test]
fn test_process_statement_file() {
    let file = "../test_statement.csv".to_string();
    let records = process_statement_file(&file).unwrap();
    assert_eq!(records.len(), 2);
    let record1 = records[0].clone();
    assert_eq!(record1.date.to_string(), "2022-09-09");
    assert_eq!(record1.time.to_string(), "01:05:51");
    assert_eq!(record1.quantity, 2.0);
    assert_eq!(record1.symbol, "SPX");
    assert_eq!(record1.expiration.to_string(), "2022-09-08");
    assert_eq!(record1.strike_price, 3980.0);
    assert_eq!(record1.side, "PUT");
    let record2 = records[1].clone();
    assert_eq!(record2.date.to_string(), "2022-09-10");
    assert_eq!(record2.time.to_string(), "01:05:57");
    assert_eq!(record2.quantity, 6.0);
    assert_eq!(record2.symbol, "AMZN");
    assert_eq!(record2.expiration.to_string(), "2022-09-09");
    assert_eq!(record2.strike_price, 140.0);
    assert_eq!(record2.side, "CALL");
}
