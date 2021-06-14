# Colorizer  

**it is a program for coloring any word in the input file displayed in your terminal**  

## How to get and compile  
```
git clone https://github.com/kulinsky/colorizer
cd colorizer
cargo build --release
```

## How to use
create a settings.json file and add a "substring" key which is a hash map of "word: color" like this:
```
{
  "substrings": {
    "info": "GREEN",
    "debug": "BLUE",
    "error": "RED"
  }
}
```
example is in the repository  

**now run the application**
```
cat server.log | ./target/release/colorizer settings.json
```