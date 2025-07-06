// SPDX-License-Identifier: MPL-2.0

use alloc::format;

use crate::{
    fs::{
        path::Dentry,
        procfs::template::{FileOps, ProcFileBuilder},
        rootfs::root_mount,
        utils::Inode,
    },
    prelude::*,
    Process,
};

/// Represents the inode at `/proc/[pid]/mounts`.
pub struct MountsFileOps;

impl MountsFileOps {
    pub fn new_inode(_process_ref: Arc<Process>, parent: Weak<dyn Inode>) -> Arc<dyn Inode> {
        ProcFileBuilder::new(Self).parent(parent).build().unwrap()
    }
}

impl FileOps for MountsFileOps {
    fn data(&self) -> Result<Vec<u8>> {
        let mounts = root_mount()
            .iter()
            .map(|mount| {
                let fs_type = mount.fs().type_();

                let dentry_ = mount
                    .mountpoint_dentry()
                    .unwrap_or(mount.root_dentry().clone());
                let dentry = Dentry::new(mount, dentry_);
                let path = dentry.abs_path();

                format!("{fs_type} {path} rw 0 0\n")
            })
            .collect::<Vec<String>>();
        Ok(mounts.join("").into_bytes())
    }
}
