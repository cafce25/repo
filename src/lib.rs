#[cfg(feature = "x")]
pub mod util {
    pub fn x() {
        println!("x");
    }
}

#[cfg(feature = "y")]
pub mod util2 {
    pub fn y() {
        println!("y");
    }
}
