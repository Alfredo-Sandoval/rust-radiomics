//! Link the Python extension when Cargo builds it directly on macOS.
fn main() {
    pyo3_build_config::add_extension_module_link_args();
}
