// Copyright(c) The Maintainers of Nanvix.
// Licensed under the MIT License.

//==================================================================================================
// Modules
//==================================================================================================

cfg_if::cfg_if! {
    if #[cfg(feature = "syscall")] {
        mod syscalls;
        pub use self::syscalls::{
            sched_yield::sched_yield,
        };
        pub mod bindings;
    }
}
