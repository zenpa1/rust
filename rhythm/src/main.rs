use rhythm::prelude::*;

fn main() {
    let track = AudioTrack::new(120); // not .new(), as it is not a real
    // object in memory yet, but rather a blueprint/concept
    let circle = HitCircle::new();
    // look inside HitCircle blueprint and run the new function
    // . opertor is for instances (interacting with objects in memory)

    circle.click(&track);
}
