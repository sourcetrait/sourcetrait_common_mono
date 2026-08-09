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

pub use crate::{
    agnostic::{
        component::{
            net::{
                net::*,
            },
            cmd::*,
            ui::*,
        },
    },
};

pub(crate) use crate::{
    agnostic::{
        consts::*,
    },
};

pub(crate) use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
    sync::LazyLock,
};

pub(crate) use sourcetrait_agnostic_bridge::{
    self as agnostic,
    //prelude::driver::*,
    CommandReturn, CmdKind,
};
pub(crate) use sourcetrait_agnostic_unix as unix;
pub(crate) use sourcetrait_stdx::process::CommandExt;
//pub(crate) use sourcetrait_twostr::*;
