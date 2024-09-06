use serde::{Deserialize, Serialize};

// ------------ 結構體
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub security: SecurityType,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SmtpContent {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub html: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SmtpCombine {
    pub config: SmtpConfig,
    pub content: SmtpContent,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum SecurityType {
    None, // 25 or other ports
    TLS, // 465
    StartTLS, // 587
}