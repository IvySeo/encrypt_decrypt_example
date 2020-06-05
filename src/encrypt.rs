
// ============================================= ENCRYPT  =============================================
#![crate_name = "sodiumoxide"]
#![crate_type = "lib"]
extern crate libsodium_sys as ffi;
use std::fs::File;
use sodiumoxide::crypto::{
    box_::{curve25519xsalsa20poly1305::PublicKey},
    sealedbox,
};
use uuid::Uuid; // Provides support for Universally Unique Identifiers (UUIDs)
use std::io::{BufWriter, Write};
extern crate std;
#[cfg(not(feature = "std"))]
use libc::c_ulonglong;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305 as box_;

pub const SEALBYTES: usize = ffi::crypto_box_SEALBYTES as usize;



extern crate libc;

// ------------------------------------------ ENCRYPT FUNCTION ------------------------------------------------------------------------
        
   // pub fn encrypt(value: &str, &box_::PublicKey(ref pk): &box_::PublicKey) -> Vec<u8> {
    pub fn encrypt (value: &[u8], &box_::PublicKey(ref pk): &box_::PublicKey) -> Vec<u8> {
            println!("value: {:?}\n public key: {:?}", value, pk);
    
            let mut c = vec![0u8; value.len() + SEALBYTES];
            unsafe {
                let sealed = ffi::crypto_box_seal(c.as_mut_ptr(),
                value.as_ptr(), value.len() as c_ulonglong,
                                     pk.as_ptr() as *const u8);

                println!("sealed: {:?}", sealed);
            }
            let encrypted_output = c.clone();
            write_file(encrypted_output);
            
            c
        }
//----------------------- WRITE A FILE ------------------------------------------------------------------------

    pub fn write_file(encrypted:  Vec<u8>){
            // UUID : unique file name
            let my_uuid = Uuid::new_v4();
            let file_created = File::create(my_uuid.to_string()).expect("create file failed");
        
            // buffer write
            let mut file_generated = BufWriter::new(file_created);
        
            //  write file
            file_generated.write_all(format!("{:?}", encrypted).as_bytes()).expect("write failed");
    
            println!("encrypted txt file generated. uuid : {}", my_uuid);
    
    }
