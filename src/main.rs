
fn main() {
    let k = "Teste de Hash";
    let h = hash(k);
    println!("Hash: {}", h);
}

fn hash(key: &str) -> u32 {
    let mut hash: u32 = 0;
    for c in key.chars() {
        hash += c as u32;
    }
    hash
}

