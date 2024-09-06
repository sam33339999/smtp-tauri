use lettre::{Message, message::header::ContentType, SmtpTransport, transport::smtp::{authentication::Credentials, client::{TlsParameters, Tls}}, Transport};
use tokio::task;

use super::structure::{SmtpCombine, SecurityType};


// ------------ 方法
#[tauri::command]
pub async fn send_mail(smtp: SmtpCombine) -> Result<String, String> {
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