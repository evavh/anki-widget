# Anki text widget

A Linux widget that shows you how many Anki cards you currently have to study.
It can be used in a text-based bar, like i3bar or waybar, or with a GNOME
extension that runs a command-line program and displays the output in the status
bar (like [this one](https://extensions.gnome.org/extension/2932/executor/)).

## Example output

Default format: `Anki - due: 12, new: 20`

Short format: `12 / 20`

## Installation

### Download binary

Download the
[latest release](https://github.com/evavh/anki-widget/releases/latest) and place
it in your path (in $HOME/bin or $HOME/.local/bin, for example).

### Build using build script

To compile the widget, we need to build the Anki Rust code, which requires
`protoc v3.15` or later. Run the script `build.sh` to download a recent version
of `protoc`, build the widget, and clean up afterwards.

### Build manually

On Ubuntu 24.10 and later, you can install protoc using
`apt install protobuf-compiler`. On other distro's, you can check your package
manager for its `protoc` version.

On earlier versions of Ubuntu, you can manually install `protoc` by downloading
a zip file [here](https://github.com/protocolbuffers/protobuf/releases/latest)
(you probably want `protoc-<version>-linux-x86_64.zip`).

Unzip it, and place the file `bin/protoc` in your path.

Once you have installed `protoc`, you can run
`cargo install --git https://github.com/evavh/anki-widget` to compile and
install `anki-widget`.

## Setup

To try it out, run `anki-widget one-shot` in the terminal, and follow any
instructions for configuration. No configuration should be needed if you only
have one Anki install with one user profile.

Once you have working output, you can follow the instructions for your bar on
how to add the widget to it. Be sure to check whether your bar runs the command
every time it updates (use the `one-shot` command) or expects continuous input
(use the `continuous` command).

## Usage

```
A widget that shows Anki's current due and new card counts

Usage: anki-widget [OPTIONS] <COMMAND>

Commands:
  one-shot    Print output once and then quit, used for GNOME and text
              bars that do the refreshing for you by running the command
              again
  continuous  Print output every minute (by default), used for text bars
              that only run the command once and expect output to change.
              Settings: --refresh-delay, --retry-delay
  help        Print this message or the help of the given subcommand(s)

Options:
  -s, --short                   Print only the card counts, in the form <due> / <new>
  -j, --json                    Print output as machine-readable json of the form {"msg": "<output>"}
  -p, --path <PATH>             The full path to your Anki2 folder, by default the
                                widget will search for this. Use this if you have
                                a custom path, or multiple paths were found
  -u, --user-profile <PROFILE>  The user profile to use. Use this if multiple
                                profiles were found
  -h, --help                    Print help
  -V, --version                 Print version
```

## Possible future features

- [x] Provide binaries
- [ ] More formatting options (suggestions welcome)
- [ ] Windows support?

If you find any bugs or would like to suggest a feature or improvement, feel
free to create an issue.
