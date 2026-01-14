use reqwest::blocking::Client;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct Gempa {
    #[serde(rename = "Tanggal")]
    tanggal: String,
    #[serde(rename = "Jam")]
    jam: String,
    #[serde(rename = "Magnitude")]
    magnitude: String,
    #[serde(rename = "Kedalaman")]
    kedalaman: String,
    #[serde(rename = "Wilayah")]
    wilayah: String,
    #[serde(rename = "Potensi")]
    potensi: String,
}

#[derive(Deserialize, Debug)]
struct Infogempa {
    gempa: Gempa,
}

#[derive(Deserialize, Debug)]
struct Root {
    #[serde(rename = "Infogempa")]
    infogempa: Infogempa,
}

fn main() {
    let url = "https://data.bmkg.go.id/DataMKG/TEWS/autogempa.xml";
    let token = env::var("TELEGRAM_TOKEN").expect("TOKEN NOT SET");
    let chat_id = env::var("TELEGRAM_CHAT_ID").expect("CHAT_ID NOT SET");

    let client = Client::new();
    let response = client.get(url).send().unwrap().text().unwrap();

    let decoded: Root = serde_xml_rs::from_str(&response).unwrap();
    let g = decoded.infogempa.gempa;
    let pesan = format!(
        "⚠️ *INFO GEMPA TERBARU* ⚠️\n\n📅 Tanggal: {}\n⏰ Jam: {}\n📉 Mag: {}\n🌊 Kedalaman: {}\n📍 Wilayah: {}\n✨ Potensi: {}",
        g.tanggal, g.jam, g.magnitude, g.kedalaman, g.wilayah, g.potensi
    );
    let tele_url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    client.post(tele_url)
        .form(&[
            ("chat_id", &chat_id), 
            ("text", &pesan), 
            ("parse_mode", &"Markdown".to_string())
        ])
        .send()
        .unwrap();
    
    println!("Notifikasi terkirim!");
}
