use lettre::SmtpTransport;
use lettre::transport::smtp::authentication::Credentials;

pub struct SmtpClient {
    transport: SmtpTransport,
    from_address: String,
}

impl SmtpClient {
    pub fn new(config: &crate::config::EmailConfig) -> anyhow::Result<Self> {
        let creds = Credentials::new(
            config.username.clone(),
            config.password.clone(),
        );

        let transport = SmtpTransport::relay(&config.smtp_server)?
            .port(config.smtp_port)
            .credentials(creds)
            .build();

        Ok(Self {
            transport,
            from_address: config.username.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::EmailConfig;

    #[test]
    fn test_smtp_client_creation() {
        let config = EmailConfig {
            smtp_server: "smtp.example.com".to_string(),
            smtp_port: 587,
            username: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let client = SmtpClient::new(&config);
        assert!(client.is_ok());
    }
}
