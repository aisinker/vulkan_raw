#[cfg(feature = "VK_KHR_external_fence_fd")]
pub mod external_fence_fd;

#[cfg(feature = "VK_KHR_external_fence_win32")]
pub mod external_fence_win32;

#[cfg(feature = "VK_KHR_pipeline_library")]
pub mod pipeline_library;

#[cfg(feature = "VK_KHR_deferred_host_operations")]
pub mod deferred_host_operations;

#[cfg(feature = "VK_KHR_surface")]
pub mod surface;

#[cfg(feature = "VK_KHR_swapchain")]
pub mod swapchain;

#[cfg(feature = "VK_KHR_win32_surface")]
pub mod win32_surface;

#[cfg(feature = "VK_KHR_acceleration_structure")]
pub mod acceleration_structure;

#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
pub mod ray_tracing_pipeline;

#[cfg(feature = "VK_KHR_ray_query")]
pub mod ray_query;
