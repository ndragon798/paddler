use paddler_bootstrap::list_gpu_devices::LlamaBackendDevice;
use paddler_bootstrap::list_gpu_devices::LlamaBackendDeviceType;
use paddler_bootstrap::list_gpu_devices::list_gpu_devices;

/// A backend device (GPU, integrated GPU, accelerator, or CPU) that the agent can be
/// restricted to, in the same index order accepted by `paddler agent --gpu-devices`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GpuDevice {
    /// Index of the device, as accepted by the agent's `gpu_devices` parameter
    pub index: usize,
    /// Full human-readable description of the device, formatted like `paddler list-gpu-devices`
    pub description: String,
}

/// Detects every backend device llama.cpp can see on this machine.
///
/// Falls back to an empty list (and logs the failure) if the llama.cpp backend cannot be
/// initialized, in which case the agent will use every detected device at runtime, as before.
#[must_use]
pub fn detect_devices() -> Vec<GpuDevice> {
    match list_gpu_devices() {
        Ok(devices) => devices
            .into_iter()
            .map(|device| GpuDevice {
                index: device.index,
                description: format_device(&device),
            })
            .collect(),
        Err(error) => {
            log::error!("Failed to list backend devices: {error}");
            Vec::new()
        }
    }
}

fn format_device(device: &LlamaBackendDevice) -> String {
    let device_type = match device.device_type {
        LlamaBackendDeviceType::Accelerator => "accelerator",
        LlamaBackendDeviceType::Cpu => "cpu",
        LlamaBackendDeviceType::Gpu => "gpu",
        LlamaBackendDeviceType::IntegratedGpu => "integrated gpu",
        LlamaBackendDeviceType::Unknown => "unknown",
    };

    format!(
        "{}: {} ({device_type}, {} backend, {} MiB free / {} MiB total) - {}",
        device.index,
        device.name,
        device.backend,
        device.memory_free / 1024 / 1024,
        device.memory_total / 1024 / 1024,
        device.description,
    )
}
