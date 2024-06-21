// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod aes;
use std::string;
use std::vec;

use aes::AESCipher;
use aes::AESKey;
use anyhow::Result;

#[tauri::command]
fn aes_verschluesseln(klartext: String, key: String) -> String {
    let mut text_bytes = klartext.as_bytes().to_vec();
    let mut key_bytes = key.as_bytes().to_vec();
    let len = text_bytes.len();

    if len % 16 != 0 {
        text_bytes.resize(len + (16 - len % 16), 0);
    }
    key_bytes.resize(16, 0);

    let mut cipher = AESCipher::new(AESKey::Key128(key_bytes.try_into().unwrap()));
    let encrypted = cipher.encrypt(text_bytes.clone());

    println!("klartext {:?}", hex::encode(text_bytes));

    return hex::encode(encrypted).to_string()
}

#[tauri::command]
fn aes_entschluesseln(geheimtext: String, key: String) -> Result<String> {
    println!("hallo {:?}", geheimtext);
    let mut text_bytes = hex::decode(geheimtext)?;
    println!("hallo 2{:?}", text_bytes);
    let mut key_bytes = key.as_bytes().to_vec();
    let len = text_bytes.len();

    if len % 16 != 0 {
        text_bytes.resize(len + (16 - len % 16), 0);
    }

    key_bytes.resize(16, 0);

    let mut cipher = AESCipher::new(AESKey::Key128(key_bytes.try_into().unwrap_or_default()));
    let mut decrypted = cipher.decrypt(text_bytes);

    while decrypted.last() == Some(&0) {
        decrypted.pop();
    }

    println!("skibidi {:?}", hex::encode(decrypted.clone()));

    return Ok(String::from_utf8_lossy(&decrypted).to_string())
}

fn main() {

    //let bytes: [u8; 16] = [0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xa0, 0xb0, 0xc0, 0xd0, 0xe0, 0xf0, 0x00];
    // 2b7e151628aed2a6abf7158809cf4f3c
    //let key: [u8; 24] = [0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    //let mut cipher = AESCipher::new(AESKey::Key192(key));

    // let encrypted = cipher.encrypt(bytes.to_vec());

    // println!("{:?}", bytes);
    // println!("{:?}", encrypted);

    // let decrypted = cipher.decrypt(encrypted);

    // println!("{:?}", decrypted);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            aes_verschluesseln,
            aes_entschluesseln,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}