# Simple SMTP server in Rust

This is a small educational project for teaching Rust.
It implements a simple, not entirely standards-compliant and very unsafe SMTP server in Rust.
The server prints received messages on stdout. 

This project is licensed under the terms of the [MIT license](LICENSE).

## Build

```bash
cargo build
```

## Usage

Starting the server:
```bash
./target/debug/rust-smtp-server
```

Sending requests using netcat:
```bash
nc localhost 2525 <<EOT
HELO localhost
MAIL FROM: someone@localhost
RCPT TO: someone.else@localhost
DATA
Hello,
the SMTP server works!
Bye.
.
QUIT
EOT
EOT
```


## Status

[![Build Status](https://travis-ci.org/az82/rust-levenshtein.svg?branch=master)](https://travis-ci.org/az82/rust-levenshtein)
