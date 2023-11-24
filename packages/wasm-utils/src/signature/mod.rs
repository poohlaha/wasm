/*!
  签名、加密帮忙类
*/
use crate::error::WasmError;
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes256;
use base64::{engine::general_purpose, Engine};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SignatureHandler;

const KEY: &str = "gXL3ZCFwaHMgfV$sn@pks8COlhFP08#K";

// const IV: &str = "RR0a#Be@KfbyGzwX";

#[wasm_bindgen]
impl SignatureHandler {
    /**
       AES 加密, 块大小通常是 16 字节（128 位）
    */
    pub fn encrypt(data: &str) -> String {
        let key = GenericArray::from_slice(KEY.as_bytes());

        let mut block = vec![0u8; data.len() + 16]; // 假设每个块大小是 16 字节
        block[..data.len()].copy_from_slice(data.as_bytes());

        let mut block_array = GenericArray::from_mut_slice(&mut block[..16]);
        let cipher = Aes256::new(&key);
        cipher.encrypt_block(&mut block_array);
        general_purpose::STANDARD.encode(&block_array)
    }

    /**
     AES 解密
    */
    pub fn decrypt(data: &str) -> Result<String, JsValue> {
        let data = general_purpose::STANDARD
            .decode(&data)
            .map_err(|err| JsValue::from_str(&WasmError::Error(err.to_string()).to_string()))?;
        let key = GenericArray::from_slice(KEY.as_bytes());
        let mut block = GenericArray::default();
        block[..data.len()].copy_from_slice(&data);

        let cipher = Aes256::new(&key);
        cipher.decrypt_block(&mut block);
        let str = general_purpose::STANDARD.encode(&block);
        Self::decode(&str)
    }

    /**
     base64 encode
    */
    pub fn encode(data: &str) -> String {
        general_purpose::STANDARD.encode(&data)
    }

    /**
      base64 decode
    */
    pub fn decode(data: &str) -> Result<String, JsValue> {
        let data = data.trim();
        // 检查 Base64 编码的长度是否是 4 的倍数
        if data.len() % 4 != 0 {
            return Err(JsValue::from_str("Invalid Base64 encoded data"));
        }

        // 检查 Base64 编码字符串是否包含非法字符
        if data
            .chars()
            .any(|c| !(c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '='))
        {
            return Err(JsValue::from_str("Invalid Base64 encoded data"));
        }

        let mut str = general_purpose::STANDARD
            .decode(&data)
            .map_err(|err| JsValue::from_str(&WasmError::Error(err.to_string()).to_string()))?;
        str.retain(|&c| c != 0); // 去除 \0 填充
        Ok(String::from_utf8(str).unwrap_or(String::new()))
    }
}
