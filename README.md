# sigma_rule_tester

Test any number of rules against a Json log to see which rules fire.  

```
Authors: Brian Kellogg
License: MIT
Purpose: Test Sigma rules against a Json log.

Usage: 
    sigma_rule_tester --log './log.json' --rules './rules'

Options:
    -l, --log <location>    Test Json log
    -r, --rules <location>  Path to the directory containing your Sigma rules
                            - rules in sub directories will be used as well
```
Output
```
# ./sigma_rule_tester -r "./rules/" -l "./log.json"
Rule Load Summary:
- Total: 2989
- Successful: 2989
- Failed: 0
- Errors: []

Results:
{
  "rule_count": 12,
  "rules": [
    "ALERT - ADS redirection used",
    "ALERT - Sus Parent to Child commandline",
    "Example Sigma Rule",
    "Execution Of Non-Existing File",
    "Field does not exist",
    "Field exists",
    "Obfuscation - ^",
    "Potential Defense Evasion Via Raw Disk Access By Uncommon Tools",
    "Publicly Accessible RDP Service",
    "Regsvr32 Used",
    "Scrobj possible exploit",
    "URL in Command Line"
  ]
}
```