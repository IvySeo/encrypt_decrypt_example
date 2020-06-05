
/*
Cryptography is used to secure and protect data during communication. 
It is helpful to prevent unauthorized person or group of users from accessing any confidential data. 

Encryption: a process which transforms the original information into an unrecognizable form.
Decryption: a process of converting encoded/encrypted data in a form that is readable and understood by a human or a computer. 
Public key: an encryption system which is based on two pairs of keys. Public keys are used to encrypt messages for a receiver.
Sealed box: designed to anonymously send messages to a recipient given its public key.
Only the recipient can decrypt these messages, using its private key. 

-- how to run --
- set up Rust
- to start the program to encrypt/decrypt: simply type 'cargo run' in a terminal
*/

mod encrypt;
mod decrypt;

use std::fs::File;
use std::io::{BufRead, BufReader};

use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305 as box_;


    fn main() {
        println!("main start");

        // start encrypting - the function located in encrypt.rs mod
       encrypt_start();

    }

    pub fn encrypt_start() {

// ------------------------------------------ PUBLIC KEY ------------------------------------------------------------------------

            let (pk, sk) = box_::gen_keypair();

// ------------------------------------------ MESSAGE TO ENCRYPT : type u8 in this example --------------------------------------

            let message: &[u8] = &[221];
            println!("message to encrypt: {:?}", message);

// ------------------------------------------ ENCRYPT IT ------------------------------------------------------------------------

            let encrypted = encrypt::encrypt(&message, &pk);
            println!("encrypted: {:?}", encrypted);

// ------------------------------------------ DECRYPT IT ------------------------------------------------------------------------

            let opened = decrypt::decrypt(&encrypted, &pk, &sk);
            println!("decrypted: {:?}", opened);

// ------------------------------------------ DECRYPT FROM THE FILE ------------------------------------------------------------------------
            /*         
            // open the encrypted file
            let filename_to_decrypt = "./example_encrypted_file_to_decrypt";
                    
            let encrypted_file = File::open(filename_to_decrypt).unwrap();
            let reader_ = BufReader::new(encrypted_file);

            println!("decrypt 1");
            // read encrypted text
            for (index_, encrypted_text) in reader_.lines().enumerate() {
                println!("decrypt 2");

                let encrypted_text = encrypted_text.unwrap(); 
                println!("decrypt 3");

                // decrypte it
                let decrypted = decrypt::decrypt(&encrypted_text, &pk, &sk);
                println!("decrypt 4 - done decrypted: {:?}", decrypted);

            }
            */

        
    }


