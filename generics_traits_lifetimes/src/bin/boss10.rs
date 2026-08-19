fn main() {
}

struct Artifact<'a> {
    fragment: &'a str, // Borrowed text field
    weight: u32,
}

trait Weight {
    // fn compare_weight(&self, competitor: &Artifact) -> u32; // Requires the return type if we plan to return something

    fn get_weight(&self) -> u32; // Required method
}

// // Approach 1: Implement the Weight trait on Artifact
// // Issue with approach: what if another struct implemented Weight?
// // A lifetime is part of a struct's type
// // There exists only Artifact<'a>, not Artifact, so annotate accordingly
// impl<'a> Weight for Artifact<'a> {
//     // If we do have a return type, ensure it exists in the blueprint as well
//     fn compare_weight(&self, competitor: &Artifact) -> u32 {
//         if self.weight > competitor.weight {
//             self.weight
//         } else {``
//             competitor.weight
//         }
//     }
// }

// Approach 2: Implement the comparison method in general
// This way, anything that implements Weight can just return their weight
fn arbiter<'b>(x: &impl Weight, y: &impl Weight, ann: &'b str) -> &impl Weight {
    // Print the announcement
    println!("{}", ann);

    // Compare weights and return the reference to the heavier entity
    if x.get_weight() > y.get_weight() {
        &x
    } else {
        &y
    }
}
