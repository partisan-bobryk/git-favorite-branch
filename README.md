# git-favorite-branch

Quickly manage git branches during intensive multi-tasking work environments

### Build Targets

While building the distribution, we leverage GH Actions to build on multiple OS. Here are all of the available build targets. **Replace `BUILD_TARGET` keyword with the target that best suites your operating system**

- `macOS_latest`
- `ubuntu-latest`

## Install

### Brand new install

All releases will be posted on the github repo. To install the cli for the first time run the following command.

```bash
wget -qO- https://raw.githubusercontent.com/VeprUA/git-favorite-branch/main/bin/install.sh | sh -s v0.2.2 BUILD_TARGET
```

You are also more than welcome and download the binary zip from the release page. Just make sure to put it in the `/usr/local/revent-studio/gfb-v0.2.2/` directory. That way an update can be done cleanly.

Here is a helper script once you download your zip:

```bash
# Unzip and give execute permission
unzip gfb-BUILD_TARGET.zip
chmod +x gfb-BUILD_TARGET/gfb

# Create an install directory
sudo mkdir -p /usr/local/revent-studio/gfb-v0.2.2

# Move the binary to the install directory
sudo mv gfb-BUILD_TARGET/gfb /usr/local/revent-studio/gfb-v0.2.2/gfb

# Create a symlink in the bin directroy
sudo ln -s /usr/local/revent-studio/gfb-v0.2.2/gfb /usr/local/bin/gfb

```

Similar directions are used in the `bin/install.sh` file. You can always use that as a reference.

### Already using gfb

It is very simple to update the current binary. You don't need to provide a build target as that is baked in during the build process.

```bash
 gfb install v0.2.2
```

## Uninstall

You can always remove the cli from your machine by running the uninstall command in the `/bin` directory.

```bash
wget -qO- https://raw.githubusercontent.com/VeprUA/git-favorite-branch/main/bin/uninstall.sh | sh
```

## Build

Currenty we don't have a build avaialbe for multiple OS, so you will need to clone the repo and build it your self. No worries here is the guide to get it working on your machine.

**Requirements**

- Rust (latest)

Installation Instructions

1. Clone this repo

```bash
git clone git@github.com:VeprUA/git-favorite-branch.git
```

2. Build

```bash
export BUILD_TARGET="<build_target>"; cargo build
```

3. With the help of cargo make your executable visible to your environment

```bash
cargo install --path .
```

4. Run the executable

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
    add     <SHORTCUT_KEY> [BRANCH_NAME] Add current branch to favorites
    del     <SHORTCUT_KEY>               Delete favorited branch
    use     <SHORTCUT_KEY>               Switch to a different branch
    prnt    <SHORTCUT_KEY>               Print out the favorited branch name
    install <VERSION>                    Install specific version of the binary. Provide version number as a semver. ie v0.2.0
    version                              Display the current binary version
    clr                                  Clear all saved favorites
    ls                                   Display all favorited branches and their keys
    help                                 Print this message or the help of the given subcommand(s)
```
