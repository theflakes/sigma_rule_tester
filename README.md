# sigma_rule_tester

Test any number of rules against a Json log to see which rules fire.  

```
Authors: Brian Kellogg
License: MIT
Purpose: Test Sigma rules against a Json log.

Usage: 
    sigma_rule_tester --log './log.json' --rules './rules'

Options:
    -l, --log <location>    Path to the test log file
    -r, --rules             Path to the directory containing your Sigma rules
                            - rules in sub directories will be used as well
```