use std::{env, io::{self, Read}, process, time::{Duration, Instant}};
use serde::Serialize;
use serde_json::Value;
use sigma_rust::{Rule, rule_from_yaml, event_from_json};
use walkdir::WalkDir;


#[derive(Debug, Default, Serialize)]
struct MatchedRules {
    rule_title: String,
    time_taken_micro: u128,
}

#[derive(Debug, Default, Serialize)]
struct SigmaResults {
    time_taken_milli: u128,
    time_taken_micro: u128,
    matched_rule_count: usize,
    matched_rules: Vec<MatchedRules>,
}


fn apply_sigma_rules(log_entry: &Value, rules: &[Rule]) -> (Vec<MatchedRules>, Duration) {
    let event = event_from_json(&log_entry.to_string()).unwrap();
    
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
    // matched_rules.sort_by_key(|matched_rule| &matched_rule.name);
    return (matched_rules, total_duration);
}

fn print_results(total: i32, successful: i32, failed: i32, errors: &Vec<String>, print_errors: bool, duration: Duration) {
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
    print_results(total, successful, failed, &errors, print_errors, duration);
    Ok(rules)
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
    let (mut matched_rules, duration) = apply_sigma_rules(&log, &rules);
    matched_rules.sort_by_key(|rule| rule.rule_title.clone());
    let sigma_results = SigmaResults {
        time_taken_milli: duration.as_millis(),
        time_taken_micro: duration.as_micros(),
        matched_rule_count: matched_rules.len(),
        matched_rules: matched_rules,
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