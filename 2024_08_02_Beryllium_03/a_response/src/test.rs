use crate::{parse_task_times_from_file, get_aggregated_durations, TaskDuration};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use csv::Reader;
use chrono::DateTime;
use chrono::Utc;

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_task_times_and_aggregation() {
        let sample_data = include_str!("task_times.csv");
        let file = File::create("test.csv")?;
        file.write_all(sample_data.as_bytes())?;
        parse_task_times_from_file("test.csv")?;

        let expected_durations: Vec<(String, i64)> = vec![
            ("9b317f7c-afe5-4c56-85a3-71c1c542423b", 86400000),
            ("622453f4-a277-4d5a-a258-3608563d4d97", 7200000),
            ("d5e35505-3e71-4607-8adc-67725645af60", 10800000),
            ("c678f148-d98c-43b5-8827-c8649786385b", 5400000),
        ];

        let actual_durations = get_aggregated_durations();

        let mut expected_map: HashMap<String, i64> = HashMap::new();
        for (task_id, duration) in expected_durations {
            expected_map.insert(task_id.clone(), duration);
        }

        assert_eq!(
            actual_durations,
            expected_map
                .into_iter()
                .map(|(k, v)| TaskDuration { task_id: k, total_duration: v })
                .collect(),
        );
    }
}
