use std::{env, io::{self, Read}, process};
use serde::Serialize;
use serde_json::Value;
use sigma_rust::{Rule, rule_from_yaml, event_from_json};
use walkdir::WalkDir;

#[derive(Debug, Default, Serialize)]
struct SigmaResults {
    rule_count: usize,
    rules: Vec<Value>,
}

fn apply_sigma_rules(log_entry: &Value, rules: &Vec<Rule>) -> Vec<Value> {
    let mut matched_rules = Vec::new();
    let event = event_from_json(&log_entry.to_string()).unwrap();

    for rule in rules {
        if rule.is_match(&event) {
            matched_rules.push(Value::String(rule.title.clone()));
        }
    }

    return matched_rules
}

fn print_results(total: i32, successful: i32, failed: i32, errors: &Vec<String>, print_errors: bool) {
    if print_errors {
        println!(
            "Rule Load Summary:\n\
               - Total: {}\n\
               - Successful: {}\n\
               - Failed: {}\n\
               - Errors: {:?}",
            total,
            successful,
            failed,
            errors,
        );
        return
    }
    println!(
        "Rule Load Summary:\n\
           - Total: {}\n\
           - Successful: {}\n\
           - Failed: {}",
        total,
        successful,
        failed,
    );
}

fn load_rules(rules_dir: &str, print_errors: bool) -> Result<Vec<Rule>, String> {
    let mut total = 0;
    let mut successful = 0;
    let mut failed = 0;
    let mut rules = vec![];
    let mut errors = vec![];

    for entry in WalkDir::new(rules_dir).into_iter().filter_map(|e| e.ok()) {
        let extension = match entry.path().extension().and_then(|s| s.to_str()) {
            Some(ext) => ext.to_lowercase(),
            None => continue,
        };

        if extension == "yml" || extension == "yaml" {
            total += 1;

            let contents = read_file_to_string(entry.path())?;
            match rule_from_yaml(&contents) {
                Ok(r) => {
                    successful += 1;
                    rules.push(r);
                }
                Err(err) => {
                    failed += 1;
                    errors.push(format!(
                        "Failed to parse YAML file {:?}: {:?}",
                        entry.path(),
                        err
                    ));
                }
            };
        }
    }
    print_results(total, successful, failed, &errors, print_errors);
    Ok(rules)
}

fn read_file_to_string(path: &std::path::Path) -> Result<String, String> {
    let mut file = std::fs::File::open(path)
        .map_err(|e| format!("Unable to open file {:?}: {}", path, e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Unable to read file {:?}: {}", path, e))?;
    Ok(contents)
}

fn main() -> io::Result<()>  {
    let (
        log, 
        rules,
        print_errors,
    ) = get_args()?;
    let file = std::fs::File::open(log).unwrap(); 
    let reader = std::io::BufReader::new(file); 
    let log = serde_json::from_reader(reader).unwrap();
    let rules = load_rules(&rules, print_errors).unwrap();
    let mut matched_rules = apply_sigma_rules(&log, &rules);
    matched_rules.sort_by(|a, b| { 
        a.as_str().unwrap_or("").cmp(&b.as_str().unwrap_or(""))
    });
    let sigma_results = SigmaResults {
        rule_count: matched_rules.len(), 
        rules: matched_rules,
    };
    let pretty_printed = serde_json::to_string_pretty(&sigma_results).unwrap();
    println!("\nResults:\n{}", pretty_printed);
    Ok(())
}

fn get_args() -> io::Result<(String, String, bool)> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 { print_help(); }
    let mut log_path = String::new();
    let mut rules_path = String::new();
    let mut get_log = false;
    let mut get_rules = false;
    let mut print_errors = false;
    for arg in args {
        match arg.as_str() {
            "-e" | "--errors" => print_errors = true,
            "-l" | "--log" => get_log = true,
            "-r" | "--rules" => get_rules = true,
            _ => {
                if get_log {
                    log_path = arg.as_str().parse::<String>().unwrap();
                    get_log = false;
                } else if get_rules {
                    rules_path = arg.as_str().parse::<String>().unwrap();
                    get_rules = false;
                }
            }
        }
    }
    Ok((log_path, rules_path, print_errors))
}

fn print_help() {
    let help = "
Authors: Brian Kellogg
License: MIT
Purpose: Test Sigma rules against a Json log.

Usage: 
    sigma_rule_tester --log './log.json' --rules './rules'

Options:
    -e, --errors            Print out all Sigma rule loading errors
    -l, --log <location>    Test Json log
    -r, --rules <location>  Path to the directory containing your Sigma rules
                            - rules in sub directories will be used as well
";
    println!("{}", help);
    process::exit(1)
}