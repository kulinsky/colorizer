# Colorizer

**A tool for highlighting words in your terminal**

Colorizer accepts one or more pattern parameters as input and highlights each match in one of the available colors red, blue or green, there are also built-in regular expressions for email, isotime and ipv4 address.

```bash
colorizer --help
```

## Quickstart

1. Install

```bash
cargo install colorizer
```

2. There are built-in regex for email, isotime, ipv4

```bash
// via pipe
cat access.log | colorizer --email --ipv4 --isotime
```

3. The program has three highlighting colors, these are red, blue and green

```bash
echo "hello world" | colorizer --color red --pattern world

echo "hello world" | ./target/release/colorizer --pattern "\w+r\w+" --color green
```

4. tail also work

```bash
tail -f access.log | colorizer --pattern ERROR
```
