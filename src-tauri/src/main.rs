// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod aes;
use aes::AESCipher;
use aes::AESKey;
// #[tauri::command]
// fn aes_verschluesseln(text: String, schluessel: String) -> String {
//     todo!()
// }

fn main() {

    let bytes: [u8; 16] = [0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xa0, 0xb0, 0xc0, 0xd0, 0xe0, 0xf0, 0x00];
    // 2b7e151628aed2a6abf7158809cf4f3c
    let key: [u8; 16] = [0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c];
    let mut cipher = AESCipher::new(AESKey::Key128(key));

    let encrypted = cipher.encrypt(bytes.to_vec());

    println!("{:?}", bytes);
    println!("{:?}", encrypted);

    let decrypted = cipher.decrypt(encrypted);

    println!("{:?}", decrypted);
    // tauri::Builder::default()
    //     .invoke_handler(tauri::generate_handler![
    //         // add,
    //     ])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}

