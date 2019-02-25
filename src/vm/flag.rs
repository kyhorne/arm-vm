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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_update() {
        let mut flag = Flag::new();
        flag.update(0x2C000000, 0xD2FFFFFF);
        assert!(!flag.get_c() && !flag.get_z() && !flag.get_n() && !flag.get_v());
    }

    #[test]
    fn test_update_with_carry() {
        let mut flag = Flag::new();
        flag.update(0xD9000000, 0xA3FFFFFF);
        assert!(flag.c && !flag.z && !flag.n && !flag.v);
    }

    #[test]
    fn test_update_with_overflow() {
        let mut flag = Flag::new();
        flag.update(0x68000000, 0xD2FFFFFF);
        assert!(!flag.c && !flag.z && !flag.n && flag.v);
    }

    #[test]
    fn test_update_with_negative() {
        let mut flag = Flag::new();
        flag.update(0xB5000000, 0xC4FFFFFF);
        assert!(!flag.c && !flag.z && flag.n && !flag.v);
    }

    #[test]
    fn test_update_with_zero() {
        let mut flag = Flag::new();
        flag.update(0x00000000, 0xFFFFFFFF);
        assert!(!flag.c && flag.z && !flag.n && !flag.v);
    }

}
