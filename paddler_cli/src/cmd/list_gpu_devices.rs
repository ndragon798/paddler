use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use command_handler::handler::Handler;
use paddler_bootstrap::list_gpu_devices::LlamaBackendDeviceType;
use paddler_bootstrap::list_gpu_devices::list_gpu_devices;
use tokio_util::sync::CancellationToken;

#[derive(Parser)]
/// Lists the backend devices llama.cpp detects on this machine, in the index order accepted by
/// `paddler agent --gpu-devices`
pub struct ListGpuDevices;

#[async_trait(?Send)]
impl Handler for ListGpuDevices {
    async fn handle(self, _shutdown: CancellationToken) -> Result<()> {
        let devices = list_gpu_devices()?;

        if devices.is_empty() {
            println!("No backend devices detected.");

            return Ok(());
        }

        for device in devices {
            let device_type = match device.device_type {
                LlamaBackendDeviceType::Accelerator => "accelerator",
                LlamaBackendDeviceType::Cpu => "cpu",
                LlamaBackendDeviceType::Gpu => "gpu",
                LlamaBackendDeviceType::IntegratedGpu => "integrated gpu",
                LlamaBackendDeviceType::Unknown => "unknown",
            };

            println!(
                "{index}: {name} ({device_type}, {backend} backend, {memory_free_mib} MiB free / {memory_total_mib} MiB total) - {description}",
                index = device.index,
                name = device.name,
                backend = device.backend,
                memory_free_mib = device.memory_free / 1024 / 1024,
                memory_total_mib = device.memory_total / 1024 / 1024,
                description = device.description,
            );
        }

        println!(
            "\nPass one or more of the indices above to `paddler agent --gpu-devices` to restrict inference to those devices, e.g. --gpu-devices 0 or --gpu-devices 0,1."
        );

        Ok(())
    }
}
