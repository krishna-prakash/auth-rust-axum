use resend_rs::{Resend, types::CreateEmailBaseOptions};

use crate::config::mail::MailConfig;

pub async fn send_mail(
    mail_config: &MailConfig,
    to: &[String],
    subject: &String,
    body: &str,
) -> Result<(), String> {
    
    let resend = Resend::new(&mail_config.api_key);
    let email = CreateEmailBaseOptions::new(
        &mail_config.smtp_from_address,
        to,
        subject
    ).with_html(&body);

    let email = resend.emails.send(email).await;
    
    match email {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}