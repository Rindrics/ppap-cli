#[derive(Debug, Clone)]
pub enum SendGridProtocol {
    Rest,
    Smtp,
}

#[derive(Debug, Clone)]
pub struct SendGridConfig {
    pub api_key: String,
    #[allow(dead_code)]
    pub protocol: SendGridProtocol,
    pub from_address: String,
}

impl SendGridConfig {
    fn from_values(
        api_key: Option<String>,
        protocol: Option<String>,
        from_address: Option<String>,
    ) -> anyhow::Result<Self> {
        let api_key = api_key.ok_or_else(|| anyhow::anyhow!("SendGrid API key is not set"))?;

        if api_key.trim().is_empty() {
            return Err(anyhow::anyhow!("SendGrid API key cannot be empty"));
        }

        let protocol = match protocol
            .unwrap_or_else(|| "rest".to_string())
            .to_lowercase()
            .as_str()
        {
            "smtp" => SendGridProtocol::Smtp,
            _ => SendGridProtocol::Rest,
        };

        let from_address =
            from_address.ok_or_else(|| anyhow::anyhow!("From address is not set"))?;

        println!("SendGrid Configuration:");
        println!("  API Key: {}", api_key);
        println!("  Protocol: {:?}", protocol);
        println!("  From Address: {}", from_address);

        Ok(Self {
            api_key,
            protocol,
            from_address,
        })
    }

    pub fn from_env() -> anyhow::Result<Self> {
        dotenv::dotenv().ok(); // .env ファイルを読み込み

        let api_key = std::env::var("SENDGRID_API_KEY").ok();
        let protocol = std::env::var("SENDGRID_PROTOCOL").ok();
        let from_address = std::env::var("EMAIL_FROM_ADDRESS").ok();

        Self::from_values(api_key, protocol, from_address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_with_rest_protocol() {
        let config = SendGridConfig::from_values(
            Some("test_api_key".to_string()),
            Some("rest".to_string()),
            Some("foo@example.com".to_string()),
        )
        .unwrap();
        assert!(matches!(config.protocol, SendGridProtocol::Rest));
    }

    #[test]
    fn test_config_with_smtp_protocol() {
        let config = SendGridConfig::from_values(
            Some("test_api_key".to_string()),
            Some("smtp".to_string()),
            Some("foo@example.com".to_string()),
        )
        .unwrap();
        assert!(matches!(config.protocol, SendGridProtocol::Smtp));
    }

    #[test]
    fn test_config_with_default_protocol() {
        let config = SendGridConfig::from_values(
            Some("test_api_key".to_string()),
            None,
            Some("foo@example.com".to_string()),
        )
        .unwrap();
        assert!(matches!(config.protocol, SendGridProtocol::Rest));
    }

    #[test]
    fn test_config_without_api_key() {
        let result = SendGridConfig::from_values(None, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_with_invalid_protocol() {
        let config = SendGridConfig::from_values(
            Some("test_api_key".to_string()),
            Some("invalid_protocol".to_string()),
            Some("foo@example.com".to_string()),
        )
        .unwrap();
        assert!(matches!(config.protocol, SendGridProtocol::Rest));
    }
}
