use std::sync::atomic::AtomicI64;
use std::sync::Mutex;

pub static LAST_CLICK: AtomicI64 = AtomicI64::new(0);
pub static IGNORE_NEXT_CLIP: Mutex<Option<String>> = Mutex::new(None);
