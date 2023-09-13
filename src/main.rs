use colored::Colorize;
use std::{fs, process::exit};

const PACKAGE_JSON: &str = "package.json";

fn main() {
    let pkg_raw_opt = fs::read_to_string(PACKAGE_JSON);
    match pkg_raw_opt {
        Ok(pkg_raw) => {
            let package_json: serde_json::Value = serde_json::from_str(&pkg_raw).unwrap();

            let scripts = package_json["scripts"].as_object().unwrap();

            let mut len = 0;
            for (key, _) in scripts {
                if key.len() > len {
                    len = key.len();
                }
            }
            len += 2;
            for (key, value) in scripts {
                let val_fmt = value.as_str().replace("\"").unwrap();
                let key_fmt = format!("{}:", key).bold();
                println!("{:width$} {}", key_fmt, val_fmt, width = len);
            }
        }
        Err(_) => {
            println!("");
            exit(1);
        }
    }
}
