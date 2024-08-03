use std::error::Error;
use std::fs::File;
use csv::Reader;
use chrono::{TimeDelta, NaiveDateTime};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct TaskTime {
    timing_id: i32,
    task_id: String,
    start_timestamp: NaiveDateTime,
    end_timestamp: NaiveDateTime,
}

#[derive(Debug)]
pub struct TaskDuration {
    task_id: String,
    duration: TimeDelta,
}

impl TaskTime {
    fn duration(&self) -> TimeDelta {
        let diff = self.end_timestamp - self.start_timestamp;
        diff
    }
}

pub fn parse_task_times_from_file(filename: &str) -> Result<Vec<TaskTime>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut reader = Reader::from_reader(file);

    let mut task_times = Vec::new();
    for record in reader.deserialize() {
        let task_time: TaskTime = record?;
        task_times.push(task_time);
    }
    Ok(task_times)
}

pub fn calculate_duration_by_task(task_times: Vec<TaskTime>) -> Vec<TaskDuration> {
    let mut grouped_tasks: Vec<TaskDuration> = Vec::new();
    for task in task_times {
        let existing = grouped_tasks.iter_mut().find(|t| t.task_id == task.task_id);
        match existing {
            Some(existing) => {
                existing.duration += task.duration();
            }
            None => {
                grouped_tasks.push(TaskDuration {
                    task_id: task.task_id.clone(),
                    duration: task.duration(),
                })
            }
        }
    }
    grouped_tasks
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_parse_task_times_from_file() -> Result<(), Box<dyn Error>> {
        let input: Vec<&str> = vec![
            "timing_id,task_id,start_timestamp,end_timestamp",
            "1,9b317f7c-afe5-4c56-85a3-71c1c542423b,2023-08-01T08:00:00,2023-08-01T10:00:00",
            "2,9b317f7c-afe5-4c56-85a3-71c1c542423b,2023-08-01T14:00:00,2023-08-01T16:00:00",
            "3,622453f4-a277-4d5a-a258-3608563d4d97,2023-08-01T11:00:00,2023-08-01T13:00:00",
            "4,622453f4-a277-4d5a-a258-3608563d4d97,2023-08-01T15:00:00,2023-08-01T16:30:00",
            "5,d5e35505-3e71-4607-8adc-67725645af60,2023-08-03T09:00:00,2023-08-03T11:00:00",
            "6,d5e35505-3e71-4607-8adc-67725645af60,2023-08-03T12:30:00,2023-08-03T14:00:00",
            "7,c678f148-d98c-43b5-8827-c8649786385b,2023-08-03T14:30:00,2023-08-03T16:00:00",
            "8,c678f148-d98c-43b5-8827-c8649786385b,2023-08-03T16:00:00,2023-08-03T17:30:00",
        ];
        let mut file = File::create("test.csv").expect("Unable to write file test.csv");
        for line in input {
            file.write_all(&line.as_bytes())?;
            file.write_all("\n".as_bytes())?;
        }

        let times = parse_task_times_from_file("test.csv")?;
        assert_eq!(times.len(), 8);
        Ok(())
    }

    #[test]
    fn test_calculate_duration_by_task() {
        let task_times: Vec<TaskTime> = vec![
            TaskTime {
                timing_id: 1,
                task_id: String::from("9b317f7c-afe5-4c56-85a3-71c1c542423b"),
                start_timestamp: NaiveDateTime::parse_from_str("2023-08-01T08:00:00", "%Y-%m-%dT%H:%M:%S").unwrap() ,
                end_timestamp: NaiveDateTime::parse_from_str("2023-08-01T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            },
            TaskTime {
                timing_id: 2,
                task_id: String::from("9b317f7c-afe5-4c56-85a3-71c1c542423b"),
                start_timestamp: NaiveDateTime::parse_from_str("2023-08-01T14:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                end_timestamp: NaiveDateTime::parse_from_str("2023-08-01T16:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            },
            // ... other task times
        ];
        let td1 = TimeDelta::from_std(Duration::from_secs(4 * 60 * 60)).unwrap();

        let expected_durations: Vec<(String, TimeDelta)> = vec![
            ("9b317f7c-afe5-4c56-85a3-71c1c542423b".to_string(), td1),
        ];

        let actual_durations = calculate_duration_by_task(task_times);

        for (expected_task_id, expected_duration) in expected_durations.iter() {
            let actual_duration = &actual_durations
                .iter()
                .filter(|d| &d.task_id == expected_task_id)
                .next()
                .map_or(TimeDelta::zero(), |d| d.duration);

            assert_eq!(
                actual_duration,
                expected_duration,
                "Duration for task '{}' mismatch", expected_task_id
            );
        }
    }
}
