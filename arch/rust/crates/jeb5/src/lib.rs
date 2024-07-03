pub fn hello_world() {
    println!("Hello world :)")
}

pub fn hello_jeb5() -> String {
  "hello jeb5".to_string()
}

uniffi::include_scaffolding!("jeb5");
