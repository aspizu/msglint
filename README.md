# msglint

Checks your commit messages.

![](./docs/assets/screenshot.png)

## Installation

### Install from source

```shell
git clone https://github.com/aspizu/msglint
cd msglint
cargo install --path .
```

### Instal from source (using cargo)

```shell
cargo install --git https://github.com/aspizu/msglint
```

## Usage

### Install the git hook into your repository

> [!NOTE] 
> You will have to do this for every repository.

```shell
msglint --install
```

### Test a commit message from a file

```shell
msglint commit-message.txt
```

### Uninstall the git hook from a repository

```shell
rm .git/hooks/commit-msg
```

## Integration

### `pre-commit`

It's recommended to not use `pre-commit` to handle `msglint`. If you don't already have
`pre-commit` hooks that run on `commit-msg`, don't install `pre-commit` into `commit-msg`.
Just install `msglint` directly into `commit-msg`.

First, specify the `commit-msg` hook to be installed by default when you
do `pre-commit install` in `.pre-commit-config.yaml`.

```yaml
default_install_hook_types: [pre-commit, commit-msg]
```

Install `pre-commit` into the `commit-msg` hook.

```shell
pre-commit install --hook-type commit-msg
```

Add the `msglint` hook to `.pre-commit-config.yaml`.

```yaml
repos:
    - repo: local
      hooks:
          - id: msglint
            name: msglint
            language: system
            entry: msglint
            stages: [commit-msg]
```
