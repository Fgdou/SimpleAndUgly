pub trait ApplicationRepo {

}

pub struct ApplicationRepoMemory {

}

impl ApplicationRepoMemory {
    pub fn new() -> Self {
        Self{}
    }
}

impl ApplicationRepo for ApplicationRepoMemory {

}