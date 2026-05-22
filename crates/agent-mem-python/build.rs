fn main() {
    // Use pyo3-build-config to find Python
    pyo3_build_config::use_pyo3_cfgs();

    // On macOS with Homebrew Python, we need to link against Python framework
    if cfg!(target_os = "macos") {
        // Get Python library path from python3-config
        if let Ok(output) = std::process::Command::new("python3-config")
            .args(["--ldflags", "--embed"])
            .output()
        {
            let flags = String::from_utf8_lossy(&output.stdout);
            for flag in flags.split_whitespace() {
                if let Some(path) = flag.strip_prefix("-L") {
                    println!("cargo:rustc-link-search=native={}", path);
                } else if let Some(lib_name) = flag.strip_prefix("-l") {
                    // Extract library name (e.g., -lpython3.14 -> python3.14)
                    println!("cargo:rustc-link-lib={lib_name}");
                } else if flag == "-framework" {
                    // Framework linking is handled separately
                } else if let Some(framework) = flag.strip_prefix("-framework=") {
                    println!("cargo:rustc-link-lib=framework={framework}");
                }
            }
        } else if let Ok(output) = std::process::Command::new("python3-config")
            .args(["--ldflags"])
            .output()
        {
            let flags = String::from_utf8_lossy(&output.stdout);
            for flag in flags.split_whitespace() {
                if let Some(path) = flag.strip_prefix("-L") {
                    println!("cargo:rustc-link-search=native={}", path);
                } else if let Some(lib_name) = flag.strip_prefix("-l") {
                    if lib_name.contains("python") {
                        println!("cargo:rustc-link-lib={lib_name}");
                    }
                }
            }
        }
    }
}
