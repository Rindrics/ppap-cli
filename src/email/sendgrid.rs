use super::config::SendGridConfig;
use super::sender::EmailSender;
use anyhow::Context;
use base64::Engine;
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
    #[serde(skip_serializing_if = "Option::is_none")]
    attachments: Option<Vec<Attachment>>,
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

#[derive(Serialize)]
struct Attachment {
    content: String,
    filename: String,
    #[serde(rename = "type")]
    content_type: String,
    disposition: String,
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

    pub async fn send_email_with_attachment(
        &self,
        to: &str,
        subject: &str,
        body: &str,
        attachment_path: &str,
    ) -> anyhow::Result<()> {
        println!("Sending email with attachment via SendGrid REST API...");
        println!("From: {}", self.from_address);
        println!("To: {}", to);
        println!("Subject: {}", subject);
        println!("Attachment: {}", attachment_path);

        // Read file and encode to base64
        let file_content = std::fs::read(attachment_path)
            .with_context(|| format!("Failed to read attachment: {}", attachment_path))?;

        // Encode to base64 without line breaks (important for SendGrid!)
        let base64_content = base64::engine::general_purpose::STANDARD.encode(&file_content);

        // Get filename from path
        let filename = std::path::Path::new(attachment_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("attachment.zip")
            .to_string();

        // Determine MIME type (zip files)
        let content_type = if filename.ends_with(".zip") {
            "application/zip"
        } else {
            "application/octet-stream"
        };

        let attachment = Attachment {
            content: base64_content,
            filename,
            content_type: content_type.to_string(),
            disposition: "attachment".to_string(),
        };

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
            attachments: Some(vec![attachment]),
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

        // Note: Don't print full JSON with base64 content (too large)
        println!("Request body: [Mail with attachment - omitted for brevity]");

        let response = self
            .client
            .post("https://api.sendgrid.com/v3/mail/send")
            .headers(headers)
            .json(&mail)
            .send()
            .await?;

        let status = response.status();
        println!("\nResponse status: {}", status);

        if status.is_success() {
            println!("Email with attachment sent successfully!");
            Ok(())
        } else {
            let error_body = response.text().await?;
            let error = format!(
                "Failed to send email with attachment: Status: {}, Body: {}",
                status, error_body
            );
            println!("Error: {}", error);
            Err(anyhow::anyhow!(error))
        }
    }
}

impl EmailSender for SendGridRestSender {
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> anyhow::Result<()> {
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
            attachments: None,
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

        let response = self
            .client
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
                status, error_body
            );
            println!("Error: {}", error);
            Err(anyhow::anyhow!(error))
        }
    }
}
