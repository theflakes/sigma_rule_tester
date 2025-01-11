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
- Total: 1
- Successful: 1
- Failed: 0
- Errors: []

Results:
{
  "rule_count": 1,
  "rules": [
    "Example Sigma Rule"
  ]
}
```