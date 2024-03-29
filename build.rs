use std::env;

fn main() {
    let mut build = cc::Build::new();
    build.include("VulkanMemoryAllocator/include");
    build.include("Vulkan-Headers/include");
    build.file("VulkanMemoryAllocatorWrapper/vma.cpp");

    if cfg!(feature = "VK_VERSION_1_3") {
        build.define("VMA_VULKAN_VERSION", "1003000");
    } else {
        build.define("VMA_VULKAN_VERSION", "1002000");
    }

    let target = env::var("TARGET").unwrap();
    build.flag("/std:c++14");
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
