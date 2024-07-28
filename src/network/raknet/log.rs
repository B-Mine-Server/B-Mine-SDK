use std::sync::atomic::{AtomicU8, Ordering};

pub static ENABLE_RAKNET_LOG: AtomicU8 = AtomicU8::new(0);