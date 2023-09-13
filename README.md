# Colorizer

## Description

Colorizer is a tool for highlighting words in your terminal. It accepts one or more pattern parameters as input and highlights each match in one of the available colors:

- Black
- Red
- Green
- Yellow
- Blue
- Purple
- Cyan
- White

## Usage

To see the available options, run:

```sh
$ colorizer --help
```

## Installation

You can install Colorizer using the following command:

```sh
$ cargo install colorizer
```

## Examples

### Example 1

Highlight the word "world" in red:

```sh
$ echo "hello world" | colorizer --color red --regex world
```

### Example 2

Highlight words containing the letter "r" in green:

```sh
$ echo "hello world" | colorizer --regex "\w+r\w+" --color green
```

## Configuration

On startup, Colorizer checks for a configuration file in the home directory ~/.config/colorizer/config.yml. It allows you to use profiles defined in this file.

Create a configuration file in this directory and use a profile from it via the -p or --profile parameter. If the configuration file exists and Colorizer is launched without specifying a profile name, the "default" profile will be used. Colorizer does not terminate abnormally if the configuration file is not found or if the specified profile does not exist.

### Usage Example

If the config file exists, you can use the "my_profile" profile as follows:

```sh
$ echo "hello, world!" | colorizer -p my_profile
```

If the config file exists, the "default" profile will be used if no profile is specified:

```sh
echo "hello, world!" | colorizer
```

### Configuration Example

Here's an example of a configuration file in YAML format:

```yml
profiles:
  default:
    red:
      - "hello .*"
      - "foo"
    blue:
      - "bar .*"
  my_profile:
    red:
      - ".*"
```

In this repository, you can find a `config.yml` file that includes a profile named `golang-test` for syntax highlighting in Go tests using the `colorizer` tool.

Enjoy using Colorizer and add some color to your terminal experience!
