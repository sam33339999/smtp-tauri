// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use lettre::message::header::ContentType;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use serde::{Deserialize, Serialize};
use tokio::task;

// ------------ 結構體
#[derive(Deserialize, Serialize, Debug, Clone)]
struct SmtpConfig {
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
    security: SecurityType,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct SmtpContent {
    from: String,
    to: String,
    subject: String,
    html: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct SmtpCombine {
    config: SmtpConfig,
    content: SmtpContent,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum SecurityType {
    None, // 25 or other ports
    TLS, // 465
    StartTLS, // 587
}

// ------------ 方法
#[tauri::command]
async fn send_mail(smtp: SmtpCombine) -> Result<String, String> {
    task::spawn_blocking(move || {
        let mail = Message::builder()
            .from(smtp.content.from.clone().parse().unwrap())
            .to(smtp.content.to.clone().parse().unwrap())
            .subject(smtp.content.subject.clone())
            .header(ContentType::TEXT_HTML)
            .body(smtp.content.html.clone())
            .unwrap();

        // println!("Email message built successfully");
        let mut mailer_builder = SmtpTransport::builder_dangerous(&smtp.config.host)
            .port(smtp.config.port);

        // println!("SMTP Transport builder created with host {} and port {}", smtp.config.host, smtp.config.port);
        if let (Some(username), Some(password)) = (smtp.config.username, smtp.config.password) {
            let credentials = Credentials::new(username, password);
            mailer_builder = mailer_builder.credentials(credentials);
            // println!("Credentials added to SMTP transport");
        } else {
            // println!("No credentials provided, skipping authentication");
        }

        match smtp.config.security {
            SecurityType::None => { // other ports
                // println!("No encryption specified");
            },
            SecurityType::TLS => { // 465
                // println!("Attempting to use TLS encryption");
                mailer_builder = mailer_builder.tls(Tls::Wrapper( // Wrapper
                    TlsParameters::new(smtp.config.host.clone())
                        .map_err(|e| format!("TLS error: {:?}", e))?
                ))
            },
            SecurityType::StartTLS => { // 587
                // println!("Attempting to use StartTLS");
                mailer_builder = mailer_builder.tls(Tls::Required( // Required
                    TlsParameters::new(smtp.config.host.clone())
                        .map_err(|e| format!("TLS error: {:?}", e))?
                ))
            },
        }
        let mailer = mailer_builder.build();
        match mailer.send(&mail) {
            Ok(_) => Ok(format!("Email sent successfully!")),
            Err(e) => Err(format!("Could not send email: {:?}", e)),
        }
    }).await.unwrap()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send_mail])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
