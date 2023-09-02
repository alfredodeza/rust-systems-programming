use flate2::read::GzDecoder;
use regex::Regex;
use std::env;
use std::fs::File;

fn read_buffer(file_path: &str) {
    // Initialize variables for error rate calculation.
    let mut total_entries = 0;
    let mut error_entries = 0;
    let mut current_hour = None;

    let timestamp_regex = Regex::new(r"(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}-\d{2})").unwrap();
    let error_keyword = "Error";
    let file = File::open(file_path).unwrap();
    use std::io::{BufRead, BufReader};

    let reader: Box<dyn BufRead> = match file_path.ends_with(".gz") {
        true => {
            // Decompress gzipped file.
            let decompressor = GzDecoder::new(file);
            Box::new(BufReader::new(decompressor))
        }
        false => Box::new(BufReader::new(file)),
    };

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                continue;
            }
        };
        // Extract timestamp from the log line.
        if let Some(captures) = timestamp_regex.captures(&line) {
            let timestamp = &captures[1];
            // Extract the date and hour part of the timestamp, assuming it's in "YYYY-MM-DD HH:MM:SS-ZZ" format.
            let date = &timestamp[0..10];
            let hour = &timestamp[11..13];
            let date_hour = format!("{}, Hour: {}", date, hour);

            // Check if the hour has changed.
            if current_hour != Some(date_hour.to_string()) {
                // Calculate and print error count for the previous hour.
                if let Some(prev_hour) = current_hour.take() {
                    println!("{prev_hour} - Error Count: {error_entries}");
                }

                // Reset counters for the new hour.
                error_entries = 0;
                current_hour = Some(date_hour.to_string());
            }

            // Check if the log entry contains an error.
            if line.contains(error_keyword) {
                error_entries += 1;
                total_entries += 1;
            }
        }
    }

    println!("Total error count for current log: {total_entries}");
}
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: log_error_rate <log_file_path>");
        std::process::exit(1);
    }

    let log_file_path = &args[1];

    read_buffer(log_file_path);
}
