// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::{Value, json};
use std::fs;
use std::io::Write;

#[tauri::command]
fn command_with_message(filepath: String) -> Result<(), String>{
    let file_content = match fs::read_to_string(&filepath) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read the file: {}", e); 
            return Err("Failed to open vpp file".to_string())
        }
    };
    // 終端NULL文字削除、しないとjsonパースできない
    let file_content_removed_nul = file_content.trim_end_matches('\0');

    let parsed_json: Value = match serde_json::from_str(&file_content_removed_nul) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to parse json: {}", e); 
            return Err("Failed to load vpp content".to_string())
        }
    };
    
    //1トークン目に代入するやつを作成
    let tokens = parsed_json["project"]["blocks"][0]["sentence-list"][0]["tokens"]
        .as_array().unwrap();
    let tokens_len = tokens.len();
    let mut syl = vec![]; 
    for token in tokens {
        let token_len = token["syl"].as_array().unwrap().len();
        if token_len != 0 {
            for i in 0..=token_len - 1{
                syl.extend(token["syl"][i].as_object());
            }
        }
    }

    //1トークン目にさっき作ったのを代入
    //pe(token編集したかどうかのparam)をtrueに
    let mut result_json = parsed_json.clone();
    for index in 0..=tokens_len - 1{
        //ここらへんもうちょっと見やすくできないのかな、Rustわからんからわからん
        if index == 0{
            result_json["project"]["blocks"][0]["sentence-list"][0]["tokens"][index]["syl"]
            = Value::Array(syl.clone().into_iter().map(|x| json!(x)).collect());
        }else{
            result_json["project"]["blocks"][0]["sentence-list"][0]["tokens"][index]["syl"] 
            = json!([])
        }
        result_json["project"]["blocks"][0]["sentence-list"][0]["tokens"][index]["pe"]
            = json!(true)
    }

    //終端NUL文字つけて日本語Unicode変換して上書き出力
    let mut json_output_data = serde_json::to_string(&result_json).unwrap();
    json_output_data.push_str("\0");
    let json_output_data = to_unicode_escape(&json_output_data);

    let mut file = match fs::File::create(&filepath) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to save vpp: {}", e); 
            return Err("Failed to save vpp".to_string())
        }
    };
    let _ = file.write_all(json_output_data.as_bytes());

    Ok(())
}

fn to_unicode_escape(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii() {
                c.to_string()
            } else {
                format!("\\u{:04x}", c as u32) // 日本語はUnicodeエスケープ形式に変換
            }
        })
        .collect()
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![command_with_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
