use serde::{Deserialize};
use reqwest::blocking::Client;
use std::thread;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct Gempa {
    #[serde(rename = "Tanggal")]
    tanggal: String,
    #[serde(rename = "Jam")]
    jam: String,
    #[serde(rename = "Magnitude")]
    magnitude: String,
    #[serde(rename = "Wilayah")]
    wilayah: String,
    #[serde(rename = "Potensi")]
    potensi: String,
}

#[derive(Debug, Deserialize)]
struct Infogempa {
    #[serde(rename = "gempa")]
    gempa: Gempa,
}

#[derive(Debug, Deserialize)]
struct Root {
    #[serde(rename = "Infogempa")]
    infogempa: Infogempa,
}

fn main() {
    let client = Client::new();
    let url = "https://data.bmkg.go.id/DataMKG/TEWS/autogempa.xml";
    let token = std::env::var("TELEGRAM_TOKEN").expect("TOKEN BELUM ADA");
    let chat_id = std::env::var("TELEGRAM_CHAT_ID").expect("CHAT_ID BELUM ADA");

    println!("Bot Monitoring Gempa Jalan...");

    loop {
        match client.get(url).send() {
            Ok(res) => {
                let body = res.text().unwrap_or_default();
                
                if let Ok(data) = serde_xml_rs::from_str::<Root>(&body) {
                    let g = data.infogempa.gempa;
                    let pesan = format!(
                        "⚠️ *INFO GEMPA TERBARU*\n\n📅 Tanggal: {}\n⏰ Jam: {}\n📉 Magnitude: {}\n📍 Wilayah: {}\n📢 Potensi: {}",
                        g.tanggal, g.jam, g.magnitude, g.wilayah, g.potensi
                    );
                    
                    let send_url = format!("https://api.telegram.org/bot{}/sendMessage", token);
                    let _ = client.post(send_url)
                        .form(&[("chat_id", &chat_id), ("text", &pesan), ("parse_mode", &"Markdown".to_string())])
                        .send();
                }
            }
            Err(e) => println!("Error: {}", e),
        }
        thread::sleep(Duration::from_secs(60));
    }
}
