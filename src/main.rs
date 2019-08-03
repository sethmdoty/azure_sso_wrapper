extern crate clap;

use clap::{Arg, App};
use std::process::{Command};

fn main() {
    let matches = App::new("azure_sso")
        .version("0.1.0")
        .author("Seth Doty")
        .about("Look mom, I'm a wrapper")
        .arg(Arg::with_name("CONFIGURE")
            .short("c")
            .long("configure")
            .help("set up azure_sso")
        )
        .arg(Arg::with_name("INSTALL")
            .short("i")
            .long("install")
            .help("install the sso binary")
        )
        .get_matches();

    if matches.is_present("INSTALL") {
        let _install_azure = Command::new("npm").arg("install").arg("-g").arg("aws-azure-login").output().expect("command failed");
        println!("Installed");
    }
    if matches.is_present("CONFIGURE") {
        let _configure_azure = println!("Lets configure some things");
    }
}