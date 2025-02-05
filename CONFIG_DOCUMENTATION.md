# Configuration Guide
The `config.json` allows you to customize the behavior of the programs formatter. It is possible to design rules for highlghting, deleting and changing either patterns or words. The following documentation will explain how the configuration and the syntax works.

# Structure
The `config.json` file consists of three main sections:
1. Highlight Rules
3. Deletion Rules
4. Change Rules

### 1. Highlight Rules
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
| Parameter   | Description                                                        |
|-------------|--------------------------------------------------------------------|
| **type**    | Specifies whether the pattern is an exact match (`"exact"`) or a regular expression (`"regex"`). |
| **pattern** | The word or regex pattern to highlight, delete, or change.         |
| **style**   | The CSS style to apply for highlighting the matched pattern.       |



### 2. Deletion Rules
Deletion rules specify which words or patterns should be removed from the log messages.

**Example:**
```json
"deletion_rules": [
    {
        "type": "exact",
        "pattern": "DEBUG"
    },
    {
        // removes patterns like <Resolve_comment> = <>
        "type": "regex",
        "pattern": "<\\w+> = <>"
    }
]
```

| Parameter   | Description                                                        |
|-------------|--------------------------------------------------------------------|
| **type**    | Specifies whether the pattern is an exact match (`"exact"`) or a regular expression (`"regex"`). |
| **pattern** | The word or regex pattern to highlight, delete, or change.         |


### 3. Change Rules
Change rules specify which words or patterns should be replaced with a different word or pattern.

**Example:**
```json
"change_rules": [
    {
        "type": "exact",
        "pattern": "old_word",
        "replacement": "new_word"
    },
    {
        // replaces occurrences of 'foo' with 'bar'
        "type": "regex",
        "pattern": "\\bfoo\\b",
        "replacement": "bar"
    }
]
```
| Parameter   | Description                                                        |
|-------------|--------------------------------------------------------------------|
| **type**    | Specifies whether the pattern is an exact match (`"exact"`) or a regular expression (`"regex"`). |
| **pattern** | The word or regex pattern to highlight, delete, or change.         |
| **replacement** | The word or string to replace the matched pattern (for change rules). |

