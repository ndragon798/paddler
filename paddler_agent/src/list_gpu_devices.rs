use anyhow::Context;
use anyhow::Result;
use llama_cpp_bindings::llama_backend::LlamaBackend;

pub use llama_cpp_bindings::LlamaBackendDevice;
pub use llama_cpp_bindings::LlamaBackendDeviceType;

/// Lists every backend device (GPU, integrated GPU, accelerator, or CPU) that llama.cpp can see
/// on this machine, in the same index order accepted by `--gpu-devices`.
///
/// # Errors
/// Returns an error if the llama.cpp backend fails to initialize.
pub fn list_gpu_devices() -> Result<Vec<LlamaBackendDevice>> {
    let _llama_backend =
        LlamaBackend::init().context("Unable to initialize llama.cpp backend")?;

    Ok(llama_cpp_bindings::list_llama_ggml_backend_devices())
}
