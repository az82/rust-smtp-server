
const HELO_START: &str = "HELO ";
const MAIL_START: &str = "MAIL FROM:";
const RCPT_START: &str = "RCPT TO:";
const DATA_LINE: &str = "DATA";
const QUIT_LINE: &str = "QUIT";


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


pub struct Response {
    code: u16,
    message: String
}

pub struct Message {
    sender: String,
    recipients: Vec<String>,
    data: Vec<String>
}


pub struct Parser {
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


impl Parser {
    pub fn new() -> Parser {
        Parser {
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

    pub fn feed_line(&mut self, line: &str) -> Result<(), String> {
        match self.state {
            State::Helo => {
                if line.starts_with(HELO_START) {
                    self.sender_domain = line[HELO_START.len()..].trim().to_string();
                    self.state = State::Mail;
                    Ok(())
                } else {
                    Err(format!("Unexpected line: {}", line).to_string())
                }
            },
            State::Mail => {
                if line.starts_with(MAIL_START) {
                    self.next_sender = line[MAIL_START.len()..].trim().to_string();
                    self.state = State::Rcpt;
                    Ok(())
                } else {
                    Err(format!("Unexpected line: {}", line).to_string())
                }
            },
            State::Rcpt => {
                if line.starts_with(RCPT_START) {
                    self.next_recipients.push(line[RCPT_START.len()..].trim().to_string());
                    self.state = State::RcptOrData;
                    Ok(())
                } else {
                    Err(format!("Unexpected line: {}", line).to_string())
                }
            },
            State::RcptOrData => {
                if line.starts_with(RCPT_START) {
                    self.next_recipients.push(line[RCPT_START.len()..].trim().to_string());
                    Ok(())
                } else if line == DATA_LINE {
                    self.state = State::Dot;
                    Ok(())
                } else {
                    Err(format!("Unexpected line: {}", line).to_string())
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
                    Ok(())
                } else {
                    self.next_data.push(line.to_string());
                    Ok(())
                }
            },
            State::MailOrQuit => {
                if line.starts_with(MAIL_START) {
                    self.next_sender = line[MAIL_START.len()..].trim().to_string();
                    self.state = State::Rcpt;
                    Ok(())
                } else if line == QUIT_LINE {
                    self.state = State::Done;
                    Ok(())
                } else {
                    Err(format!("Unexpected line: {}", line).to_string())
                }
            },
            State::Done => {
                Err(format!("Unexpected line: {}", line).to_string())
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_message() {
        // When
        let mut parser = Parser::new();

        // TODO: Read from file
        parser.feed_line("HELO localhost");
        parser.feed_line("MAIL FROM: tester@localhost");
        parser.feed_line("RCPT TO: admin@localhost");
        parser.feed_line("DATA");
        parser.feed_line("It works!");
        parser.feed_line(".");
        parser.feed_line("QUIT");

        // Then
        assert_eq!(parser.get_sender_domain(), Some("localhost"));

        let messages = parser.get_messages().unwrap();
        assert_eq!(messages.len(), 1);

        let message = messages.first().unwrap();

        assert_eq!(message.get_sender(), "tester@localhost");
        assert_eq!(message.get_recipients().join(", "), "admin@localhost");
        assert_eq!(message.get_data(), "It works!");
    }
}