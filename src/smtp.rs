use lettre::{Message, SmtpTransport, Transport};
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

    pub fn send_test_email(&self, to: &str) -> anyhow::Result<()> {
        let email = Message::builder()
            .from(self.from_address.parse()?)
            .to(to.parse()?)
            .subject("Test Email")
            .body(String::from("This is a test email"))?;

        self.transport.send(&email)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::EmailConfig;

    struct MockSmtpClient {
        sent_emails: Vec<String>,
    }

    impl MockSmtpClient {
        fn new() -> Self {
            Self {
                sent_emails: Vec::new(),
            }
        }

        fn send(&mut self, to: &str) -> anyhow::Result<()> {
            self.sent_emails.push(to.to_string());
            Ok(())
        }
    }

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

    #[test]
    fn test_mock_email_sending() {
        let mut mock_client = MockSmtpClient::new();
        assert!(mock_client.send("test@example.com").is_ok());
        assert_eq!(mock_client.sent_emails.len(), 1);
        assert_eq!(mock_client.sent_emails[0], "test@example.com");
    }
}
