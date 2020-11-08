use std::env;

fn main() {
    let mut build = cc::Build::new();
    build.include("VulkanMemoryAllocator/src");
    build.include("Vulkan-Headers/include");
    build.file("VulkanMemoryAllocatorWrapper/vma.cpp");

    let target = env::var("TARGET").unwrap();
    build.flag("-std=c++11");
    if target.contains("darwin") {
        build.cpp_link_stdlib("c++").cpp_set_stdlib("c++");
    } else if target.contains("android") {
        build.cpp_link_stdlib("c++");
    } else if target.contains("linux") {
        build.cpp_link_stdlib("stdc++");
    } else if target.contains("windows") && target.contains("gnu") {
        build.cpp_link_stdlib("stdc++");
    }
    build.cpp(true);
    build.compile("vma");
}
