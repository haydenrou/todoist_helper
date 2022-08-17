# todoist_helper
A simple CLI for adding, updating and viewing TODOs in Todoist

## Usage
- `cargo run -- add todo` or with the optional `-c "My new content"` flag
- `cargo run -- show today`
- `cargo run -- show overdue`

## TODO: (I see the irony in this)
- Fix issue where you cannot return two different types from a `match`
- Add GET for todos and projects
- Add a way to set a date to a TODO that you're adding (see due_string)
- Add a display for TODOs (Bonus: display for "Today")
- Complete a task
- Stretch goal: run `cargo run -- complete todo` and automatically spit out a list of tasks to FZF that you can complete, then complete one when selected
