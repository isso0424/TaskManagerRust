# TaskManagerRust
Task managing tool develop with rust.  
## Commands
### Create Task
```bash
tmr create task <task title>
```

|option|explanation|format|
|---|---|---|
|--limit|task limit|--limit YYYY-MM-DD|
|--label|task labels|--label LABEL1 LABEL2|

### Create Label
```bash
tmr create label <label title>
```

### Check Task
```bash
tmr check task
```

|option|explanation|format|
|---|---|---|
|-k|search keyword|-s KEYWORD|
|-l|search label|-l LABEL|

### Check Label
```bash
tmr check label
```

|option|explanation|format|
|---|---|---|
|-k|search keyword|-s KEYWORD|
|-l|search label|-l LABEL|

### Mark Task Done
```bash
tmr done <task title>
```

### Update Task
```bash
tmr update task <task title>
```

|option|explanation|format|
|---|---|---|
|--title|task new title|-t TITLE|
|--labels|task new labels(overwrite)|--labels LABEL1 LABEL2|
|--limit|task new limit(overwrite)|--limit YYYY-MM-DD|

### Update Label
```bash
tmr update label <label title>
```

### Delete Task
```bash
tmr delete task <task title>
```

### Delete Label
```bash
tmr delete label <label title>
```

## Tips
- Cannot create tasks and labels with duplication.
- Cannot notify when expire.
