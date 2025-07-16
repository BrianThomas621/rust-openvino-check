use openvino::Core;
use openvino::{PropertyKey, RwPropertyKey};

fn main() -> anyhow::Result<()> {
    let core = Core::new().expect("Failed to create OpenVINO Core");
    println!("OpenVINO Core initialized successfully.");
    println!("Enumerating available devices...");

    let devices = core.available_devices()?;
    // Print the available devices
    for device in devices {
        let device_name = core.get_property(&device, &PropertyKey::DeviceFullName)?;
        let device_capable = core.get_property(&device, &PropertyKey::DeviceCapabilities)?;
        let inference_precision = core.get_property(
            &device,
            &PropertyKey::Rw(RwPropertyKey::HintInferencePrecision),
        )?;
        println!("{}", "-".repeat(100));
        print! {"|\tDevice Name: {}", device_name};
        print! {"\n|\tDevice Type: {}", device};
        print! {"\n|\tInference Precision: {}", inference_precision};
        print! {"\n|\tDevice Capabilities: {:?}", device_capable};
        println!();
    }
    println!("{}", "-".repeat(100));
    Ok(())
}
