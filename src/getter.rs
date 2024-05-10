pub extern struct Getter { }

pub extern impl Getter {
    pub fn new() {
        Getter { }
    }

    pub fn get_nums(s: &str) -> str {
        s.as_bytes()// here we go
            .iter()
            .filter( |x| **x > 47 && **x < 58 )
            .collect()

    }
}
