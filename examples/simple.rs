use mac_address::get_mac_address;

fn main() {
    match get_mac_address() {
        Ok(ls) => {
            if ls.len() <= 0 {
                println!("No MAC address found.");
            }
            for ma in &ls {
                println!("MAC addr = {}", ma);
                println!("bytes = {:?}", ma.bytes());
            }
        }
        Err(e) => println!("{:?}", e),
    }
}
