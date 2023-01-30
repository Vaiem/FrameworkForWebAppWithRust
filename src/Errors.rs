use std::{fmt,error::Error};

// ServErrorBuild is server::Build, an application instance can only be assembled once
#[derive(Debug)]
pub struct ServErrorBuild;

impl fmt::Display for ServErrorBuild {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ServErrorBuild is server::Build, an application instance can only be assembled once")
    }
}

