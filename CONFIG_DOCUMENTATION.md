# Configuration Guide
The `config.json` allows you to customize the behavior of the programs formatter. It is possible to design rules for highlghting, deleting and changing either patterns or words. The following documentation will explain how the configuration works.

# Structure
The `config.json` file consists of three main sections:
1. Highlight Rules
3. Deletion Rules
4. Change Rules

### 1. Highligt Rules
Highlight rules specify which words or patterns should be highlighted in the log messages and the style to apply.
```json
"highlight_rules": [
    {
        "type": "exact",
        "pattern": "error",
        "style": "color:red;font-weight:bold;"
    },
    {
        // highlights ip addresses
        "type": "regex",
        "pattern": "\\b(?:\\d{1,3}\\.){3}\\d{1,3}\\b",
        "style": "color:blue;font-weight:bold;"
    }
]
```
The highlight_rules section can hold the following parameters:
- type
- pattern
- style

**type:** This can be either an exact match (a string) or a regex match (a regex string):
```
"type": "exact"
"type": "regex"
```

**pattern:** This can be a regex or a string, this is defined by the type parameter.

**style:** The CSS style to apply for highlighting the matched pattern. For example:




### 2. Deletion Rules

### 3. Change Rules
