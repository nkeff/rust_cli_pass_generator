use sha2::{Sha256, Digest};


pub fn gen_pass(login: &String, master_pass: &String, salt: &String) -> String {
    let pass_length = 16;
    let allower_chars = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        '!', '#', '$', '%', '&', '(', ')', '*', '+', '-', ';', '<', '=',
        '>', '?', '@', '^', '_', '`', '{', '|', '}', '~'
    ];
    let mut result: String;

    let mut creds_str: String = String::from(login);
    creds_str.push_str(&master_pass);
    creds_str.push_str(&salt);
    let mut sha256 = Sha256::new();
    sha256.update(creds_str);
    let hash: String = format!("{:X}", sha256.finalize());
    
    for i in (1..pass_length) {
        result.push_str(allower_chars[hash[i].chars()])
    }
    return hash;
}