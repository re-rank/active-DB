pub mod engine;
pub mod gateway;
#[cfg(feature = "compiler")]
pub mod compiler;
pub mod protocol;
pub mod utils;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
