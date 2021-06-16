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
  "default": {
    "substrings": {
      "info": "GREEN",
      "debug": "BLUE",
      "error": "RED"
    }
  },
  "profile1": {
    "substrings": {
      "null": "CYAN",
      "time": "YELLOW",
      "message": "PURPLE"
    }
  }
}
```
example is in the repository  

**now run the application**
```
cat server.log | ./target/release/colorizer --config settings.json --profile profile1
```