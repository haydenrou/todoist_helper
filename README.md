# todoist_helper
A simple CLI for adding, updating and viewing TODOs in Todoist

## Usage
- `cargo run -- add todo` or with the optional `-c "My new content"` flag
- `cargo run -- show today`
- `cargo run -- show overdue`

## Options
When adding a TODO, you can give the content (`-c`) flag to provide content
- `cargo run -- add todo -c "My new TODO"`
When showing TODOs, you can add filters with the filters (`-f`) flag, and multiple separated by commas. See [Todoist's filter documentation](https://todoist.com/help/articles/introduction-to-filters)
- `cargo run -- show todos -f "today"`
- `cargo run -- show todos -f "today | overdue"`
When showing TODOs, you can specify the columns you'd like to see as a result with the attribute(s) (`-a`) flag
- `cargo run -- show todos -a "id,content"`

## TODO: (I see the irony in this)
- Add GET for projects
- Add a way to set a date to a TODO that you're adding (see due_string)
- Complete a task
- Stretch goal: run `cargo run -- complete todo` and automatically spit out a list of tasks to FZF that you can complete, then complete one when selected
