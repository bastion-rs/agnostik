//! If a runtime flag is provided, the type for the specific executor
//! will be re-exported here.

#[cfg(bastion)]
mod bastion;
#[cfg(bastion)]
pub use bastion::*;

#[cfg(async_std)]
mod async_std;
#[cfg(async_std)]
pub use async_std::*;

#[cfg(tokio)]
mod tokio;
#[cfg(tokio)]
pub use tokio::*;

#[cfg(tokio1)]
mod tokio1;
#[cfg(tokio1)]
pub use tokio1::*;

#[cfg(smol)]
mod smol;
#[cfg(smol)]
pub use smol::*;
