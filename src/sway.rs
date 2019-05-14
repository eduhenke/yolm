use std::os::unix::process::CommandExt;
use std::process::Command;
use users::User;

pub fn spawn(user: User) -> Result<(), ()> {
    // we now try to spawn `/usr/bin/sway` as this user
    // note that setting the uid/gid is likely to fail if this program is not already run as the
    // proper user or as root
    let sway_call = Command::new("/usr/bin/sway")
        .env("XDG_RUNTIME_DIR", format!("/run/user/{}", user.uid()))
        .uid(user.uid())
        .gid(user.primary_group_id())
        .spawn();

    match sway_call {
        Ok(_sway_proc) => Ok(()),
        Err(_e) => Err(()),
    }
}