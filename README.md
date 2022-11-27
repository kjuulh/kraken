<p align="center">
  <image src="https://git.front.kjuulh.io/kjuulh/octopush/raw/branch/v0.2/assets/octopush.svg" width="300" height="300"/>
</p>
<h1 align="center">Octopush - Your cute action executor</h1>

## Purpose

The goal of this project is to easily do batch changes or queries on a host of
repositories. In large organisations using multi-repository strategies, it may
be painful to change even small things across many repositories, because there
are so many of them. Octopush aims to change that.

**DISCLAIMER:** It is still early days, and the api is subject to change.

## Features

- [x] Uses an actions repository, where you store all your pending commands or
      queries to be performed across your fleet of repositories. (See
      \_examples)
- [x] Actions can both execute changes, open pull-requests or in some cases
      commit directly to your preferred branch
  - [x] Actions natively use either shell, go or docker files to execute changes
        (see \_examples/actions)
- [ ] Actions can also be analytical, so you can query your fleet for whatever
      you would like
- Supports SSH/https for fetching repos
- [ ] Supports GPG signing
- [ ] Supports dry-run mode for easy testing when developing your actions
      (enabled by default on the cli)

## Roadmap

Refer to [roadmap.md](roadmap.md)

## Installation

Octopush runs on your client and acts on your behalf, unless of course it is
setup on a remote server

### Client (CLI)

Download executable from [releases](https://github.com/kjuulh/octopush/releases)

#### Or Use docker image

```bash
docker run --rm kasperhermansen/octopushcli:latest version
```

#### Or Build from source

```bash
git clone https://github.com/kjuulh/octopush.git
cd octopush

cargo build --release --target=x64_86-unknown-linux_musl
./target/x64_86-unknown-linux_musl/octopush version
```

#### Or Build with cuddle

```bash
git clone https://github.com/kjuulh/octopush.git
cd octopush

cuddle_cli x build_cli
```

## Usage

**DISCLAIMER:** It is still early days, and the api of the CLI is subject to
change, this provides the aim of the project, but as it is currently in flux,
there may not be as much handholding in the actual usage.

### Creating a new action

Creating a new action

```bash
git init my-actions # should only be done once
cd my-actions
octopush tmpl init write-a-readme --action
cat write-a-readme/octopush.yml

# Output
# apiVersion: action
# name: write-a-readme
# select:
#   git:
#     repositories: []
# actions:
#   - type: shell
#     entry: "main.sh"
```

Octopush also ships with yaml schema, which should help write the yaml
configuration.

#### Add upstream repositories (victims)

Now add a preferred repository

```
cat << EOF > write-a-readme/octopush.yml
apiVersion: git.front.kjuulh.io/kjuulh/octopush/blob/main/schema/v1
name: write-a-readme
select:
  gitea:                  # new
    repositories:         # new
      "kjuulh/octopush"   # new
actions:
  - type: shell
    entry: "main.sh"
EOF
```

This will take all your repositories under an organisation and run the script
on.

Another could be to use

```bash
cat << EOF > write-a-readme/octopush.yml
apiVersion: git.front.kjuulh.io/kjuulh/octopush/blob/main/schema/v1
name: write-a-readme
select:
  repositories:                                       #new
    - git@git.front.kjuulh.io:kjuulh/octopush.git       #new
    - git@git.front.kjuulh.io:kjuulh/octopush-test.git  #new
actions:
  - type: shell
    entry: "main.sh"
EOF
```

This will just apply to those repositories instead. Both can also be combined
for a shared effect.

### Execute action

To run the script use

```bash
octopush process --path "write-a-readme"
```

This will cause the octopush process to automatically apply the action on the
repo and open a pr.

### Query repositories

Octopush can also be used to query.

```bash
cat << EOF > write-a-readme/octopush.yml
apiVersion: git.front.kjuulh.io/kjuulh/octopush/blob/main/schema/v1
name: write-a-readme
select:
  repositories:
    - git@git.front.kjuulh.io:kjuulh/octopush.git
    - git@git.front.kjuulh.io:kjuulh/octopush-test.git
queries:
  - type: grep
    query: "# README"
EOF
```

Using the same command as above, will return the lines on each repo with those
criteria. Everything is run in docker, even locally, so no need to install fancy
tools.

Do note: All actions will be run as dry-run unless `--apply` is added. This is
to help test locally, as well as not cause serious issues. The server
configuration is pretty much the same, except the command would look like so:
`octopush server process --path "write-a-readme" --apply`. Octopush will try to
infer as much as possible, but it may be needed to apply some extra flags to
specify upstream repositories and such. Octopush will also help you setup keys
and such on the first run, using `octopush setup` or `octopush server setup`.

## Contributing

It is still early days, and as such things are moving fast, I may not be able to
implement features, because I am focusing my energy on the API. That said PRs
are welcome, though they are at your own risk.

### Bugs & features requests

Please use [issues](https://github.com/kjuulh/octopush/issues)

### Development

We use [cuddle](https://git.front.kjuulh.io/kjuulh/cuddle) to improve ease of
use, it is however, not a requirement, and probably won't need to be used
outside core maintainers.

Simply:

```bash
cargo build
```
