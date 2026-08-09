#[cfg(feature = "agnostic")]
pub(crate) mod agnostic {
    pub(crate) mod component {
        pub(crate) mod net {
            pub(crate) mod net;
        }
        pub(crate) mod cmd;
        pub(crate) mod ui;
    }
    pub(crate) mod consts;
}
pub(crate) mod winsys;

pub use crate::{
    agnostic::{
        component::{
            net::{
                net::*,
            },
            cmd::*,
            ui::*,
        },
        consts::*,
    },
    winsys::*,
};

pub(crate) use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
    ffi::c_void,
    os::windows::ffi::OsStrExt,
};

pub(crate) use sourcetrait_twostr::*;
pub(crate) use sourcetrait_agnostic_bridge::{
    self as agnostic,
    prelude::driver::*
};
pub(crate) use widestring::{U16CString, U16CStr};
