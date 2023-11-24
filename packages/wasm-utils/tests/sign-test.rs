use wasm_utils::signature::SignatureHandler;

#[test]
fn test_aes() {
    let str = "connecting...";
    let encrypt = SignatureHandler::encrypt(str);
    let decrypt = SignatureHandler::decrypt(&encrypt).unwrap();
    assert_eq!(str, decrypt);
}

#[test]
fn test_base64() {
    let str = "{'name':'BeJson','url':'http://www.bejson.com','page':88,'isNonProfit':true,'address':{'street':'科技园路.','city':'江苏苏州','country':'中国'},'links':[{'name':'Google','url':'http://www.google.com'},{'name':'Baidu','url':'http://www.baidu.com'},{'name':'SoSo','url':'http://www.SoSo.com'}]}";
    let encode = SignatureHandler::encode(str);
    let decode = SignatureHandler::decode(&encode).unwrap();
    assert_eq!(str, decode);
}
