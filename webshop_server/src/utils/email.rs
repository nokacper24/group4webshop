use lettre::{Message,SmtpTransport, message::header::ContentType, Transport};

pub struct Email {
    pub recipient_email: String,
    pub subject: String,
    pub body: String,
}

impl Email {
    pub fn new(recipient_email: String, subject: String, body: String) -> Self {
        Self {
            recipient_email,
            subject,
            body,
        }
    }
}

#[derive(Debug)]
pub enum MailError {
    InvalidRecipient,
    InvalidSubject,
    InvalidBody,

    SendError,
    NotImplemented,
}

impl std::fmt::Display for MailError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MailError::InvalidRecipient => write!(f, "Invalid recipient"),
            MailError::InvalidSubject => write!(f, "Invalid subject"),
            MailError::InvalidBody => write!(f, "Invalid body"),
            MailError::SendError => write!(f, "Error sending email"),
            MailError::NotImplemented => write!(f, "Email sending not implemented"),
        }
    }
}

pub async fn send_email(email: Email,mailer: &SmtpTransport) -> Result<(), MailError> {
    // log::info!(
    //     "Sending email to {}\n with content {}",
    //     email.recipient_email,
    //     email.body
    // );
    // Err(MailError::NotImplemented)
    let email = generate_email(email.recipient_email);

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    };
    Ok(())
}


fn generate_email(to: String) -> Message
{
    let email = Message::builder()
    .from("ProFlex <group04webshop@gmail.com>".parse().unwrap())
    .to(to.parse().unwrap())
    .subject("test email")
    .header(ContentType::TEXT_PLAIN)
    .body(String::from("It works!!!"))
    .unwrap();
    email
}