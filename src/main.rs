use std::{env, fs::File, io::{self, BufReader, Read}, process, time::{Duration, Instant}};
use serde::Serialize;
use serde_json::Value;
use sigma_rust::{Rule, rule_from_yaml, event_from_json};
use walkdir::WalkDir;


#[derive(Debug, Default, Serialize, Clone)]
struct RuleResults {
    rule_title: String,
    time_taken_micro: u128,
}

#[derive(Debug, Default, Serialize, Clone)]
struct RuleLoads {
    total: i64,
    successful: i64,
    failed: i64,
    load_time_milli: u128,
    errors: Vec<String>,
}

#[derive(Debug, Default, Serialize)]
struct SigmaResultsTimed {
    rule_load: RuleLoads,
    run_number: i64,
    time_taken_milli: u128,
    time_taken_micro: u128,
    matched_rule_count: usize,
    matched_rules: Vec<RuleResults>,
    unmatched_rules: Vec<RuleResults>,
}

#[derive(Debug, Default, Serialize)]
struct SigmaResultsUntimed {
    rule_load: RuleLoads,
    run_number: i64,
    time_taken_milli: u128,
    time_taken_micro: u128,
    matched_rule_count: usize,
    matched_rules: Vec<String>,
    unmatched_rules: Vec<String>,
}


fn print_results_timed(
                    rule_loads: &RuleLoads,
                    matched_rules: &Vec<RuleResults>, 
                    unmatched_rules: &Vec<RuleResults>, 
                    total_duration: Duration, 
                    run_number: i64, 
                    pretty: bool,
                    print_unmatched: bool,
                    print_errors: bool,
                ) 
{
    let rule_load = if print_errors {
        rule_loads.clone()
    } else {
        let mut cloned = rule_loads.clone();
        cloned.errors.clear();
        cloned
    };
    let mut sigma_results = SigmaResultsTimed {
        rule_load: rule_load.clone(),
        run_number: run_number + 1,
        time_taken_milli: total_duration.as_millis(),
        time_taken_micro: total_duration.as_micros(),
        matched_rule_count: matched_rules.len(),
        matched_rules: matched_rules.clone(),
        unmatched_rules: if print_unmatched { unmatched_rules.clone() } else { Vec::new() }
    };
    sigma_results.matched_rules.sort_by_key(|rule| rule.rule_title.clone());
    sigma_results.unmatched_rules.sort_by_key(|rule| rule.rule_title.clone());
    let mut _results: String = String::new();
    if pretty {
        _results = serde_json::to_string_pretty(&sigma_results).unwrap();
    } else {
        _results = serde_json::to_string(&sigma_results).unwrap();
    }
    println!("{}", _results);
}

fn apply_sigma_rules_timed(
                            rule_loads: &RuleLoads,
                            log_entry: &Value, 
                            rules: &[Rule], 
                            num_runs: i64, 
                            pretty: bool, 
                            print_unmatched: bool,
                            print_errors: bool,
                        ) 
{
    let event = event_from_json(&log_entry.to_string()).unwrap();
    
    for i in 0..num_runs {
        let mut matched_rules = Vec::new();
        let mut unmatched_rules = Vec::new();
        let mut total_duration = Duration::new(0, 0);
        for rule in rules {
            let start = Instant::now();
            if rule.is_match(&event) {
                let duration = start.elapsed();
                matched_rules.push(RuleResults {
                    rule_title: rule.title.clone(),
                    time_taken_micro: duration.as_micros(),
                });
                total_duration += duration;
            } else {
                let duration = start.elapsed();
                unmatched_rules.push(RuleResults {
                    rule_title: rule.title.clone(),
                    time_taken_micro: duration.as_micros(),
                });
                total_duration += duration;
            }
        }

        print_results_timed(
            rule_loads, 
            &matched_rules, 
            &unmatched_rules, 
            total_duration,
            i, 
            pretty, 
            print_unmatched, 
            print_errors
        );
    }
}

fn print_results_untimed(
                            rule_loads: &RuleLoads,
                            matched_rules: &Vec<String>,
                            unmatched_rules: &Vec<String>,
                            total_duration: Duration, 
                            run_number: i64, 
                            pretty: bool,
                            print_unmatched: bool,
                            print_errors: bool,
                        ) 
{
    let rule_load = if print_errors {
        rule_loads.clone()
    } else {
        let mut cloned = rule_loads.clone();
        cloned.errors.clear();
        cloned
    };
    let mut sigma_results = SigmaResultsUntimed {
        rule_load: rule_load.clone(),
        run_number: run_number + 1,
        time_taken_milli: total_duration.as_millis(),
        time_taken_micro: total_duration.as_micros(),
        matched_rule_count: matched_rules.len(),
        matched_rules: matched_rules.clone(),
        unmatched_rules: if print_unmatched { unmatched_rules.clone() } else { Vec::new() }
    };
    sigma_results.matched_rules.sort();
    sigma_results.unmatched_rules.sort();
    let mut _results: String = String::new();
    if pretty {
        _results = serde_json::to_string_pretty(&sigma_results).unwrap();
    } else {
        _results = serde_json::to_string(&sigma_results).unwrap();
    }
    println!("{}", _results);
}

fn apply_sigma_rules_untimed(
                                rule_loads: &RuleLoads,
                                log_entry: &Value, 
                                rules: &[Rule], 
                                num_runs: i64, 
                                pretty: bool, 
                                print_unmatched: bool,
                                print_errors: bool,
                            ) 
{
    let event = event_from_json(&log_entry.to_string()).unwrap();
    
    for i in 0..num_runs {
        let mut matched_rules = Vec::new();
        let mut unmatched_rules = Vec::new();
        let mut total_duration = Duration::new(0, 0);
        for rule in rules {
            let start = Instant::now();
            if rule.is_match(&event) {
                let duration = start.elapsed();
                matched_rules.push(rule.title.clone());
                total_duration += duration;
            } else {
                let duration = start.elapsed();
                unmatched_rules.push(rule.title.clone());
                total_duration += duration;
            }
        }

        print_results_untimed(
            rule_loads, 
            &matched_rules, 
            &unmatched_rules, 
            total_duration,
            i, 
            pretty, 
            print_unmatched, 
            print_errors
        );
    }
}

fn read_file_to_string(path: &std::path::Path) -> Result<String, String> {
    let mut file = std::fs::File::open(path)
        .map_err(|e| format!("Unable to open file {:?}: {}", path, e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Unable to read file {:?}: {}", path, e))?;
    Ok(contents)
}

fn load_rules(rules_dir: &str) -> Result<(Vec<Rule>, RuleLoads), String> {
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
    let rule_loads = RuleLoads {
        total,
        successful,
        failed,
        load_time_milli: duration.as_millis(),
        errors,
    };
    Ok((rules, rule_loads))
}

fn process_logs(
                timed: bool,
                logs: &Vec<Value>,
                rule_loads: &RuleLoads,
                rules: &[Rule], 
                num_runs: i64, 
                pretty: bool, 
                print_unmatched: bool,
                print_errors: bool,
            )  
{
    for log in logs {
        if timed {
            apply_sigma_rules_timed(
                &rule_loads, 
                log, 
                &rules, 
                num_runs, 
                pretty, 
                print_unmatched, 
                print_errors
            );
        } else {
            apply_sigma_rules_untimed(
                &rule_loads, 
                log, 
                &rules, 
                num_runs, 
                pretty, 
                print_unmatched, 
                print_errors
            );
        }
    }
}

fn main() -> io::Result<()>  {
    let (
        log, 
        rules,
        num_runs,
        print_errors,
        pretty,
        print_unmatched,
        timed,
    ) = get_args()?;
    let file = File::open(log)?; 
    let reader = BufReader::new(file); 
    let entries: Value = serde_json::from_reader(reader).expect("Failed to parse JSON"); 
    let logs: Vec<Value> = if entries.is_array() { 
        serde_json::from_value(entries).expect("Failed to deserialize JSON as array of logs") 
    } else { 
        vec![serde_json::from_value(entries).expect("Failed to deserialize JSON as single log")]
    };
    let (rules, rule_loads) = load_rules(&rules).unwrap();
    process_logs(
        timed, 
        &logs, 
        &rule_loads, 
        &rules, 
        num_runs, 
        pretty, 
        print_unmatched, 
        print_errors);
    Ok(())
}

fn get_args() -> io::Result<(String, String, i64, bool, bool, bool, bool)> {
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
    let mut print_unmatched = false;
    let mut timed = false;
    for arg in args {
        match arg.as_str() {
            "-e" | "--errors" => print_errors = true,
            "-l" | "--log" => get_log = true,
            "-n" | "--number" => get_num_runs = true,
            "-p" | "--pretty" => pretty = true,
            "-r" | "--rules" => get_rules = true,
            "-t" | "--timed" => timed = true,
            "-u" | "--unmatched" => print_unmatched = true,
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
    Ok(
        (
            log_path, 
            rules_path, 
            num_runs, 
            print_errors, 
            pretty, 
            print_unmatched, 
            timed
        )
    )
}

fn print_help() {
    let help = "
Authors: Brian Kellogg
License: MIT
Purpose: Test Sigma rules against a Json log.

Usage: 
    sigma_rule_tester --log './log.json' --rules './rules'

Options:
    -e, --errors            Print out all Sigma rule load errors
    -l, --log <location>    Test Json log
    -n, --number            Number of times to run the log through all rules
    -p, --pretty            Pretty print output
    -r, --rules <location>  Path to the directory containing your Sigma rules
                            - rules in sub directories will be used as well
    -t, --timed             Track time for a rule to evaulate the log
    -u, --unmatched         Include all unmatched rules in the output
";
    println!("{}", help);
    process::exit(1)
}