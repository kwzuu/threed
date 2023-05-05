#![feature(cell_update)]

mod physics;

/// This example shows how to describe the adapter in use.
async fn run() {
    let adapter = {
        let instance = wgpu::Instance::default();
        #[cfg(not(target_arch = "wasm32"))]
        {
            log::info!("Available adapters:");
            for a in instance.enumerate_adapters(wgpu::Backends::all()) {
                log::info!("    {:?}", a.get_info())
            }
        }
        instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap()
    };

    log::info!("Selected adapter: {:?}", adapter.get_info())
}

fn main() {
    env_logger::init();
    pollster::block_on(run());
}