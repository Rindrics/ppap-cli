use super::sender::EmailSender;
use super::config::SendGridConfig;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::Serialize;

pub struct SendGridRestSender {
    api_key: String,
    from_address: String,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct SendGridMail {
    personalizations: Vec<Personalization>,
    from: EmailAddress,
    subject: String,
    content: Vec<Content>,
}

#[derive(Serialize)]
struct Personalization {
    to: Vec<EmailAddress>,
}

#[derive(Serialize)]
struct EmailAddress {
    email: String,
}

#[derive(Serialize)]
struct Content {
    r#type: String,
    value: String,
}

impl SendGridRestSender {
    pub fn new(config: &SendGridConfig) -> Self {
        Self {
            api_key: config.api_key.clone(),
            from_address: config.from_address.clone(),
            client: reqwest::Client::new(),
        }
    }

    fn create_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.api_key))
                .expect("Invalid API key format"),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers
    }
}

impl EmailSender for SendGridRestSender {
    async fn send_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> anyhow::Result<()> {
        println!("Sending email via SendGrid REST API...");
        println!("From: {}", self.from_address);
        println!("To: {}", to);
        println!("Subject: {}", subject);

        let mail = SendGridMail {
            personalizations: vec![Personalization {
                to: vec![EmailAddress {
                    email: to.to_string(),
                }],
            }],
            from: EmailAddress {
                email: self.from_address.clone(),
            },
            subject: subject.to_string(),
            content: vec![Content {
                r#type: "text/plain".to_string(),
                value: body.to_string(),
            }],
        };

        println!("Request headers:");
        let headers = self.create_headers();
        for (key, value) in headers.iter() {
            if key == "Authorization" {
                println!("  {}: Bearer **********", key);
            } else {
                println!("  {}: {:?}", key, value);
            }
        }

        println!("Request body (sanitized):");
        if let Ok(json) = serde_json::to_string_pretty(&mail) {
            println!("{}", json);
        }

        let response = self.client
            .post("https://api.sendgrid.com/v3/mail/send")
            .headers(headers)
            .json(&mail)
            .send()
            .await?;

        let status = response.status();
        println!("\nResponse status: {}", status);
        println!("Response headers:");
        for (key, value) in response.headers() {
            println!("  {}: {:?}", key, value);
        }

        if status.is_success() {
            println!("Email sent successfully!");
            Ok(())
        } else {
            let error_body = response.text().await?;
            let error = format!(
                "Failed to send email: Status: {}, Body: {}",
                status,
                error_body
            );
            println!("Error: {}", error);
            Err(anyhow::anyhow!(error))
        }
    }
}
