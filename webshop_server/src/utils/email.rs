pub struct Email {
    pub recipient_email: String,
    pub subject: String,
    pub body: String,
}

impl Email {
    pub fn new(
        recipient_email: String,
        subject: String,
        body: String,
    ) -> Self {
        Self {
            recipient_email,
            subject,
            body,
        }
    }
}

pub enum MailError {
    InvalidRecipient,
    InvalidSubject,
    InvalidBody,

    SendError,
    NotImplemented
}

pub async fn send_email(
    email: Email,
) -> Result<(), MailError> {
    log::info!("Sending email to {}\n with content {}", email.recipient_email, email.body);
    return Err(MailError::NotImplemented);
}