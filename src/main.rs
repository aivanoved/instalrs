use std::io::{self, Write};
use std::process::Command;

trait PackageManager {
    // String prefix_path;
    // String install_command;
    fn install_package(&self, package_name: &str, install_flags: Option<&str>) -> Result<(), &str>;
}

struct Apt {}

impl PackageManager for Apt {
    // String prefix_path = String::new("");
    // String install_command = String::new("apt-get install");

    #[allow(unused_variables)]
    fn install_package(&self, package_name: &str, install_flags: Option<&str>) -> Result<(), &str> {
        let output = Command::new("sudo")
            .arg("apt")
            .arg("install")
            .args(install_flags.unwrap_or("").split(' ').collect::<Vec<_>>())
            .output();
        #[allow(unused_must_use)]
        match output {
            Ok(output) => {
                io::stdout().write(&output.stdout);
                Ok(())
            }
            Err(e) => {
                println!("error has occurred");
                println!("{:#?}", e);
                Err("error")
            }
        }
    }
}

#[allow(unused_must_use)]
fn main() {
    let apt = Apt {};
    apt.install_package("zsh", None);
}
