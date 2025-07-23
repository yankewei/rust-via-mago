use mago_php_version::PHPVersion;

fn main() {
    // let version = PHPVersion::new(8, 4, 0);
    let version: PHPVersion = "8.4.0".parse().unwrap();
    let json = serde_json::to_string(&version).unwrap();
    println!("{}", json);
}
