fn main() {
  if cfg!(target_os = "windows") {
    println!("cargo:rustc-link-lib=dylib=LibFT4222-64");
    println!("cargo:rustc-link-lib=dylib=ftd2xx");
    println!("cargo:rustc-link-search=native=./lib");  
  }
  tauri_build::build()
}
