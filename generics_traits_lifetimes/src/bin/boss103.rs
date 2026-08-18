fn main() {
    let transmission = "hello_world";
    let top_secret_dossier: Dossier;

    {
        let one_time_pad = String::from("bite of 87");
        top_secret_dossier = extract(transmission, &one_time_pad);
    } // Scope of one_time_pad ends

    println!("{:?}", top_secret_dossier.encrypted_transmission);
}

struct Dossier<'a> {
    encrypted_transmission: &'a str, // Reference to original document
}

fn extract<'a, 'b>(encrypted_transmission: &'a str, one_time_pad: &'b str) -> Dossier<'a> { // Dossier<'a>, not &'a Dossier
    // 'a and 'b as it is not related to Dossier
    println!("One-time-pad: {}", one_time_pad);
    Dossier {
        encrypted_transmission
    }
}