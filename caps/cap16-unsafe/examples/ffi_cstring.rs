// La misma palabra como String y como CString.
use std::ffi::CString;

fn main() {
    let s = String::from("hola");
    let c = CString::new("hola").expect("sin nulos");
    println!("String : {:?}", s.as_bytes());
    println!("CString: {:?}", c.as_bytes_with_nul());
    // Un String admite un nulo en medio; un CString, no.
    let error = CString::new("ho\0la").unwrap_err();
    println!("nulo en la posición {}", error.nul_position());
}
