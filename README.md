# Colorizer  

**A tool for highlighting words in your terminal**  

## Quickstart
1. Install
```
cargo install colorizer
```
2. There are built-in profiles, currently only one (nginx) :)
```
// via pipe
cat access.log | colorizer -p nginx
```

3. There are built-in regular expressions, you can start using them without config
```
// via pipe
cat server.log | colorizer --email YELLOW --ipv4 RED --isotime CYAN

// or file as arg
colorizer server.log --email YELLOW --ipv4 RED --isotime CYAN
```

4. Create or Download [settings.json](https://github.com/kulinsky/colorizer/blob/master/settings.json)  and create one or more profiles, the profile with the name "default" is used if you do not pass the names of the profiles as arguments  
  
5. You can use multiple profiles
```
cat server.log | colorizer --config settings.json -p prof1 prof2 prof3
```
  
6. tail also work
```
tail -f access.log | colorizer -p nginx
```

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
   available colors: BLACK, RED, GREEN, BLUE, CYAN, YELLOW, PURPLE, WHITE, FORESTGREEN, MAGENTA, ORANGE  
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
