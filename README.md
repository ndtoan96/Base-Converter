# Description
A small program written in rust to change between hex, dec, bin with nice format for binary display. Mainly aim for embedded developer.

# Usage:
A command always start with ":". Below is the list of command:
```
    :from <base> to <base>      change input base and output base
    :from <base>                change input base
    :to <base>                  change output base
<base> can be "hex", "dec", "bin"
    :h or :help                 print help message
    :q or :quit                 stop program
```