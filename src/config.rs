#[derive(Debug, Clone)]
pub struct EmailConfig {
    pub smtp_server: String,
    pub smtp_port: u16,
    pub username: String,
    pub password: String,
}

impl EmailConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv::dotenv().ok();

        Ok(Self {
            smtp_server: std::env::var("SMTP_SERVER")
                .map_err(|_| anyhow::anyhow!("SMTP_SERVER is not set"))?,
            smtp_port: std::env::var("SMTP_PORT")
                .map_err(|_| anyhow::anyhow!("SMTP_PORT is not set"))?
                .parse()?,
            username: std::env::var("SMTP_USERNAME")
                .map_err(|_| anyhow::anyhow!("SMTP_USERNAME is not set"))?,
            password: std::env::var("SMTP_PASSWORD")
                .map_err(|_| anyhow::anyhow!("SMTP_PASSWORD is not set"))?,
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn setup_test_env() {
        env::set_var("SMTP_SERVER", "smtp.example.com");
        env::set_var("SMTP_PORT", "587");
        env::set_var("SMTP_USERNAME", "test@example.com");
        env::set_var("SMTP_PASSWORD", "password123");
    }

    #[test]
    fn test_from_env() {
        setup_test_env();

        let config = EmailConfig::from_env().unwrap();

        assert_eq!(config.smtp_server, "smtp.example.com");
        assert_eq!(config.smtp_port, 587);
        assert_eq!(config.username, "test@example.com");
        assert_eq!(config.password, "password123");
    }
}
