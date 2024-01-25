fn main() {    
   let cipher_text = encryption("kaspar hausep", "rsidpy dkawoy");
   println!("Cipher text: {:?}", cipher_text);

   let plain_text = decrpytion("bsaspp kkuosp", "rsidpy dkawoy");
   println!("Plain text: {:?}", plain_text);
}

fn encryption(plain_text: &str, key: &str) -> Option<String> {
    if plain_text.len() == key.len() {
        let mut cipher_text = String::new();
        
        for (index, c) in plain_text.char_indices() {

            if c as u8 != 32 { 
                let char_map = encode(c as u8);
                let key_map = encode(key.bytes().nth(index).unwrap() as u8);
            
                cipher_text.push(
                    decode((char_map + key_map) % 26) as char
                );
            }
            else {
                cipher_text.push(' ');
            }
        }
        return Some(cipher_text);
    }
    
    None
}

fn decrpytion(cipher_text: &str, key: &str) -> Option<String> 
{
    if cipher_text.len() == key.len() {
        let mut plain_text = String::new();
        
        for (index, c) in cipher_text.char_indices() {
            if c as u8 != 32 { 
                let char_map = encode(c as u8);
                let key_map = encode(key.bytes().nth(index).unwrap() as u8);

                let mut plain_code = char_map as i8 - key_map as i8;
                while plain_code < 0 {
                    plain_code += 26
                }
                
                plain_text.push(decode(plain_code as u8) as char);
            }
            else{
                plain_text.push(' ');
            }
        }
        return Some(plain_text);
    }
    
    None 
}

fn encode(c: u8) -> u8 {
   
    if c >= 97 {
        return (c - (('A' as u8) + 6)) % 26;
    }

    let ans = (c - ('A' as u8)) % 26;
    ans
}

fn decode(c: u8) -> u8 {
    c + 97
}