use std::sync::atomic::{AtomicBool, AtomicI64};
use std::sync::Mutex;

pub static LAST_CLICK: AtomicI64 = AtomicI64::new(0);
pub static LAST_BLUR: AtomicI64 = AtomicI64::new(0);
pub static SUPPRESS_HIDE: AtomicBool = AtomicBool::new(false);
pub static IGNORE_NEXT_CLIP: Mutex<Option<String>> = Mutex::new(None);
