# Simple SMTP server in Rust

This is a small educational project for teaching Rust.
It implements a simple, not standards-compliant and very unsafe SMTP server in Rust.
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

## About SMTP

Original SMTP specification: [RFC 821](https://tools.ietf.org/html/rfc821).

Sample conversation:

```text
<- 220 smtp.server.com Simple Mail Transfer Service Ready
-> HELO localhost
<- 250 Hello localhost
-> MAIL FROM:<user@localhost>
<- 250 OK
-> RCPT TO:<admin@localhost>
<- 250 OK
-> DATA
<- 354 Send message content
-> <Mail Data>
-> .
<- 250 OK
-> QUIT
<- 221 Bye
```

## Status

[![CircleCI](https://circleci.com/gh/az82/rust-smtp-server.svg?style=svg&circle-token=e44f845dc05807a8693d2b3674bb023eb94c3996)](https://circleci.com/gh/az82/rust-smtp-server)
