pub struct HitCircle {

}

impl HitCircle {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn click(&self, track: &super::audio::AudioTrack) {
        println!("{}", track.get_bpm());
    }
}