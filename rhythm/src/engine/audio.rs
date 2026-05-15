pub struct AudioTrack {
    bpm: u32,
}

impl AudioTrack {
    pub fn new(bpm: u32) -> Self {
        Self {
            bpm,
        }
    }

    // pub fn get_bpm(&self) -> &u32 {
    //     &self.bpm
    // }

    // for primitive types that use the Copy trait, it is faster and more
    // idiomatic to return a copy of the value instead of a reference
    pub fn get_bpm(&self) -> u32 {
        self.bpm // return value directly
    }
}