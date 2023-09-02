use std::process::Command;
use serde_json;
use log::{error, debug};

fn run_command(command: &str) -> String {
    debug!("Raw command: {command}");
    let args: Vec<&str> = command.split(" ").collect();
    debug!("Raw command split: {args:?}");
    let output = Command::new(args[0])
        .args(&args[1..])
        .output();
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.to_string()
        },
        Err(error) => {
            println!("Command failed: {command}");
            error!("error: {}", error);
            "".to_string()
        }
    }

}

// make this json serializable
#[derive(Debug, serde::Serialize)]
pub struct Filesystem {
    pub filesystem: String,
    pub size: String,
    pub used: String,
    pub available: String,
    pub use_percent: String,
    pub mounted_on: String,
}

impl Filesystem {
    pub fn new(filesystem: String, size: String, used: String, available: String, use_percent: String, mounted_on: String) -> Filesystem {
        Filesystem {
            filesystem,
            size,
            used,
            available,
            use_percent,
            mounted_on,
        }
    }
}

pub fn parse_df_output(input: &str) -> Vec<Filesystem> {
  // Parse output of df command:
  // Filesystem     1K-blocks     Used Available Use% Mounted on
  // overlay        123329088 43470228  73551072  38% /
  // tmpfs              65536        0     65536   0% /dev
  // tmpfs            6134932        0   6134932   0% /sys/fs/cgroup
  // shm                65536        0     65536   0% /dev/shm
  // /dev/vda1      123329088 43470228  73551072  38% /etc/hosts
  // tmpfs            6134932        0   6134932   0% /proc/acpi
  // tmpfs            6134932        0   6134932   0% /sys/firmware

  let mut devices: Vec<Filesystem> = Vec::new();

  for line in input.lines() {
    if line.starts_with("Filesystem") {
        debug!("Skipping header line: {line}");
      continue;
    }
    if line.len() == 0 {
        debug!("skipping that is empty");
      continue;
    }
    let mut parts = line.split_whitespace();
    let filesystem = parts.next().unwrap().to_string();
    let size = parts.next().unwrap().to_string();
    let used = parts.next().unwrap().to_string();
    let available = parts.next().unwrap().to_string();
    let use_percent = parts.next().unwrap().to_string();
    let mounted_on = parts.next().unwrap().to_string();
    let device = Filesystem::new(filesystem, size, used, available, use_percent, mounted_on);
    devices.push(device);
  }
devices
}

pub fn which_executable(command: &str) -> String {
    // find in different system paths the executable
    let acceptable_paths = vec!["/bin", "/usr/bin", "/usr/local/bin"];
    for path in acceptable_paths {
        let full_path = format!("{}/{}", path, command);
        // if the path exists then return it
        if std::path::Path::new(&full_path).exists() {
            return full_path;
        }
    }
    return command.to_string();
}

pub fn run_df(path: &str) -> serde_json::Value {
    let command = "idf";
    let output = run_command(command);
    if output.is_empty() {
        error!("No output from command: {command}");
        return serde_json::json!({});
    }

    // serialize the result of the parsing and return the JSON array
    let devices = parse_df_output(&output);
    if path.len() == 0 {
        return serde_json::json!(devices);
    } else {
        for device in devices {
            if device.mounted_on == path {
                return serde_json::json!(device);
            }
        }.into()
    }
}
