use std::path::PathBuf;

pub fn find_requirements_dir() -> String {
    let mut current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    println!("Current directory: {:?}", current_dir);

    current_dir.as_mut_os_string()
        .to_str()
        .unwrap_or("")
        .to_string()
        + "/requirements"
}