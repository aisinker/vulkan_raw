#[cfg(feature = "docs")]
fn main(){}

#[cfg(not(feature = "docs"))]
fn main(){
    let path = std::env::var("VK_SDK_PATH").expect("error!"); // or VULKAN_SDK
    println!("cargo:rustc-link-search=native={}{}Lib", path, std::path::MAIN_SEPARATOR);
}