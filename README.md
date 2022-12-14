# todoist_helper
A simple CLI for adding, updating and viewing TODOs in Todoist

## Prerequisites
- Set an env variable for your Todoist API token (go to settings and integrations)
  - `export TODOIST_TOKEN=1234567890`

## Usage
To build, run `cargo build --release`. The executable will live in `./target/release/todoist_helper` - you can move this to somewhere your $PATH includes to run from anywhere
- `todoist_helper add todo`
- `todoist_helper show todos`
- `todoist_helper complete todo 12345`

## Options
- When adding a TODO, you can give the content (`-c`) flag to provide content
  - `todoist_helper add todo -c "My new TODO"`
- When showing TODOs, you can add filters with the filters (`-f`) flag, and multiple separated by commas. See [Todoist's filter documentation](https://todoist.com/help/articles/introduction-to-filters) *
  - `todoist_helper show todos -f "today"`
- When showing TODOs, you can specify the columns you'd like to see as a result with the attribute(s) (`-a`) flag
  - `todoist_helper show todos -a "id,content"`

## Advanced Usage
If you have [fzf](https://github.com/junegunn/fzf) installed, you can use the following to complete a TODO
- `todoist_helper show todos -a "id,content" | fzf | xargs -I{} todoist_helper complete todo {}`

## TODO: (I see the irony in this)
- * multiple filters doesn't seem to work through the API i.e. (today | overdue). Need to investigate this

## Contributions
You're more than welcome to submit an issue or PR with any bugs or feature suggestions.

