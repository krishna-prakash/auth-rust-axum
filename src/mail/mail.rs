use resend_rs::{Resend, types::CreateEmailBaseOptions};
use uuid::Uuid;

use crate::{config::mail::MailConfig, mail::sendmail::send_mail};


pub async fn send_verification_email(
    mail_config: &MailConfig, 
    to_email: &str,
    verification_token: Uuid
) -> Result<(), String> {
    // contruct verification email here

    let subject = "Verification email".to_string();
    // let backend_base = format!("{}/auth/verify", &mail_config.backend_base_url);
    // let template_path = format!("{}/verification_email.html", &mail_config.mail_template_path);
    let to_email = [to_email.to_string()];

    let body = include_str!("./templates/verification_email.html");
    println!("{:?}", body);

    send_mail(&mail_config, &to_email, &subject, body).await    
}