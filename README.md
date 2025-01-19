# sigma_rule_tester

Test any number of rules against a Json log to see which rules fire.  

```
Authors: Brian Kellogg
License: MIT
Purpose: Test Sigma rules against a Json log.

Note: 
    When a rule uses simple glob matching, '*' and '?', the first time this
    logic is encountered it is converted into a regex and cached for quicker
    execution of that logic on subsequent runs.

Usage: 
    sigma_rule_tester --log './log.json' --rules './rules'

Options:
    -e, --errors            Print out all Sigma rule load errors
    -l, --log <location>    Test Json log
    -n, --number            Number of time to run the log through all rules
    -p, --pretty            Pretty print output
    -r, --rules <location>  Path to the directory containing your Sigma rules
                            - rules in sub directories will be used as well
    -u, --unmatched         Include all unmatched rules in the output
```
Output
```
# .\sigma_rule_tester -r .\config\sigma_rules\ -l .\log.json -n 3 --pretty
Rule Load Summary:
- Total: 2989
- Successful: 2989
- Failed: 0
- Load time: 209.8988ms
{
  "run_number": 1,
  "time_taken_milli": 0,
  "time_taken_micro": 855,
  "matched_rule_count": 17,
  "matched_rules": [
    {
      "rule_title": "ALERT - ADS redirection used",
      "time_taken_micro": 58
    },
    {
      "rule_title": "ALERT - ADS redirection used and CMD line exists",
      "time_taken_micro": 39
    },
    {
      "rule_title": "ALERT - Sus Parent to Child commandline",
      "time_taken_micro": 153
    },
    {
      "rule_title": "Example Sigma Rule",
      "time_taken_micro": 356
    },
    {
      "rule_title": "Execution Of Non-Existing File",
      "time_taken_micro": 4
    },
    {
      "rule_title": "Failed Authentications From Countries You Do Not Operate Out Of",
      "time_taken_micro": 1
    },
    {
      "rule_title": "Field does not exist and cmd.exe in CMD Line",
      "time_taken_micro": 178
    },
    {
      "rule_title": "Field exists",
      "time_taken_micro": 0
    },
    {
      "rule_title": "Obfuscation - ^",
      "time_taken_micro": 9
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
      "time_taken_micro": 9
    },
    {
      "rule_title": "Scrobj possible exploit",
      "time_taken_micro": 1
    },
    {
      "rule_title": "Space After Filename - macOS",
      "time_taken_micro": 2
    },
    {
      "rule_title": "TEST integer compare",
      "time_taken_micro": 2
    },
    {
      "rule_title": "URL and scrobj in cmd line",
      "time_taken_micro": 19
    },
    {
      "rule_title": "URL in Command Line",
      "time_taken_micro": 5
    }
  ]
}
{
  "run_number": 2,
  "time_taken_milli": 0,
  "time_taken_micro": 238,
  "matched_rule_count": 17,
  "matched_rules": [
    {
      "rule_title": "ALERT - ADS redirection used",
      "time_taken_micro": 39
    },
    {
      "rule_title": "ALERT - ADS redirection used and CMD line exists",
      "time_taken_micro": 32
    },
    {
      "rule_title": "ALERT - Sus Parent to Child commandline",
      "time_taken_micro": 80
    },
    {
      "rule_title": "Example Sigma Rule",
      "time_taken_micro": 28
    },
    {
      "rule_title": "Execution Of Non-Existing File",
      "time_taken_micro": 3
    },
    {
      "rule_title": "Failed Authentications From Countries You Do Not Operate Out Of",
      "time_taken_micro": 2
    },
    {
      "rule_title": "Field does not exist and cmd.exe in CMD Line",
      "time_taken_micro": 6
    },
    {
      "rule_title": "Field exists",
      "time_taken_micro": 1
    },
    {
      "rule_title": "Obfuscation - ^",
      "time_taken_micro": 2
    },
    {
      "rule_title": "Potential Defense Evasion Via Raw Disk Access By Uncommon Tools",
      "time_taken_micro": 8
    },
    {
      "rule_title": "Publicly Accessible RDP Service",
      "time_taken_micro": 0
    },
    {
      "rule_title": "Regsvr32 Used",
      "time_taken_micro": 8
    },
    {
      "rule_title": "Scrobj possible exploit",
      "time_taken_micro": 2
    },
    {
      "rule_title": "Space After Filename - macOS",
      "time_taken_micro": 1
    },
    {
      "rule_title": "TEST integer compare",
      "time_taken_micro": 4
    },
    {
      "rule_title": "URL and scrobj in cmd line",
      "time_taken_micro": 10
    },
    {
      "rule_title": "URL in Command Line",
      "time_taken_micro": 5
    }
  ]
}
{
  "run_number": 3,
  "time_taken_milli": 0,
  "time_taken_micro": 193,
  "matched_rule_count": 17,
  "matched_rules": [
    {
      "rule_title": "ALERT - ADS redirection used",
      "time_taken_micro": 46
    },
    {
      "rule_title": "ALERT - ADS redirection used and CMD line exists",
      "time_taken_micro": 13
    },
    {
      "rule_title": "ALERT - Sus Parent to Child commandline",
      "time_taken_micro": 90
    },
    {
      "rule_title": "Example Sigma Rule",
      "time_taken_micro": 4
    },
    {
      "rule_title": "Execution Of Non-Existing File",
      "time_taken_micro": 2
    },
    {
      "rule_title": "Failed Authentications From Countries You Do Not Operate Out Of",
      "time_taken_micro": 1
    },
    {
      "rule_title": "Field does not exist and cmd.exe in CMD Line",
      "time_taken_micro": 1
    },
    {
      "rule_title": "Field exists",
      "time_taken_micro": 0
    },
    {
      "rule_title": "Obfuscation - ^",
      "time_taken_micro": 1
    },
    {
      "rule_title": "Potential Defense Evasion Via Raw Disk Access By Uncommon Tools",
      "time_taken_micro": 5
    },
    {
      "rule_title": "Publicly Accessible RDP Service",
      "time_taken_micro": 0
    },
    {
      "rule_title": "Regsvr32 Used",
      "time_taken_micro": 13
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
      "time_taken_micro": 4
    },
    {
      "rule_title": "URL in Command Line",
      "time_taken_micro": 2
    }
  ]
}
```