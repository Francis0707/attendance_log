use chrono::{Local,};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self,};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct AttendanceLog {
    employee_id: String,
    logs: Vec<WorkLog>,
}

#[derive(Serialize, Deserialize, Debug)]
struct WorkLog {
    clock_in: chrono::NaiveDateTime,
    clock_out: Option<chrono::NaiveDateTime>, // None if not clocked out yet
}


impl AttendanceLog {
    fn new(employee_id: &str) -> Self {
        Self {
            employee_id: employee_id.to_string(),
            logs: Vec::new(),
        }
    }

    fn clock_in(&mut self) {
        let now = Local::now().naive_local();
        if let Some(last_log) = self.logs.last() {
            if last_log.clock_out.is_none() {
                println!("You have already clocked in without clocking out.");
                return;
            }
        }
        self.logs.push(WorkLog {
            clock_in: now,
            clock_out: None,
        });
        println!("Clocked in at {}", now);
    }

    fn clock_out(&mut self) {
        if let Some(last_log) = self.logs.last_mut() {
            if last_log.clock_out.is_none() {
                let now = Local::now().naive_local();
                last_log.clock_out = Some(now);
                println!("Clocked out at {}", now);
                return;
            }
        }
        println!("You must clock in before clocking out.");
    }

    fn display_logs(&self) {
        println!("\nAttendance Logs for Employee ID: {}", self.employee_id);
        for (i, log) in self.logs.iter().enumerate() {
            println!(
                "{}. Clock In: {}, Clock Out: {}",
                i + 1,
                log.clock_in,
                log.clock_out
                    .map(|t| t.to_string())
                    .unwrap_or("Not Clocked Out".to_string())
            );
        }
    }

    fn save_to_file(&self) -> io::Result<()> {
        let filename = format!("logs/{}.json", self.employee_id);
        let file = File::create(&filename)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    fn load_from_file(employee_id: &str) -> Option<Self> {
        let filename = format!("logs/{}.json", employee_id);
        if Path::new(&filename).exists() {
            if let Ok(file) = File::open(&filename) {
                if let Ok(log) = serde_json::from_reader(file) {
                    return Some(log);
                }
            }
        }
        None
    }
}

fn main() {
    // Ensure the logs directory exists
    fs::create_dir_all("logs").expect("Failed to create logs directory.");

    println!("Welcome to the Attendance Log Manager!");

    let mut employees: HashMap<String, AttendanceLog> = HashMap::new();

    loop {
        println!("\nOptions:");
        println!("1. Clock In");
        println!("2. Clock Out");
        println!("3. View Logs");
        println!("4. Exit");
        print!("Enter your choice: ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("Enter your Employee ID:");
                let mut employee_id = String::new();
                io::stdin().read_line(&mut employee_id).unwrap();
                let employee_id = employee_id.trim();

                let log = employees
                    .entry(employee_id.to_string())
                    .or_insert_with(|| AttendanceLog::load_from_file(employee_id).unwrap_or_else(|| AttendanceLog::new(employee_id)));

                log.clock_in();
                if let Err(e) = log.save_to_file() {
                    eprintln!("Failed to save logs: {}", e);
                }
            }
            "2" => {
                println!("Enter your Employee ID:");
                let mut employee_id = String::new();
                io::stdin().read_line(&mut employee_id).unwrap();
                let employee_id = employee_id.trim();

                if let Some(log) = employees.get_mut(employee_id) {
                    log.clock_out();
                    if let Err(e) = log.save_to_file() {
                        eprintln!("Failed to save logs: {}", e);
                    }
                } else {
                    println!("Employee ID not found. Please clock in first.");
                }
            }
            "3" => {
                println!("Enter your Employee ID:");
                let mut employee_id = String::new();
                io::stdin().read_line(&mut employee_id).unwrap();
                let employee_id = employee_id.trim();

                if let Some(log) = employees.get(employee_id) {
                    log.display_logs();
                } else if let Some(log) = AttendanceLog::load_from_file(employee_id) {
                    log.display_logs();
                } else {
                    println!("No logs found for Employee ID: {}", employee_id);
                }
            }
            "4" => {
                println!("Exiting Attendance Log Manager. Goodbye!");
                break;
            }
            _ => println!("Invalid choice! Please try again."),
        }
    }
}
