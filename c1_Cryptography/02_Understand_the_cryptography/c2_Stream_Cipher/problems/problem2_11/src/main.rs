use std::ops::{BitAnd, BitOr, BitXor};

fn main() {
    // Incoming cipher text is `j5a0edj2b`
    // We knwon first three letters is "WPI".
    let well_knwon_header = "WPI";
    let junk_cipher = "j5a0edj2b";

    let mut encoded_header:Vec<u8> = Vec::new(); 
    let mut encoded_wk: Vec<u8> = Vec::new();

    println!("encoded version of j5a0edj2b is: ");
    for char in junk_cipher.chars() {
        let e = encode(char as u8);
        println!("c: {:?}, encode: {:?}, binary: {:05b}", char, e, e);

        encoded_wk.push(e);
    }

    println!("encoded version of Known header 'wpi' is: ");
    for char in well_knwon_header.chars() {
        let e = encode(char as u8);
        println!("c: {:?}, encode: {:?}, binary: {:05b}", char, e, e);

        encoded_header.push(e);
    }
    println!("===================================");

    println!("Step1: XOR with plaintext and ciphertext to get keystream ");
    for (i, encoded_c) in encoded_header.into_iter().enumerate() {
        print!("{:05b}", encoded_c.bitxor(encoded_wk.get(i).unwrap()) );
    }
    println!("");
    println!("===================================");

    println!("Step3: Construct a LFSRs.");

    // We've to decrypt "j5a0edj2b" which each take 5 bits as a block -
    // total 45 bits size are need.
    let s_i = keygen(0b111111, 45, 6);
    println!("Generated key s_i: {:045b}", s_i);

    println!("===================================");
    println!("Step4: Decrypt cipher text by using generated key stream");
    let plain_text = decryption(s_i, 45, "j5a0edj2b");

    println!("Incoming cipher text on network: j5a0edi2b");
    println!("plain text: {:?}", plain_text);
}

fn keygen(mut init_vector: u8, keysize: u8, m_degree: u8) -> u64{
    let loop_count = (keysize - m_degree) as u64;

    let mut output_s_i: u64 = init_vector as u64;

    let mask = 0b000001;

    // coefficient of LFRSs
    let p_0 = 0b1;
    let p_1 = 0b1;

    let p_2 = 0b0;
    let p_3 = 0b0;
    let p_4 = 0b0;
    let p_5 = 0b0;

    for _i in 0..loop_count {
        let s_i = (
            p_0 * (init_vector.bitand(mask)) + 
            p_1 * (init_vector.bitand(mask << 1) >> 1) + 
            p_2 * (init_vector.bitand(mask << 2) >> 2) +
            p_3 * (init_vector.bitand(mask << 3) >> 3) +
            p_4 * (init_vector.bitand(mask << 4) >> 4) +
            p_5 * (init_vector.bitand(mask << 5) >> 5)
        ) % 2;

        let next_s_i = (s_i).bitand(mask);
        init_vector = (init_vector >> 1).bitor(next_s_i << 5);

        output_s_i = (output_s_i << 1).bitor(s_i as u64);
    }

    output_s_i
}

fn encode(c: u8) -> u8 {
    if c >= 97 {
        return ((c - 32) -65) % 32;
    }

    if c>= 48 && c <= 53 {
        let gap = c - 22;
        return gap % 32;
    }

    let ans = (c - 65) % 32;
    ans
}

fn decode(c: u8) -> u8 {
    c + 65
}

fn decryption(s_i: u64, keysize: u8, cipher_text: &str) -> String {

    let block_size = keysize - 5; // 40
    let mask = 0b11111;

    let mut plain_text = String::new();

    for (index, x) in cipher_text.char_indices() {

        // Calculate the motion to capture the s_i 5-bits block value.
        let motion = (block_size - ((index as u8 )* 5)) as u64;

        let y_i = 
            encode(x as u8).bitxor(
                (s_i.bitand(mask << motion) >> motion) as u8
            );
        
        plain_text.push(decode(y_i as u8) as char);
    }   

    plain_text
}