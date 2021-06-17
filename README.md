# Colorizer  

**it is a program for coloring any word in the input file displayed in your terminal**  

https://user-images.githubusercontent.com/794932/122374294-eaefd000-cf6a-11eb-8794-5f520c0e91b8.mov

## Install
```cargo install colorizer```

## How to get and compile  
```
git clone https://github.com/kulinsky/colorizer
cd colorizer
cargo build --release
```

## How to use
1. create a settings file and create one or more profiles  
```
{
  "default": {},
  "profile1": {}
}
```
2. profile can have key "substrings", which is a hashmap with a key: the search word,  and a value: target color.
3. profile can have key "regex", which is a hashmap with a key: the search regex,  and a value: target color.
```
{
  "default": {
    "substrings": {
      "info": "GREEN",
      "debug": "BLUE",
      "error": "RED"
    },
    "regex": {
      "(\\d{4})-(\\d{2})-(\\d{2})": "PURPLE"
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
cat server.log | colorizer --config settings.json --profile profile1
```
