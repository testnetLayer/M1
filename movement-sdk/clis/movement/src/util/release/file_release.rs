use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use super::{ReleaseOperations, Release};
use crate::util::location::Location;
use crate::util::util::Version;
use crate::util::sys::{Arch, OS};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct File(PathBuf);

impl File {

    pub fn new(path : PathBuf) -> Self {
        Self(path)
    }

    pub fn path(&self) -> &PathBuf {
        &self.0
    }

}

#[async_trait::async_trait]
impl ReleaseOperations for File {

    async fn get(&self, location : &Location) -> Result<(), anyhow::Error> {

        match location {
            Location::StagedFiles(release_dest)=>{

                // use the 0th path in the release target paths
                match release_dest.release_target_paths.get(0) {
                    Some(path) => {
                        // copy the release file to the path
                        std::fs::copy(self.path(), path)?;
                    },
                    None => {
                        anyhow::bail!("Cannot get a file release to a non-release location.");
                    }
                }

            }
            _ => {
                anyhow::bail!("Cannot get a file release to a non-release location.");
            }
        }
  
        Ok(())

    }

    fn with_version(mut self, version : &Version) -> Self {
        self
    }

    fn with_arch(mut self, arch : &Arch) -> Self {
        self
    }

    fn with_os(mut self, os : &OS) -> Self {
        self
    }

}

impl Into<Release> for File {
    fn into(self) -> Release {
        Release::File(self)
    }
}