# SMTP (Simple Mail Transfer Protocol)
> 一種用於發送和接收電子郵件的網路標準協議；<br/>
> 他定義了電子郵件伺服器之間如何互相傳送、接收與中繼郵件的規則。<br/>
> 郵件的接收由 `POP3 (郵局協議|接收)` 或 `IMAP (網路消息訪問協議|發送)` 來處理。

- POP3 (Post Office Protocol 3)
> POP3 是 TCP/IP 協議中的一員，由 RFC1939 進行定義。<br/>
> 本協議主要用於支持使用客戶端遠端管理在服務器上的電子郵件。<br/>
> 提供了 SSL 加密的 POP3 協議，即 `POP3S`。
- IMAP (Internet Message Access Protocol)
> IMAP 以前稱作`交互郵件訪問協議`是一個應用層協議，
> 用來以本地郵件客戶端工具(如: MS Outlook, Outlook Express, Foxmail, Mozilla Thunderbird) 存取遠端服務器的郵件。<br/>



- 25: 傳統的非加密 SMTP 端口（通常被阻擋或不推薦使用）
- 465: SSL 加密的 SMTP 端口
- 587: TLS 加密的 SMTP 端口（現代推薦使用）

### 從 mailer 變成 mail_builder
> 使用 `SmtpTransport::builder_dangerous` 替代 `SmtpTransport::relay`，<br/>
> 這允許我們連接到非標準端口（如 Mailpit 的 1025）<br/><br/>
> 在使用 builder 去指定是否加密連接。