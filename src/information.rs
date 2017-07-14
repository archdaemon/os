use std::process::Command;

#[derive(Debug)]
pub struct OSInfo {
    pub kernel_name: String,
    pub os_name: String,
}

pub fn get_info() -> OSInfo {
    let output = Command::new("uname")
                         .arg("-a")
                         .output()
                         .expect("Failed to get OS info");

    parse_info(String::from_utf8(output.stdout).unwrap())
}

fn parse_info(string: String) -> OSInfo {
    let info: Vec<&str> = string.trim().split(" ").collect();
    
    OSInfo {
        kernel_name: info.first().unwrap().to_string(),
        os_name: info.last().unwrap().to_string(),
    }
}
