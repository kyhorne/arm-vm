pub struct Flag {
    v: bool, // oVerflow.
    z: bool, // Zero.
    n: bool, // Negative.
    c: bool, // Carry.
}

impl Flag {
    pub fn new() -> Flag {
        Flag {
            v: false,
            z: false,
            n: false,
            c: false,
        }
    }
    pub fn update(&mut self, op1: u32, op2: u32) {
        // Update carry flag.
        if let Some(result) = op1.checked_sub(op2) {
            self.c = false;
            // Update zero flag.
            if result == 0 {
                self.z = true;
            } else {
                self.z = false;
            }
            // Always positive under the unsigned interpretation.
            self.n = false;
        } else {
            self.c = true;
        }
        let op1 = op1 as i32;
        let op2 = op2 as i32;
        // Check overflow flag.
        if let Some(result) = op1.checked_sub(op2) {
            self.v = false;
            // Update zero flag.
            if result == 0 {
                self.z = true;
            } else {
                self.z = false;
            }
            // Update negative flag.
            if result < 0 {
                self.n = true;
            } else {
                self.n = false;
            }
        } else {
            self.v = true;
        }
    }
    pub fn get_z(&self) -> bool {
        self.z
    }
    pub fn get_v(&self) -> bool {
        self.v
    }
    pub fn get_n(&self) -> bool {
        self.n
    }
    pub fn get_c(&self) -> bool {
        self.c
    }
}
