use lettre::{
    message::{header::ContentType, Mailbox},
    Message, SmtpTransport, Transport,
};

use crate::data_access;

pub struct Email {
    pub recipient_email: String,
    pub mail_type: EmailType,
    pub invite_code: Option<String>,
}

impl Email {
    pub fn new(recipient_email: String, _type: EmailType, invite_code: Option<String>) -> Self {
        Email {
            recipient_email,
            mail_type: _type,
            invite_code,
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

pub struct otherEmail {
    pub recipient_email: String,
    pub subject: String,
    pub body: String,
}

pub enum EmailType {
    RegisterUser,
    RegisterUserCompany,
    ResetPassword,
    Support,
    other(otherEmail),
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

    let email: Result<Message, lettre::error::Error> = match email.mail_type {
        EmailType::RegisterUser => {
            let email_template = register_user_template(&email.recipient_email);
            let email = Message::builder()
                .from(from)
                .to(to)
                .subject("Confirm Your User Account on ProFlex")
                .header(ContentType::TEXT_HTML)
                .body(email_template)
                .unwrap();
            Ok(email)
        }
        EmailType::RegisterUserCompany => {
            let email_template = register_user_company_template(&email.recipient_email);
            let email = Message::builder()
                .from(from)
                .to(to)
                .subject("Confirm Your Company Account on ProFlex")
                .header(ContentType::TEXT_HTML)
                .body(email_template)
                .unwrap();
            Ok(email)
        }
        EmailType::ResetPassword => {
            let email_template = reset_password_template(&email.recipient_email);
            let email = Message::builder()
                .from(from)
                .to(to)
                .subject("Reset Your Password on ProFlex")
                .header(ContentType::TEXT_HTML)
                .body(email_template)
                .unwrap();
            Ok(email)
        }
        EmailType::Support => {
            let email_template = support_template();
            let email = Message::builder()
                .from(from)
                .to(to)
                .subject("Support Request on ProFlex")
                .header(ContentType::TEXT_HTML)
                .body(email_template)
                .unwrap();
            Ok(email)
        }
        EmailType::other(otherEmail) => {
            let email_template = other_template(otherEmail);
            let email = Message::builder()
                .from(from)
                .to(to)
                .subject(otherEmail.subject)
                .header(ContentType::TEXT_HTML)
                .body(otherEmail.body)
                .unwrap();
            Ok(email)
        }
    };

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

fn register_user_company_template(invite_code: &str) -> String {
    let email_template = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">

        <head>
            <meta charset="UTF-8">
            <title>Confirm Your Company Account on ProFlex</title>
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
                <p>Thank you for accepting our invitation to create a company account on ProFlex. We're thrilled to have you on
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

fn reset_password_template(invite_code: &str) -> String {
    let email_template = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">

        <head>
            <meta charset="UTF-8">
            <title>Reset Your Password on ProFlex</title>
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
                <p>We have received a request to reset your password. If you did not request this, please ignore this e-mail.
                </p>
                <p>To reset your password, please click the following link: <a
                        href="https://group04.web-tek.ninja/reset/{invite_code}">Reset your password</a>.</p>
                <p><b>Best regards,<br>
                    The ProFlex Team</b></p>
            </div>
        </body>

        </html>
    "#,
        invite_code = invite_code
    );

    email_template
}

fn support_template() -> String {
    let email_template = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">

        <head>
            <meta charset="UTF-8">
            <title>Support Request on ProFlex</title>
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
                <p>We have received your support request. We will get back to you as soon as possible.
                </p>
                <p><b>Best regards,<br>
                    The ProFlex Team</b></p>
            </div>
        </body>

        </html>
    "#
    );

    email_template
}

fn other_template(email: otherEmail) -> String {
    let email_template = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">

        <head>
            <meta charset="UTF-8">
            <title>{subject}</title>
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
                <p>{body}</p>
                <p><b>Best regards,<br>
                    The ProFlex Team</b></p>
            </div>
        </body>

        </html>
    "#,
        subject = email.subject,
        body = email.body
    );

    email_template
}
