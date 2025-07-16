
# OpenVINO, Windows...and Rust. 

## Purpose

This Rust program is designed to help you verify that both the OpenVINO toolkit and the Rust `openvino` crate are installed and configured correctly on your Windows system. By running this project, you can quickly check that your environment is ready for developing and deploying AI inference workloads using OpenVINO in Rust.

## My Setup 

- Windows 11
- Asus ZenBook Duo, Intel Core Ultra 9 285H with 32GB RAM
- GPU: Intel® Arc™ 140T GPU; Peak TOPS: 77
- NPU: Intel® AI Boost: TOPS: 13
- Rust / Cargo: Version 1.88.0 
- Rust Dependencies: 
- - [anyhow](https://crates.io/crates/anyhow): a trait object based error type for easy idiomatic error handling in Rust applications. 
- - [openvino](https://docs.rs/openvino/latest/openvino/index.html): The openvino crate provides high-level, ergonomic, safe Rust bindings to OpenVINO. 

## Prerequisites

*Install/Update Intel® NPU Driver*

Check to see if an NPU driver is already installed on your device and what version you have:

Right-click Windows Start Button > select "Device Manager" and see if Intel® AI Boost is listed under Neural Processors. 

Check the latest version of the [Intel® NPU Driver - Windows](https://www.intel.com/content/www/us/en/download/794734/intel-npu-driver-windows.html)

If you need to install/update, follow [these instructions](https://downloadmirror.intel.com/854488/NPU_Win_Release_Notes_v32.0.100.4023.pdf)

*Install OpenVINO on Windows*

1. Create a folder named "Intel" in your local `C:\Program Files (x86)` directory, if it does not already exist.

2. Download the [OpenVINO Runtime Archive File for Windows](https://storage.openvinotoolkit.org/repositories/openvino/packages/2025.2/windows/) to your local Downloads folder. Look for the archive file that starts with "openvino_toolkit_windows_".

3. Extract the archive file and rename the extracted folder "`openvino_2025`", then move this renamed folder into the Intel subfolder from Step 1. 

4. Create an Environment Variable `OPENVINO_INSTALL_DIR`, with value `C:\Program Files (x86)\Intel\openvino_2025`. Later on, when we use the Rust openvino crate, it will search for this environment variable. 

5. Add the folllowing two subfolders to your System PATH: 
- `C:\Program Files (x86)\Intel\openvino_20 25\runtime\bin\intel64\Release`
- `C:\Program Files (x86)\Intel\openvino_2025\runtime\3rdparty\tbb\bin`

You will now be able to use the Rust `openvino` crate for your Rust projects. 

6. In the `rust-openvino-check` folder, run `cargo run.` If OpenVINO is successfully installed, an OpenVINO core will be intialized and a quick device check will occur that will display your avaiable devices and a few basic properties. 

## OpenVINO 

OpenVINO™ (Open Visual Inference and Neural Network Optimization) is an open-source toolkit developed by Intel to **optimize and accelerate AI inference** across a wide range of Intel hardware platforms—including CPUs, integrated and discrete GPUs, NPUs, VPUs, and FPGAs.

### 🧠 What OpenVINO Is
- A toolkit designed for **deploying deep learning models** efficiently.
- Supports models from popular frameworks like TensorFlow, PyTorch, and ONNX.
- Converts trained models into an intermediate representation (IR) for optimized execution.

### 🎯 Purpose of OpenVINO
- **Maximize inference performance** for AI workloads such as computer vision, NLP, and generative AI.
- Enable **cross-platform deployment** with a “write once, deploy anywhere” approach.
- Reduce latency and increase throughput for real-time applications at the edge and in the cloud.

### ⚙️ How It Uses Intel GPUs and NPUs
- **Intel GPUs** (e.g., Arc™, Flex Series) are leveraged for high-throughput parallel processing using features like:
  - **XMX (Xe Matrix Extensions)** for accelerated matrix multiplication.
  - **Parallel stream execution** to run multiple inference requests simultaneously.
- **Intel NPUs** (Neural Processing Units) are supported for low-power, high-efficiency inference—ideal for edge devices.
- OpenVINO uses **OneDNN** kernels and its own GPU plugins to tap into hardware-specific acceleration blocks like systolic arrays.
- Automatically selects the best execution path based on hardware capabilities, including support for **INT8 and FP16 precision** for faster inference.

In short, OpenVINO is Intel’s secret sauce for squeezing every ounce of performance out of its silicon for AI workloads. Want to dive deeper into how it compares across devices or how to tune it for your own models? I’ve got plenty more where that came from.

### Additional Resources: 

- [OpenVINO 2025.2 Documentation / Tutorials](https://docs.openvino.ai/2025/index.html)
- [What is OpenVINO? A Guide for Beginners on Roboflow Blog](https://blog.roboflow.com/what-is-openvino/)


## What is an NPU? 

Intel's **Neural Processing Unit (NPU)** is a specialized accelerator designed to handle **AI and machine learning workloads** efficiently, particularly those involving **neural networks**. It is integrated into Intel's latest processors to offload AI tasks from the CPU and GPU, improving performance and power efficiency.

#### **History of Intel’s NPU**
1. **Early AI Efforts (Pre-2019):**  
   - Intel initially relied on CPUs and GPUs for AI workloads, with optimizations via libraries like **Intel MKL (Math Kernel Library)** and frameworks such as **OpenVINO** for inference.
   - In 2019, Intel acquired **Habana Labs** (an AI chip startup) to strengthen its AI accelerator portfolio, though Habana's products (Gaudi/Goya) remain separate from consumer NPUs.

2. **Introduction of NPUs in Consumer CPUs (2023):**  
   - Intel first introduced an integrated NPU in its **Meteor Lake (14th Gen Core Ultra)** processors (released December 2023).  
   - This marked Intel’s shift toward **on-chip AI acceleration** for PCs, competing with Apple’s Neural Engine and AMD’s Ryzen AI (XDNA architecture).

3. **Future Developments (2024+):**  
   - Intel plans to enhance NPU performance in upcoming architectures like **Arrow Lake (15th Gen)** and **Lunar Lake**, with higher TOPS (Tera Operations Per Second) for AI workloads.

#### **What Intel’s NPU Is Used For**
The NPU is optimized for **low-power, sustained AI workloads**, including:
- **AI-powered features in Windows 11** (e.g., Studio Effects, Live Captions, Copilot+ AI features).
- **Local AI inference** (running models like Stable Diffusion, Whisper, or Llama locally instead of in the cloud).
- **Background AI tasks** (noise cancellation, gaze correction, voice recognition).
- **Content creation acceleration** (AI upscaling, object removal in Adobe apps, AI-assisted video editing).

#### **Key Features**
- **Low-power operation** (designed for efficiency in thin-and-light laptops).
- **Offloads AI from CPU/GPU**, freeing them for other tasks.
- **Supports Windows ML, DirectML, and OpenVINO** for AI model deployment.

Intel’s NPU represents its push into **on-device AI acceleration**, enabling better performance and battery life for AI applications in PCs. As AI becomes more integral to computing, Intel plans to expand NPU capabilities in future chips.