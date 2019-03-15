use std::io::{BufRead, Error, Write};

// Client commands
const HELO_START: &str = "HELO ";
const MAIL_START: &str = "MAIL FROM:";
const RCPT_START: &str = "RCPT TO:";
const DATA_LINE: &str = "DATA";
const QUIT_LINE: &str = "QUIT";

// Server responses
const MSG_READY: &str = "220 ready";
const MSG_OK: &str = "250 OK";
const MSG_SEND_MESSAGE_CONTENT: &str = "354 Send message content";
const MSG_BYE: &str = "221 Bye";
const MSG_SYNTAX_ERROR: &str = "500 unexpected line";

/// An Email message
pub struct Message {
    sender: String,
    recipients: Vec<String>,
    data: Vec<String>,
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

/// SMTP States
///
/// States are named by the next expected command(s).
enum State {
    Helo,
    Mail,
    Rcpt,
    RcptOrData,
    Dot,
    MailOrQuit,
    Done,
}

/// The state of a SMTP connection.
pub struct Connection {
    state: State,
    sender_domain: String,
    messages: Vec<Message>,
    next_sender: String,
    next_recipients: Vec<String>,
    next_data: Vec<String>,
}

impl Connection {
    pub fn new() -> Connection {
        Connection {
            state: State::Helo,
            sender_domain: "".to_string(),
            messages: Vec::new(),
            next_sender: "".to_string(),
            next_recipients: Vec::new(),
            next_data: Vec::new(),
        }
    }

    /// Handle an incoming connection
    pub fn handle(reader: &mut BufRead, writer: &mut Write) -> Result<Connection, Error> {
        let mut result = Connection::new();

        writeln!(writer, "{}", MSG_READY)?;

        // TODO: Do something here /////////////////////////////////////////////////////////////////

        Ok(result)
    }

    fn get_if_done<R, F: FnOnce() -> R>(&self, getter: F) -> Option<R> {
        match self.state {
            State::Done => Some(getter()),
            _ => None,
        }
    }

    pub fn get_messages(&self) -> Option<&Vec<Message>> {
        self.get_if_done(|| &self.messages)
    }

    pub fn get_sender_domain(&self) -> Option<&str> {
        self.get_if_done(|| self.sender_domain.as_str())
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn parse_message() {
        // When
        let request = "HELO localhost\n\
                       MAIL FROM: tester@localhost\n\
                       RCPT TO: admin@localhost\n\
                       DATA\n\
                       It works!\n\
                       .\n\
                       QUIT\n";
        let mut reader = BufReader::new(request.as_bytes());

        let mut response_bytes = Vec::new();
        let result = Connection::handle(&mut reader, &mut response_bytes).unwrap();
        let response = String::from_utf8(response_bytes).unwrap();

        // Then
        assert_eq!(result.get_sender_domain(), Some("localhost"));

        let messages = result.get_messages().unwrap();
        assert_eq!(messages.len(), 1);

        let message = messages.first().unwrap();

        assert_eq!(message.get_sender(), "tester@localhost");
        assert_eq!(message.get_recipients().join(", "), "admin@localhost");
        assert_eq!(message.get_data(), "It works!");

        assert_eq!(
            response,
            "220 ready\n\
             250 OK\n\
             250 OK\n\
             250 OK\n\
             354 Send message content\n\
             250 OK\n\
             221 Bye\n"
        )
    }
}
