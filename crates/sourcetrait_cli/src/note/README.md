SourceTrait CLI: Note
================================================================================
*A very, very simple Markdown notes manager*

This command-line tool manages note files in your preferred directory, organized
under one of the following four categories:
- Today
- Idea
- Todo
- Plan

The *Today* category is sharded by year and month. The *Plan* category is either
singular or topical. The rest of the categories are named with their topic.

When running the *today* subcommand, you can optionally carry over your daily
list from the previous day.


Usage
--------------------------------------------------------------------------------
For help, see `srctrait note --help`.

`srctrait note <command>`
- `today` Edits today's note
  - `today from <date>` Carries over notes from the previous date into today's
- `yesterday` Edits yesterday's note
- `day` Edits the given day's note
- `idea <topic>` Edits an idea note for that topic
- `plan` Edits your master plan
- `plan <topic>` Edits topical plan
- `todo <topic>` Edits a topical TODO list
- `config` Edits the command's config file. Configures the notes dir and editor command.
- `pick <note type>` Uses [yazi](https://github.com/sxyazi/yazi) to choose a note to edit


System Setup
--------------------------------------------------------------------------------
It's recommended to make an alias to something like `note` for ease of use.

In Fish, this would be something like:
```bash
alias note "srctrait note"
funcsave note
```
