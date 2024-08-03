use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;
use csv::Reader;
use chrono::Duration;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct TaskTime {
    timing_id: i32,
    task_id: String,
    start_timestamp: DateTime<Utc>,
    end_timestamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct TaskDuration {
    task_id: String,
    total_duration: i64,
}

// Define the error type for this library
struct ParseError;

impl Error for ParseError {
    fn description(&self) -> &str {
        "Error parsing task time data"
    }
}

pub fn parse_task_times_from_file(filename: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = Reader::from_reader(file);

    for result in reader.deserialize_record(TaskTime::deserialize)? {
        let task_time = result?;
        let duration = calculate_duration(&task_time);
        match aggregate_duration(&task_time.task_id, duration) {
            Ok(_) => (),
            Err(e) => return Err(Box::new(e)),
        }
    }

    Ok(())
}

fn calculate_duration(task_time: &TaskTime) -> i64 {
    task_time.end_timestamp.diff(task_time.start_timestamp).as_millis()
}

pub fn get_aggregated_durations() -> Vec<TaskDuration> {
    let task_durations = aggregate_durations::get_mut();
    task_durations.drain()
}

fn aggregate_duration(task_id: &str, duration: i64) -> Result<(), ParseError> {
    aggregate_durations::insert(task_id, duration);
    Ok(())
}

lazy_static! {
    static ref mut aggregate_durations: Mutex<HashMap<String, i64>> = Mutex::new(HashMap::new());
}
