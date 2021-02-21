# zlib

command line tool for encode/decode zlib stream from stdin, only output result to stdout

compress input raw stream

```
echo "hello rust" | zlib > output
```

decompress input zlib stream

```
zlib -d < output
```