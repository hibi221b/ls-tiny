# ls-tiny
ls-tiny is a less functional ls command

# How to install

`$ cargo install ls-tiny`

# Demo

![lstiny](https://user-images.githubusercontent.com/29950288/80045180-9a850a00-8541-11ea-8933-860307eed0ba.gif)

# Screenshot

<img width="647" alt="スクリーンショット 2020-04-22 18 52 01" src="https://user-images.githubusercontent.com/29950288/79967910-68849100-84ca-11ea-8e03-c2a703a62cb1.png">

# Usage

```console
ls-tiny VERSION
hibi221b
ls-tiny is a less functional ls command

USAGE:
    ls-tiny <PATH>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <PATH>    Sets the directory
```

# Run

`$ ls-tiny DIRECTORY`

# Update
- It can run against all directories. (`ls-tiny ../../../`) - v0.1.5

# Error
- ~~Entering a relative path results in an error --> v0.1.0~~ (modified v0.1.1)
- ~~An error occurs when a parent directory is specified -> v0.1.3~~ (modified v0.1.4)

