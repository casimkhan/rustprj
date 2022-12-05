use std::collections::HashMap;
use std::io::BufRead;
use std::time::Duration;

struct Process {
    name: String,
    id: u32,
    recent_file_system_events: Vec<(Duration, String)>,
    recent_child_process_events: Vec<(Duration, u32)>,
    recent_thread_events: Vec<(Duration, u32)>,
}

fn find_process_events(name_or_id: &str) -> Option<Process> {
    let mut processes = HashMap::new();

    // read input from stdin
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(' ').collect();
        let timestamp = Duration::from_secs(parts[0].parse().unwrap());
        let event_type = parts[1];
        let name_or_id = parts[2];

        match event_type {
            "fs" => {
                let fs_event = parts[3];
                let process = processes.entry(name_or_id).or_insert(Process {
                    name: String::new(),
                    id: 0,
                    recent_file_system_events: Vec::new(),
                    recent_child_process_events: Vec::new(),
                    recent_thread_events: Vec::new(),
                });
                process.recent_file_system_events.push((timestamp, fs_event.to_string()));
            }
            "child" => {
                let child_id = parts[3].parse().unwrap();
                let process = processes.entry(name_or_id).or_insert(Process {
                    name: String::new(),
                    id: 0,
                    recent_file_system_events: Vec::new(),
                    recent_child_process_events: Vec::new(),
                    recent_thread_events: Vec::new(),
                });
                process.recent_child_process_events.push((timestamp, child_id));
            }
            "thread" => {
                let thread_id = parts[3].parse().unwrap();
                let process = processes.entry(name_or_id).or_insert(Process {
                    name: String::new(),
                    id: 0,
                    recent_file_system_events: Vec::new(),
                    recent_child_process_events: Vec::new(),
                    recent_thread_events: Vec::new(),
                });
                process.recent_thread_events.push((timestamp, thread_id));
            }
            _ => (),
        }
    }

    // find process with given name or id
    let process = processes.get(name_or_id);
    if let Some(process) = process {
        Some(process.clone())
    } else {
        None
    }
}

fn main() {
    let name_or_id = "process1";
    let process = find_process_events(name_or_id);
    if let Some(process) = process {
        println!("Recent file system events for process {} ({}):
