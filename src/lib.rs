#![deny(warnings)]
#![warn(unused_extern_crates)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unreachable)]
#![deny(clippy::await_holding_lock)]
#![deny(clippy::needless_pass_by_value)]
#![deny(clippy::trivially_copy_pass_by_ref)]

#[macro_use]
extern crate libnss;
#[macro_use]
extern crate lazy_static;

use std::str::FromStr;
use libnss::group::{Group, GroupHooks};
use libnss::interop::Response;
use libnss::passwd::{Passwd, PasswdHooks};

struct SynthPasswd;
libnss_passwd_hooks!(synth, SynthPasswd);

impl PasswdHooks for SynthPasswd {
    fn get_all_entries() -> Response<Vec<Passwd>> {
        Response::Success(vec![])
    }

    fn get_entry_by_uid(uid: libc::uid_t) -> Response<Passwd> {
        Response::Success(
            Passwd {
                name: uid.to_string(),
                gecos: uid.to_string(),
                passwd: "x".to_string(),
                uid,
                gid: uid,
                dir: "/var/lib/empty".to_string(),
                shell: "/usr/bin/false".to_string(),
            }
        )
    }

    fn get_entry_by_name(name: String) -> Response<Passwd> {
        if let Ok(uid) = u32::from_str(&name) {
            Self::get_entry_by_uid(uid)
        } else {
            Response::NotFound
        }
    }
}

struct SynthGroup;
libnss_group_hooks!(synth, SynthGroup);

impl GroupHooks for SynthGroup {
    fn get_all_entries() -> Response<Vec<Group>> {
        Response::Success(vec![])
    }

    fn get_entry_by_gid(gid: libc::gid_t) -> Response<Group> {
        Response::Success(
            Group {
                name: gid.to_string(),
                passwd: "x".to_string(),
                gid,
                members: vec![],
            }
        )
    }

    fn get_entry_by_name(name: String) -> Response<Group> {
        if let Ok(gid) = u32::from_str(&name) {
            Self::get_entry_by_gid(gid)
        } else {
            Response::NotFound
        }
    }
}
