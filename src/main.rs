use chrono::Duration;
use colored::*;
use std::io::{self, Write};
use std::thread;
use std::time::Instant;

fn main() {
    let work_duration = get_duration("Enter work duration in minutes: ");
    let short_break_duration = get_duration("Enter short break duration in minutes: ");
    let long_break_duration = get_duration("Enter long break duration in minutes: ");
    let cycles = get_cycles("Enter the number of cycles: ");
    let long_break_after_cycles = get_cycles("Enter the number of cycles before a long break: ");

    for cycle in 0..cycles {
        println!("{}", format!("Cycle {}: Work for {} minutes", cycle + 1, work_duration.num_minutes()).green());
        start_timer(work_duration);

        if (cycle + 1) % long_break_after_cycles == 0 {
            println!("{}", format!("Take a long break for {} minutes", long_break_duration.num_minutes()).blue());
            start_timer(long_break_duration);
        } else if cycle < cycles - 1 {
            println!("{}", format!("Take a short break for {} minutes", short_break_duration.num_minutes()).yellow());
            start_timer(short_break_duration);
        }
    }

    println!("{}", "Pomodoro session complete! Take a well-deserved break.".magenta());
}

fn get_duration(prompt: &str) -> Duration {
    let minutes = get_input(prompt).parse::<i64>().unwrap_or(25);
    Duration::minutes(minutes)
}

fn get_cycles(prompt: &str) -> i32 {
    get_input(prompt).parse::<i32>().unwrap_or(4)
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn start_timer(duration: Duration) {
    let start = Instant::now();
    let duration_secs = duration.num_seconds();

    while Instant::now() - start < duration.to_std().unwrap() {
        let elapsed = Instant::now() - start;
        let remaining_secs = duration_secs - elapsed.as_secs() as i64;
        let minutes = remaining_secs / 60;
        let seconds = remaining_secs % 60;

        print!("\rTime left: {:02}:{:02}", minutes, seconds);
        io::stdout().flush().unwrap();
        thread::sleep(std::time::Duration::from_secs(1));
    }
    println!(); // Move to the next line after the timer ends.
}
