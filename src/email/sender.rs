pub trait EmailSender {
    async fn send_email(&self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> anyhow::Result<()>;
}
