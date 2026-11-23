// Ejercicio 16.5: una envoltura segura de `strlen`.
use std::ffi::{CStr, CString, c_char};

unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
}

pub fn longitud_c(s: &CStr) -> usize {
    // SAFETY: un `&CStr` siempre apunta a bytes válidos
    // terminados en nulo, que no cambian mientras dura `s`.
    unsafe { strlen(s.as_ptr()) }
}

pub fn longitud_texto(s: &str) -> Option<usize> {
    let c = CString::new(s).ok()?;
    Some(longitud_c(&c))
}
