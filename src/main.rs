use std::io;
use std::io::Write;
use std::os::unix::process::CommandExt;
use std::process::Command;
use pam::Authenticator;
use users::User;
mod input;

// A simple program that requests a login and a password and then spawns /bin/bash as the
// user who logged in.
//
// Note that this proto-"sudo" is very insecure and should not be used in any production setup,
// it is just an example to show how the PAM api works.

#[derive(Debug, Clone)]
// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
enum LoginError {
    AuthFailed,
    NoUser,
}



fn login(username: String, password: String) -> Result<User, LoginError> {
    use LoginError::*;

    let user = match users::get_user_by_name(&username) {
        Some(u) => u,
        None => return Err(NoUser),
    };

    // Now, setup the authenticator, we require the basic "login" service
    let mut authenticator = 
        Authenticator::with_password("login").expect("Failed to init PAM client!");

    authenticator.get_handler().set_credentials(username, password);

    if let Err(_) = authenticator.authenticate() {
        return Err(AuthFailed);
    }

    authenticator.close_on_drop = false;
    authenticator
        .open_session()
        .expect("Failed to open a session!");
    
    Ok(user)
}

fn main() {
    // First, prompt the user for a login and a password
    print!("login: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    username.pop(); // remove the trailing '\n'
    print!("password: ");
    io::stdout().flush().unwrap();
    let password =
        input::read_password_from_tty().expect("could not read password");

    let user = login(username, password).expect("Login Failed");

    loop {
        print!("spawn sway[y,N]: ");
        io::stdout().flush().expect("Could not print to stdout");
        let mut opt = String::new();
        io::stdin().read_line(&mut opt).expect("Could not read from stdin");
        opt.pop();
        if opt == "y" {
            break;
        }
    }

    // we now try to spawn `/usr/bin/sway` as this user
    // note that setting the uid/gid is likely to fail if this program is not already run as the
    // proper user or as root
    let sway_call = Command::new("/usr/bin/sway")
        .env("XDG_RUNTIME_DIR", format!("/run/user/{}", user.uid()))
        .uid(user.uid())
        .gid(user.primary_group_id())
        .spawn();

    let _sway_proc = match sway_call {
        Ok(p) => p,
        Err(e) => panic!("error on calling sway: {:?}", e)
    };
    
    loop {
        print!("exit yolm[y,N]: ");
        io::stdout().flush().expect("Could not print to stdout");
        let mut opt = String::new();
        io::stdin().read_line(&mut opt).expect("Could not read from stdin");
        opt.pop();
        if opt == "y" {
            break;
        }
    }
}
