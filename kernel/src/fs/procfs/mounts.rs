// SPDX-License-Identifier: MPL-2.0

use crate::{
    fs::{
        procfs::{ProcSymBuilder, SymOps},
        utils::Inode,
    },
    prelude::*,
};

/// Represents the inode at `/proc/mounts`.
pub struct MountsSymOps;

impl MountsSymOps {
    pub fn new_inode(parent: Weak<dyn Inode>) -> Arc<dyn Inode> {
        ProcSymBuilder::new(Self).parent(parent).build().unwrap()
    }
}

impl SymOps for MountsSymOps {
    fn read_link(&self) -> Result<String> {
        Ok("self/mounts".to_string())
    }
}
