# json-formatter

json-formatter is cli tool for formatting JSON.
This tool supports JSON of RFC 8259 style. (maybe) 

![](./demo.gif)

## install

```bash
git clone https://github.com/hikaru7719/json-formatter.git
cd json-formatter
cargo install --path .
```

## usage

```bash
echo '{"a": "test"}' | json-formatter
```
