# git-favorite-branch

Quickly manage git branches during intensive multi-tasking work environments

## Build

Currenty we don't have a build avaialbe for multiple OS, so you will need to clone the repo and build it your self. No worries here is the guide to get it working on your machine.

**Requirements**

- Rust (latest)

Installation Instructions

1. Clone this repo

```bash
git clone git@github.com:VeprUA/git-favorite-branch.git
```

2. With the help of cargo make your executable visible to your environment

```bash
cargo install --path .
```

3. Run the executable

```bash
gfb
```

## Usage

You can always run `gfb <SUBCOMMAND> -h` to get the latest usage.

```
gfb <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    add  <SHORTCUT_KEY> [BRANCH_NAME] Add current branch to favorites
    del  <SHORTCUT_KEY>               Delete favorited branch
    use  <SHORTCUT_KEY>               Switch to a different branch
    clr                               Clear all saved favorites
    ls                                Display all favorited branches and their keys
    help                              Print this message or the help of the given subcommand(s)
```
