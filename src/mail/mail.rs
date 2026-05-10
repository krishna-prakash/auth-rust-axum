use resend_rs::{Resend, types::CreateEmailBaseOptions};

use crate::config::mail::MailConfig;


pub async fn send_verification_email(
    mail_config: &MailConfig, 
) -> Result<(), &'static str> {
    let resend = Resend::new(&mail_config.api_key);
    let from_email = "onboarding@resend.dev";
    let to = ["krishnakprakash296@gmail.com"];
    let subject = "Dev check";

    let email = CreateEmailBaseOptions::new(from_email, to, subject)
        .with_html("<p>This is first outgoing email from web service</p>");

    let _email = resend.emails.send(email).await;
    print!("{:?}", _email);
    Ok(())
}