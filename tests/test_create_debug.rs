use serde_json::json;

fn main() {
    // Test the JSON structure for MsgInstantiateContract2
    let msg_value = json!({
        "sender": "xion1test",
        "code_id": 1260,
        "label": "Treasury-test",
        "msg": "base64encoded",
        "salt": "base64salt",
        "funds": [],
        "admin": "xion1test",
    });

    println!("MsgInstantiateContract2 JSON:");
    println!("{}", serde_json::to_string_pretty(&msg_value).unwrap());
    println!("\nField names (camelCase):");
    for key in msg_value.as_object().unwrap().keys() {
        println!("  - {}", key);
    }
}
