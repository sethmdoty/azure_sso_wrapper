extern crate clap;

use clap::{Arg, App};
use std::process::{Command};
use std::env;

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
        let _install_azure = Command::new("npm").arg("install").arg("-g").arg("aws-azure-login").status().expect("process failed");
        assert!(_install_azure.success());
        println!("aws-azure-login was installed");
    }
    if matches.is_present("CONFIGURE") {
        let _configure_azure = println!("Configuring Sandbox");
        let tenant = "AZURE_TENANT_ID";
        let app_id = "AZURE_APP_ID_URI";
        env::set_var(tenant, "some value");
        env::set_var(app_id, "some value");
        assert_eq!(env::var(tenant), Ok("some value".to_string()));
        assert_eq!(env::var(app_id), Ok("some value".to_string()));
        let _configure_environment = Command::new("aws-azure-login").arg("--profile").arg("sandbox").status().expect("command failed");
    }
}