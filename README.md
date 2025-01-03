# Anki text widget

A Linux widget that you can use in a text-based bar, like i3bar or waybar, that shows you how many Anki cards you currently have to study.

Example output:
`Anki - new: 20, due: 12`

## Usage

To try it out, run it on the command line using `cargo run`, and follow any instructions for configuration. No configuration is needed if you only have one Anki install with one user profile.

```

A widget that shows Anki's current due and new card counts

Usage: anki-widget [OPTIONS]

Options:
  -r, --refresh-delay <MINUTES>  Minutes between checking the database for new card
                                 counts. [default: 1]
  -t, --retry-delay <SECONDS>    Seconds between retries when the database is in use,
                                 or some other error occurs. [default: 10]
  -p, --path <PATH>              The full path to your Anki2 folder, by default the
                                 widget will search for this. Use this if you have
                                 a custom path, or multiple paths were found.
  -u, --user-profile <PROFILE>   The user profile to use. Use this if multiple
                                 profiles were found.
  -h, --help                     Print help
  -V, --version                  Print version
```

## Possible future features

- [ ] Provide binaries
- [ ] GNOME support
- [ ] More formatting options (suggestions welcome)
- [ ] Windows support?

If you find any bugs or would like to suggest a feature or improvement, feel free to create an issue.
