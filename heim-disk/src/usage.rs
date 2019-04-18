use std::fmt;
use std::path::Path;

use heim_common::prelude::*;
use heim_common::units::iec::u64::Information;
use heim_common::units::si::f64::Ratio;

use crate::sys;

/// Disk usage statistics.
#[derive(heim_derive::ImplWrap)]
pub struct Usage(sys::Usage);

impl Usage {
    /// Returns total information amount available in partition.
    pub fn total(&self) -> Information {
        self.as_ref().total()
    }

    /// Returns used information amount used in partition.
    pub fn used(&self) -> Information {
        self.as_ref().used()
    }

    /// Returns free information about used in partition.
    pub fn free(&self) -> Information {
        self.as_ref().free()
    }

    pub fn ratio(&self) -> Ratio {
        self.as_ref().ratio()
    }
}

impl fmt::Debug for Usage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Usage")
            .field("total", &self.total())
            .field("used", &self.used())
            .field("free", &self.free())
            .field("ratio", &self.ratio())
            .finish()
    }
}

/// Returns disk [Usage] statistics about the partition which contains the given `path`.
pub fn usage<T>(path: T) -> impl Future<Item = Usage, Error = Error>
where
    T: AsRef<Path> + Send + 'static,
{
    sys::usage(path).map(Into::into)
}