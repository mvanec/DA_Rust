#![allow(unused)]
use std::error::Error;
use std::fs::File;
use std::process;
use chrono::{NaiveDate, NaiveDateTime, TimeDelta};
use csv::Reader;
use serde;

// Define data structures

#[derive(Debug, Clone, serde::Deserialize)]
struct Project {
    project_id: String,
    project_name: String,
    project_start_date: NaiveDate,
    project_end_date: NaiveDate,
    pay_rate: f64,
    project_duration: i64,
    project_total_pay: f64,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct Task {
    task_id: String,
    project_id: String,
    task_name: String,
    task_duration: i64,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct TaskTime {
    timing_id: i32,
    task_id: String,
    start_timestamp: NaiveDateTime,
    end_timestamp: NaiveDateTime,
}

// Define functions to load CSV files

fn load_projects(file_path: &str) -> Result<Vec<Project>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = Reader::from_reader(file);

    let header = reader.headers()?;
    let expected_header = vec![
        "project_id",
        "project_name",
        "project_start_date",
        "project_end_date",
        "pay_rate",
        "project_duration",
        "project_total_pay",
    ];

    if header != expected_header {
        return Err(From::from("Invalid header in projects.csv"));
    }

    let mut projects = Vec::new();
    for record in reader.deserialize() {
        let proj: Project = record?;
        projects.push(proj);
    }

    Ok(projects)
}

fn load_tasks(file_path: &str) -> Result<Vec<Task>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = Reader::from_reader(file);

    let header = reader.headers()?;
    let expected_header = vec!["task_id", "project_id", "task_name", "task_duration"];

    if header != expected_header {
        return Err(From::from("Invalid header in tasks.csv"));
    }

    let mut tasks = Vec::new();
    for record in reader.deserialize() {
        let a_task: Task = record?;
        tasks.push(a_task);
    }

    Ok(tasks)
}

fn load_task_times(file_path: &str) -> Result<Vec<TaskTime>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = Reader::from_reader(file);

    let header = reader.headers()?;
    let expected_header = vec!["timing_id", "task_id", "start_timestamp", "end_timestamp"];

    if header != expected_header {
        return Err(From::from("Invalid header in task_times.csv"));
    }

    let mut task_times = Vec::new();
    for record in reader.deserialize() {
        let task_time: TaskTime = record?;
        task_times.push(task_time);
    }


    Ok(task_times)
}

fn calculate_delta(task_time: TaskTime) -> TimeDelta {
    task_time.end_timestamp - task_time.start_timestamp
}

fn validate_task_duration(task: &Task, task_times: &Vec<TaskTime>) -> Result<(), Box<dyn Error>> {
    let task_time_deltas = task_times
        .iter()
        .cloned()
        .filter_map(|task_time| {
            if task_time.task_id == task.task_id {
                Some(calculate_delta(task_time))
            } else {
                None
            }
        })
        .collect::<Vec<TimeDelta>>();

    let sum_duration = task_time_deltas.iter().sum::<TimeDelta>();

    if sum_duration.num_milliseconds() != task.task_duration {
        return Err(
            From::from(format!(
                "Task duration mismatch for task {}: expected {}, actual {}",
                task.task_id,
                task.task_duration,
                sum_duration.num_milliseconds(),
            )),
        );
    }

    Ok(())
}

fn calculate_project_duration(tasks: &Vec<Task>) -> i64 {
    tasks
        .iter()
        .map(|task| task.task_duration)
        .sum::<i64>()
}

fn validate_project_duration(project: &Project, tasks: &Vec<Task>) -> Result<(), Box<dyn Error>> {
    let project_duration = calculate_project_duration(&tasks);

    if project_duration != project.project_duration {
        return Err(
            From::from(format!(
                "Project duration mismatch for project {}: expected {}, actual {}",
                project.project_id,
                project.project_duration,
                project_duration,
            )),
        );
    }

    Ok(())
}

fn main() {
    let projects_file_path = "projects.csv";
    let tasks_file_path = "tasks.csv";
    let task_times_file_path = "task_times.csv";

    let projects_result = load_projects(projects_file_path);
    let tasks_result = load_tasks(tasks_file_path);
    let task_times_result = load_task_times(task_times_file_path);

    if let Err(error) = projects_result {
        eprintln!("Error loading projects: {}", error);
        process::exit(1);
    }

    if let Err(error) = tasks_result {
        eprintln!("Error loading tasks: {}", error);
        process::exit(1);
    }

    if let Err(error) = task_times_result {
        eprintln!("Error loading task times: {}", error);
        process::exit(1);
    }

    let projects = projects_result.unwrap();
    let tasks = tasks_result.unwrap();
    let task_times = task_times_result.unwrap();

    for project in projects {
        let project_tasks = tasks
            .iter()
            .cloned()
            .filter(|task| task.project_id == project.project_id)
            .collect::<Vec<Task>>();

        let project_task_times = task_times
            .iter()
            .cloned()
            .filter(|task_time| project_tasks.iter().any(|task| task.task_id == task_time.task_id))
            .collect::<Vec<TaskTime>>();

        for task in &project_tasks {
            if let Err(error) = validate_task_duration(&task, &project_task_times) {
                eprintln!("Task duration validation error: {}", error);
            }
        }

        if let Err(error) = validate_project_duration(&project, &project_tasks) {
            eprintln!("Project duration validation error: {}", error);
        }
    }
}
