use std::sync::Once;

use crate::ffi;

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| unsafe {
        ffi::xed_tables_init();
    });
}
