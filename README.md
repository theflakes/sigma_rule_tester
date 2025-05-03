# sigma_rule_tester

Test any number of rules against a Json log to see which rules fire.  

Many thanks to Dr.-Ing. Johannes Pohl for his `sigma-rust` crate: https://github.com/jopohl/sigma-rust  

```
Authors: Brian Kellogg
License: MIT
Purpose: Test Sigma rules against a Json log.

Note: 
    When a rule uses simple glob matching, '*' and '?', the first time this
    logic is encountered it is converted into a regex and cached for quicker
    execution of that logic on subsequent logs its run against.

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
```
Output
```
# ./sigma_rule_tester -r ./config/sigma_rules/ -l ./log.json -p -n 2 --timed
{
  "rule_load": {
    "total": 2989,
    "successful": 2989,
    "failed": 0,
    "load_time_milli": 207,
    "errors": []
  },
  "run_number": 1,
  "time_taken_milli": 10,
  "time_taken_micro": 10671,
  "matched_rule_count": 17,
  "matched_rules": [
    {
      "rule_title": "ALERT - ADS redirection used",
      "time_taken_micro": 49
    },
    {
      "rule_title": "ALERT - ADS redirection used and CMD line exists",
      "time_taken_micro": 42
    },
    {
      "rule_title": "ALERT - Sus Parent to Child commandline",
      "time_taken_micro": 141
    },
    {
      "rule_title": "Example Sigma Rule",
      "time_taken_micro": 434
    },
    {
      "rule_title": "Execution Of Non-Existing File",
      "time_taken_micro": 5
    },
    {
      "rule_title": "Failed Authentications From Countries You Do Not Operate Out Of",
      "time_taken_micro": 1
    },
    {
      "rule_title": "Field does not exist and cmd.exe in CMD Line",
      "time_taken_micro": 231
    },
    {
      "rule_title": "Field exists",
      "time_taken_micro": 1
    },
    {
      "rule_title": "Obfuscation - ^",
      "time_taken_micro": 10
    },
    {
      "rule_title": "Potential Defense Evasion Via Raw Disk Access By Uncommon Tools",
      "time_taken_micro": 12
    },
    {
      "rule_title": "Publicly Accessible RDP Service",
      "time_taken_micro": 1
    },
    {
      "rule_title": "Regsvr32 Used",
      "time_taken_micro": 5
    },
    {
      "rule_title": "Scrobj possible exploit",
      "time_taken_micro": 1
    },
    {
      "rule_title": "Space After Filename - macOS",
      "time_taken_micro": 1
    },
    {
      "rule_title": "TEST integer compare",
      "time_taken_micro": 1
    },
    {
      "rule_title": "URL and scrobj in cmd line",
      "time_taken_micro": 18
    },
    {
      "rule_title": "URL in Command Line",
      "time_taken_micro": 5
    }
  ],
  "unmatched_rules": []
}
```