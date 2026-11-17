//! SigmaOS-systemd-importd — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_systemd_importd_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-systemd-importd: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-systemd-importd: ok");
}
