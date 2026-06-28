pub struct SGCFLIndex {
    sg_to_cfl_index: Vec<u32>,
    cfl_to_sg_index: Vec<u32>,
}

impl SGCFLIndex {
    pub fn new() -> Self {
        Self {
            sg_to_cfl_index: Vec::new(),
            cfl_to_sg_index: Vec::new(),
        }
    }

    pub fn with_capacity(sg_capacity: usize, cfl_capacity: usize) -> Self {
        Self {
            sg_to_cfl_index: Vec::with_capacity(sg_capacity),
            cfl_to_sg_index: Vec::with_capacity(cfl_capacity),
        }
    }

    pub fn sg_to_cfl(&self, sg: u32) -> Option<u32> {
        if sg >= self.sg_to_cfl_index.len() as u32 {
            None
        } else {
            let value = self.sg_to_cfl_index[sg as usize];
            if value == u32::MAX {
                None
            } else {
                Some(value)
            }
        }
    }

    pub fn cfl_to_sg(&self, cfl: u32) -> Option<u32> {
        if cfl >= self.cfl_to_sg_index.len() as u32 {
            None
        } else {
            let value = self.cfl_to_sg_index[cfl as usize];
            if value == u32::MAX {
                None
            } else {
                Some(value)
            }
        }
    }

    pub fn add(&mut self, sg: u32, cfl: u32) {
        if sg < self.sg_to_cfl_index.len() as u32 {
            self.sg_to_cfl_index[sg as usize] = cfl;
        } else {
            self.sg_to_cfl_index.resize(sg as usize + 1, u32::MAX);
            self.sg_to_cfl_index[sg as usize] = cfl;
        }
        if cfl < self.cfl_to_sg_index.len() as u32 {
            self.cfl_to_sg_index[cfl as usize] = sg;
        } else {
            self.cfl_to_sg_index.resize(cfl as usize + 1, u32::MAX);
            self.cfl_to_sg_index[cfl as usize] = sg;
        }
    }
}
