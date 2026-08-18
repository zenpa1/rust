fn main() {
    let transmission = "hello_world";
    let top_secret_dossier: &str;

    {
        let one_time_pad = "bite of 87";
        top_secret_dossier = extract(transmission, one_time_pad);
    } // Scope of one_time_pad ends

    println!("{}", top_secret_dossier);
}

struct Dossier<'a> {
    encrypted_transmission: &'a str, // Reference to original document
}

fn extract<'a, 'b>(encrypted_transmission: &'a str, one_time_pad: &'b str) -> &'a str {
    // 'a and 'b as it is not related to Dossier
    // First elision rule infers 'a from &self
    // Third elision rule infers 'a as output lifetime due to &self
    println!("One-time-pad: {}", one_time_pad);
    encrypted_transmission
}