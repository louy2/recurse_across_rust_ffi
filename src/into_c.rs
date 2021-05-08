extern "C" {
  pub fn into_c();
}

use std::io::Write;

#[no_mangle]
extern "C" fn into_rust() {
  std::io::stdout().write_all(b"hello from rust\n").unwrap();
  unsafe { into_c() }
} 