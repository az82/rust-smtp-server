use std::io::prelude::*;
use std::io::{BufReader, Error};
use std::net::TcpStream;


const HELO_START: &str = "HELO ";
const MAIL_START: &str = "MAIL FROM:";
const RCPT_START: &str = "RCPT TO:";
const DATA_LINE: &str = "DATA";
const QUIT_LINE: &str = "QUIT";

const MSG_READY: &str = "220 ready";
const MSG_OK: &str = "250 OK";
const MSG_SEND_MESSAGE_CONTENT: &str = "354 Send message content";
const MSG_BYE: &str = "221 Bye";
const MSG_SYNTAX_ERROR: &str = "500 unexpected line";


//S: 220 smtp.server.com Simple Mail Transfer Service Ready
//C: HELO client.example.com
//S: 250 Hello client.example.com
//C: MAIL FROM:<mail@samlogic.com>
//S: 250 OK
//C: RCPT TO:<john@mail.com>
//S: 250 OK
//C: DATA
//S: 354 Send message content; end with <CRLF>.<CRLF>
//C: <The message data (body text, subject, e-mail header, attachments etc) is sent>
//C: .
//S: 250 OK, message accepted for delivery: queued as 12345
//C: QUIT
//S: 221 Bye

enum State {
    Helo,
    Mail,
    Rcpt,
    RcptOrData,
    Dot,
    MailOrQuit,
    Done
}


pub struct Message {
    sender: String,
    recipients: Vec<String>,
    data: Vec<String>
}


pub struct ConnectionResult {
    state: State,
    sender_domain: String,
    messages: Vec<Message>,
    next_sender: String,
    next_recipients: Vec<String>,
    next_data: Vec<String>
}


impl Message {
    pub fn get_sender(&self) -> &str {
        &self.sender
    }

    pub fn get_recipients(&self) -> &Vec<String> {
        &self.recipients
    }

    pub fn get_data(&self) -> String {
        self.data.join("\n")
    }

}


impl ConnectionResult {
    pub fn new() -> ConnectionResult {
        ConnectionResult {
            state: State::Helo,
            sender_domain: "".to_string(),
            messages: Vec::new(),
            next_sender: "".to_string(),
            next_recipients: Vec::new(),
            next_data: Vec::new(),
        }
    }

    fn get_if_done<R, F: FnOnce() -> R>(&self, getter: F) -> Option<R> {
        match self.state {
            State::Done => Some(getter()),
            _ => None
        }
    }

    pub fn get_messages(&self) -> Option<&Vec<Message>> {
        self.get_if_done(|| {&self.messages})
    }

    pub fn get_sender_domain(&self) -> Option<&str> {
        self.get_if_done(|| {self.sender_domain.as_str()})
    }

    pub fn feed_line<'a>(&mut self, line: &'a str) -> Result<&'a str, &'a str> {
        match self.state {
            State::Helo => {
                if line.starts_with(HELO_START) {
                    self.sender_domain = line[HELO_START.len()..].trim().to_string();
                    self.state = State::Mail;
                    Ok(MSG_OK)
                } else {
                    Err(MSG_SYNTAX_ERROR)
                }
            },
            State::Mail => {
                if line.starts_with(MAIL_START) {
                    self.next_sender = line[MAIL_START.len()..].trim().to_string();
                    self.state = State::Rcpt;
                    Ok(MSG_OK)
                } else {
                    Err(MSG_SYNTAX_ERROR)
                }
            },
            State::Rcpt => {
                if line.starts_with(RCPT_START) {
                    self.next_recipients.push(line[RCPT_START.len()..].trim().to_string());
                    self.state = State::RcptOrData;
                    Ok(MSG_OK)
                } else {
                    Err(MSG_SYNTAX_ERROR)
                }
            },
            State::RcptOrData => {
                if line.starts_with(RCPT_START) {
                    self.next_recipients.push(line[RCPT_START.len()..].trim().to_string());
                    Ok(MSG_OK)
                } else if line == DATA_LINE {
                    self.state = State::Dot;
                    Ok(MSG_SEND_MESSAGE_CONTENT)
                } else {
                    Err(MSG_SYNTAX_ERROR)
                }
            },
            State::Dot => {
                if line == "." {
                    self.messages.push(Message {
                        sender: self.next_sender.clone(),
                        recipients: self.next_recipients.clone(),
                        data: self.next_data.clone()
                    });
                    self.state = State::MailOrQuit;
                    Ok(MSG_OK)
                } else {
                    self.next_data.push(line.to_string());
                    Ok("")
                }
            },
            State::MailOrQuit => {
                if line.starts_with(MAIL_START) {
                    self.next_sender = line[MAIL_START.len()..].trim().to_string();
                    self.state = State::Rcpt;
                    Ok(MSG_OK)
                } else if line == QUIT_LINE {
                    self.state = State::Done;
                    Ok(MSG_BYE)
                } else {
                    Err(MSG_SYNTAX_ERROR)
                }
            },
            State::Done => {
                Err(MSG_SYNTAX_ERROR)
            }
        }
    }
}


pub fn handle_connection(mut stream: TcpStream) -> Result<ConnectionResult, Error> {
    let mut result = ConnectionResult::new();

    writeln!(stream, "{}", MSG_READY)?;

    let mut reader = BufReader::new(stream.try_clone()?);

    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        // read_line will leave trailing newlines which must be removed
        match result.feed_line(line.trim_right_matches(|c: char|{ c== '\n' || c == '\r'})) {
            Ok("") => {}
            Ok("221 Bye") => { break; }
            Ok(s) => {
                writeln!(stream, "{}", s)?;
            },
            Err(e) => {
                writeln!(stream, "{}", e)?;
            }
        }
    }

    Ok(result)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_message() {
        // When
        let mut result = ConnectionResult::new();

        result.feed_line("HELO localhost").unwrap();
        result.feed_line("MAIL FROM: tester@localhost").unwrap();
        result.feed_line("RCPT TO: admin@localhost").unwrap();
        result.feed_line("DATA").unwrap();
        result.feed_line("It works!").unwrap();
        result.feed_line(".").unwrap();
        result.feed_line("QUIT").unwrap();

        // Then
        assert_eq!(result.get_sender_domain(), Some("localhost"));

        let messages = result.get_messages().unwrap();
        assert_eq!(messages.len(), 1);

        let message = messages.first().unwrap();

        assert_eq!(message.get_sender(), "tester@localhost");
        assert_eq!(message.get_recipients().join(", "), "admin@localhost");
        assert_eq!(message.get_data(), "It works!");
    }
}