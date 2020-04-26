# ls-tiny
ls-tiny is a less functional ls command

# How to install

`$ cargo install ls-tiny`

# Environment
mac (ls-tiny can **NOT** display the color on windows os.)

# Demo

![movie](https://user-images.githubusercontent.com/29950288/80307787-4b233000-8806-11ea-8529-58f78dd62ea0.gif)

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
- ls-tiny can run against all directories. (`ls-tiny ../../../`) - v0.1.5

# Error
- ~~Entering a relative path results in an error --> v0.1.0~~ (modified v0.1.1)
- ~~An error occurs when a parent directory is specified -> v0.1.3~~ (modified v0.1.4)

