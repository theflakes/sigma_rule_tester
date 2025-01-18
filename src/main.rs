use std::{env, io::{self, Read}, process, time::{Duration, Instant}};
use serde::Serialize;
use serde_json::Value;
use sigma_rust::{Rule, rule_from_yaml, event_from_json};
use walkdir::WalkDir;


#[derive(Debug, Default, Serialize, Clone)]
struct MatchedRules {
    rule_title: String,
    time_taken_micro: u128,
}

#[derive(Debug, Default, Serialize)]
struct SigmaResults {
    run_number: i64,
    time_taken_milli: u128,
    time_taken_micro: u128,
    matched_rule_count: usize,
    matched_rules: Vec<MatchedRules>,
}


fn print_results(matched_rules: &Vec<MatchedRules>, total_duration: Duration, run_number: i64, pretty: bool) {
    let mut sigma_results = SigmaResults {
        run_number: run_number + 1,
        time_taken_milli: total_duration.as_millis(),
        time_taken_micro: total_duration.as_micros(),
        matched_rule_count: matched_rules.len(),
        matched_rules: matched_rules.clone(),
    };
    sigma_results.matched_rules.sort_by_key(|rule| rule.rule_title.clone());
    let mut _results: String = String::new();
    if pretty {
        _results = serde_json::to_string_pretty(&sigma_results).unwrap();
    } else {
        _results = serde_json::to_string(&sigma_results).unwrap();
    }
    println!("{}", _results);
}

fn apply_sigma_rules(log_entry: &Value, rules: &[Rule], num_runs: i64, pretty: bool) {
    let event = event_from_json(&log_entry.to_string()).unwrap();
    
    for i in 0..num_runs {
        // Clear matched_rules at the start of each run
        let mut matched_rules = Vec::new();
        
        let mut total_duration = Duration::new(0, 0);
        for rule in rules {
            let start = Instant::now();
            if rule.is_match(&event) {
                let duration = start.elapsed();
                matched_rules.push(MatchedRules {
                    rule_title: rule.title.clone(),
                    time_taken_micro: duration.as_micros(),
                });
                total_duration += duration;
            }
        }

        print_results(&matched_rules, total_duration,i, pretty);
    }
}

fn print_rule_load_results(total: i32, successful: i32, failed: i32, errors: &Vec<String>, print_errors: bool, duration: Duration) {
    if print_errors {
        println!(
            "Rule Load Summary:\n\
               - Total: {}\n\
               - Successful: {}\n\
               - Failed: {}\n\
               - Load time: {:?}\n\
               - Errors: {:?}",
            total,
            successful,
            failed,
            duration,
            errors,
        );
        return
    }
    println!(
        "Rule Load Summary:\n\
           - Total: {}\n\
           - Successful: {}\n\
           - Failed: {}\n\
           - Load time: {:?}",
        total,
        successful,
        failed,
        duration,
    );
}

fn read_file_to_string(path: &std::path::Path) -> Result<String, String> {
    let mut file = std::fs::File::open(path)
        .map_err(|e| format!("Unable to open file {:?}: {}", path, e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Unable to read file {:?}: {}", path, e))?;
    Ok(contents)
}

fn load_rules(rules_dir: &str, print_errors: bool) -> Result<Vec<Rule>, String> {
    let mut total = 0;
    let mut successful = 0;
    let mut failed = 0;
    let mut rules = vec![];
    let mut errors = vec![];

    let start = Instant::now();
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
                        "Failed to parse Sigma rule file {:?}: {:?}",
                        entry.path(),
                        err
                    ));
                }
            };
        }
    }
    let duration = start.elapsed();
    print_rule_load_results(total, successful, failed, &errors, print_errors, duration);
    Ok(rules)
}

fn main() -> io::Result<()>  {
    let (
        log, 
        rules,
        num_runs,
        print_errors,
        pretty,
    ) = get_args()?;
    let file = std::fs::File::open(log).unwrap(); 
    let reader = std::io::BufReader::new(file); 
    let log = serde_json::from_reader(reader).unwrap();
    let rules = load_rules(&rules, print_errors).unwrap();
    apply_sigma_rules(&log, &rules, num_runs, pretty);
    Ok(())
}

fn get_args() -> io::Result<(String, String, i64, bool, bool)> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 { print_help(); }
    let mut log_path = String::new();
    let mut rules_path = String::new();
    let mut get_log = false;
    let mut get_rules = false;
    let mut print_errors = false;
    let mut num_runs = 1;
    let mut get_num_runs = false;
    let mut pretty = false;
    for arg in args {
        match arg.as_str() {
            "-e" | "--errors" => print_errors = true,
            "-l" | "--log" => get_log = true,
            "-n" | "--number" => get_num_runs = true,
            "-p" | "--pretty" => pretty = true,
            "-r" | "--rules" => get_rules = true,
            _ => {
                if get_log {
                    log_path = arg.as_str().parse::<String>().unwrap();
                    get_log = false;
                } else if get_rules {
                    rules_path = arg.as_str().parse::<String>().unwrap();
                    get_rules = false;
                } else if get_num_runs {
                    num_runs =  match arg.parse::<i64>() {
                        Ok(n) => n,
                        Err(_) => {
                            eprintln!("Error: '{}' is not a valid integer.", arg);
                            process::exit(1);
                        }
                    };
                    get_num_runs = false;
                }
            }
        }
    }
    Ok((log_path, rules_path, num_runs, print_errors, pretty))
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
    -n, --number            Number of time to run the log through all rules
    -r, --rules <location>  Path to the directory containing your Sigma rules
                            - rules in sub directories will be used as well
";
    println!("{}", help);
    process::exit(1)
}