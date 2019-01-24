
const HELO_START: &str = "HELO ";
const MAIL_START: &str = "MAIL FROM:";
const RCPT_START: &str = "RCPT TO:";
const DATA_LINE: &str = "DATA";
const QUIT_LINE: &str = "QUIT";

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


struct Parser {
    state: State,
    sender_domain: String,
    messages: Vec<Message>,
    next_sender: String,
    next_recipients: Vec<String>,
    next_data: Vec<String>
}

impl Message {
    fn new() -> Message {
        Message {
            sender: "".to_string(),
            recipients: Vec::new(),
            data: Vec::new()
        }
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

    pub fn get_messages(&self) -> Option<&Vec<Message>> {
        match self.state {
            State::Done => Some(&self.messages),
            _ => None
        }
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
                    self.next_data.push(line[RCPT_START.len()..].to_string());
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