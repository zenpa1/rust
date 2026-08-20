fn main() {
    let artifact1 = Artifact {
        fragment: "Divinity Scroll",
        weight: 5,
    };

    let artifact2 = Artifact {
        fragment: "Nexus Coin",
        weight: 500,
    };

    arbiter(&artifact1.fragment, &artifact2.fragment, "Announcement!");
}

struct Artifact<'a> {
    fragment: &'a str, // Borrowed text field
    weight: u32,
}

trait Weight {
    // fn compare_weight(&self, competitor: &Artifact) -> u32; // Requires the return type if we plan to return something

    fn get_weight(&self) -> u32; // Required method
}

impl<'a> Weight for Artifact<'a> {
    fn get_weight(&self) -> u32 {
        self.weight
    }
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
// Utilize generics to ensure that the same type is used even if separate types both implement Weight
fn arbiter<'a, 'b, T: Weight>(x: &'a T, y: &'a T, ann: &'b str) -> &'a T {
    // Print the announcement
    println!("{}", ann);

    // Compare weights and return the reference to the heavier entity
    if x.get_weight() > y.get_weight() {
        x // x is already a reference, so &x is a reference to a reference
    } else {
        y
    }
}
