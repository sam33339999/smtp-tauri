// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use lettre::message::header::ContentType;
// use lettre::{SmtpTransport, transport::smtp::{authentication::Credentials}, Message};
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use serde::{Deserialize, Serialize};
use tokio::task;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct SmtpConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct SmtpContent {
    from: String,
    to: String,
    subject: String,
    text: String,
    html: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct SmtpCombine {
    config: SmtpConfig,
    content: SmtpContent,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn send_mail(smtp: SmtpCombine) -> Result<String, String> {
    task::spawn_blocking(move || {
        let credentials = Credentials::new(
            smtp.config.username.clone(),
            smtp.config.password.clone(),
        );

        let mail = Message::builder()
        .from(smtp.content.from.clone().parse().unwrap())
        .to(smtp.content.to.clone().parse().unwrap())
        .subject(smtp.content.subject.clone())
        .header(ContentType::TEXT_HTML)
        .body(smtp.content.html.clone())
        .unwrap();

        // let mailer = SmtpTransport::builder_dangerous(smtp.config.host.clone().as_str())
        let mailer = SmtpTransport::relay(smtp.config.host.clone().as_str())
            .unwrap()
            .port(smtp.config.port.clone())
            .credentials(credentials)
            .build();

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
