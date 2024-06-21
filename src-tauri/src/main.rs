// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod aes;
use std::vec;

use aes::AESCipher;
use aes::AESKey;



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

    return hex::encode(encrypted).to_string()
}




#[tauri::command]
fn aes_entschluesseln(geheimtext: String, key: String) -> String {

    let mut text_bytes = match hex::decode(geheimtext) {
        Ok(bytes) => bytes,
        Err(_) => return "FEHLER".to_string(),
    };

    let mut key_bytes = key.as_bytes().to_vec();
    
    let len = text_bytes.len();

    if len % 16 != 0 {
        text_bytes.resize(len + (16 - len % 16), 0);
    }

    key_bytes.resize(16, 0);

    let mut cipher = AESCipher::new(AESKey::Key128(key_bytes.try_into().unwrap_or_default()));

    let mut decrypted = cipher.decrypt(text_bytes);

    // entfernt Nullbytes (Nullen) am Ende des Strings
    while decrypted.last() == Some(&0) {
        decrypted.pop();
    }

    return String::from_utf8_lossy(&decrypted).to_string()
}





fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            aes_verschluesseln,
            aes_entschluesseln,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}