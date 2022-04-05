// second_module.rs
pub fn message() -> String {
    String::from("This is the 2nd module.")
}

pub mod movies {
    pub fn play(name:String) {
        println!("Playing movie {}",name);
    }
}
