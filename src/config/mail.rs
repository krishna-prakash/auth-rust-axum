#[derive(Debug, Clone)]
pub struct MailConfig {
    pub api_key: String,
    pub smtp_from_address: String,
    pub mail_template_path: String,
    pub backend_base_url: String
}

impl MailConfig {
    pub fn init() -> Self {
        let api_key = std::env::var("EMAIL_API_KEY").expect("API key must be set");
        let smtp_from_address = std::env::var("FROM_EMAIL").expect("From email must be set");
        let mail_template_path = std::env::var("EMAIL_TEMPLATE_PATH").expect("email template path must be set");
        let backend_base_url = std::env::var("BACKEND_BASE_URL").expect("email template path must be set");
        
        Self {
            api_key,
            smtp_from_address,
            mail_template_path,
            backend_base_url
        }
    }
}