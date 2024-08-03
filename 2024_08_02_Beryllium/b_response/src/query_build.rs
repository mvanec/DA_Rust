// query_builder.rs

use query::Query;
use models::Project;

pub fn from_project(project: &Project) -> Query {
    // Your query building logic here.
    // This is just a simple example.
    let sql = "INSERT INTO projects VALUES (?, ?, ?, ?, ?, ?, ?)";
    let values = vec![
        &project.project_id.to_string(),
        &project.project_name,
        &project.project_start_date.to_string(),
        &project.project_end_date.to_string(),
        &project.pay_rate,
        &project.project_duration,
        &project.project_total_pay,
    ];
    Query { sql, values }
}
