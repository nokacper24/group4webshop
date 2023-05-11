use lettre::{
    message::{header::ContentType, Mailbox},
    Message, SmtpTransport, Transport,
};

use crate::data_access;

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

pub async fn send_email(email: Email, mailer: &SmtpTransport) -> Result<(), MailError> {
    // log::info!(
    //     "Sending email to {}\n with content {}",
    //     email.recipient_email,
    //     email.body
    // );
    // Err(MailError::NotImplemented)
    let email = match generate_email(email) {
        Ok(email) => email,
        Err(e) => return Err(e),
    };

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    };
    Ok(())
}

fn generate_email(email: Email) -> Result<Message, MailError> {
    let from: Mailbox = match "ProFlex <group04webshop@gmail.com>".parse() {
        Ok(mailbox) => mailbox,
        Err(_) => return Err(MailError::SendError),
    };

    let to: Mailbox = match email.recipient_email.parse() {
        Ok(mailbox) => mailbox,
        Err(_) => return Err(MailError::InvalidRecipient),
    };

    let email = Message::builder()
        .from(from)
        .to(to)
        .subject("Confirm Your User Account on ProFlex")
        .header(ContentType::TEXT_HTML)
        .body(register_user_template(&email.body));

    match email {
        Ok(email) => Ok(email),
        Err(_) => Err(MailError::SendError),
    }
}

fn register_user_template(invite_code: &str) -> String {
    let email_template = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">

        <head>
            <meta charset="UTF-8">
            <title>Confirm Your User Account on ProFlex</title>
        </head>

        <style>
            p {{
                margin-top: 2em;
                margin-bottom: 2em;
            }}
        </style>

        <body style=" font-family: sans-serif">
            <div style="padding: 1em;">
                <h1 style="font-size: 1.2em">Dear Customer,</h1>
                <p>Thank you for accepting our invitation to create a user account on ProFlex. We're thrilled to have you on
                    board!
                </p>
                <p>To complete your registration, please click the following link: <a
                        href="https://group04.web-tek.ninja/verify/{invite_code}">Verify your e-mail</a>.</p>
                <p>You'll be asked to set
                    up your account and create a password. Once you've completed this step, you'll be
                    able to access all of ProFlex's features. Thank you for choosing us. We look forward to working with you.
                </p>
                <p><b>Best regards,<br>
                    The ProFlex Team</b></p>
                <p style="font-size: 0.8em; opacity: 0.8;">If you did not request this invitation, please ignore this e-mail.
                </p>
            </div>
        </body>

        </html>
    "#,
        invite_code = invite_code
    );

    email_template
}
