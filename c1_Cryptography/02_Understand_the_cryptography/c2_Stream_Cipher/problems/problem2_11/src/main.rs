fn main() {
    // Incoming cipher text is `j5a0edj2b`
    // We knwon first three letters is "WPI".
    let well_knwon_header = "WPI";
    let junk_cipher = "j5a0edj2b";

    println!("encoded version of j5a0edj2b is: ");
    for char in junk_cipher.chars() {
        let e = encode(char as u8);
        println!("c: {:?}, encode: {:?}, binary: {:05b}", char, e, e);
    }

    println!("encoded version of Known header 'wpi' is: ");
    for char in well_knwon_header.chars() {
        let e = encode(char as u8);
        println!("c: {:?}, encode: {:?}, binary: {:05b}", char, e, e);
    }
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
   
    if c>= 0 && c<=5 {
        return c + 48
    }

    c + 65
}
