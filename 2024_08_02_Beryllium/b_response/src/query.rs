// query.rs

pub struct Query<'a> {
    sql: &'a str,
    values: Vec<&'a Value>,
}
