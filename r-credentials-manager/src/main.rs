use database::{Credential, Database};
use rusqlite::Connection;
use utils::{compare_hashes, hash_master, vigenere_decrypt, vigenere_encrypt};

mod database;
mod utils;

fn main() {
    // Establish a connection to the SQLite database
    let conn = Connection::open("test.db");
    let db = Database::setup(conn.unwrap());

    // Create the table if it doesn't exist
    db.create_if_not_exist();

    // Store a master password hash
    let master_password = "my_secure_password".to_string();

    println!(
        "stored {}",
        db.store_master_hash(master_password.clone()).unwrap()
    );
    // Check if the stored master password hash matches the input
    let is_correct = db.check_master(master_password);
    if is_correct.unwrap() {
        println!("Master password is correct!");
    } else {
        println!("Master password is incorrect.");
    }

    // Insert some credentials
    let credential = Credential {
        id: String::new(), // The ID will be auto-generated
        username: "user1".to_string(),
        usage_desc: "Email".to_string(),
        password: "secret_password".to_string(),
    };
    let encryption_keyword = "encryption_key".to_string();
    db.insert_creds(credential, encryption_keyword.clone());

    // Retrieve and print all credentials
    let credentials_encrypted = db.select_all_creds_usage_desc().unwrap();
    for cred in credentials_encrypted {
        println!("ID: {}", cred.id);
        println!("Usage: {}", cred.usage_desc);
        println!("Username: {}", cred.username);
        println!("Password: {}", cred.password);
    }
    println!("-----------------");
    let credentials = db
        .select_all_creds_all_info(encryption_keyword.clone())
        .unwrap();
    for cred in credentials {
        println!("ID: {}", cred.id);
        println!("Usage: {}", cred.usage_desc);
        println!("Username: {}", cred.username);
        println!("Password: {}", cred.password);
    }
}

fn test_utils() {
    // Test hashing and comparing hashes
    let master_password = String::from("my_secret_master");
    let stored_hash = hash_master(&master_password.clone());
    println!("Stored Hash: {}", stored_hash);

    let input_password = String::from("my_secret_master");
    let input_hash = hash_master(&input_password.clone());
    println!("Input Hash: {}", input_hash);

    if compare_hashes(stored_hash.clone(), input_hash) {
        println!("Master password is correct!");
    } else {
        println!("Master password is incorrect!");
    }

    // Test key derivation

    // Test encryption
    let password_to_encrypt = String::from("my_password12345");
    let encrypted_password = vigenere_encrypt(&password_to_encrypt.clone(), &master_password);
    println!("Encrypted password: {:?}", encrypted_password);

    // Test decryption
    let decrypted_password = vigenere_decrypt(&encrypted_password, &master_password);
    println!("Decrypted password: {}", decrypted_password);
}
