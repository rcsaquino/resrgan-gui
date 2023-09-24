<p align="center">
  <img src="assets/app-screenshot.jpg">
</p>

# Real ESRGAN GUI

- `Real ESRGAN GUI` is a simple and minimal GUI for [xinntao's Real-ESRGAN](https://github.com/xinntao/Real-ESRGAN)
- This allows you to enhance low quality images and videos using AI technology
- This GUI uses Rust for the backend and JS/HTML/CSS for the frontend using the [Tauri framework](https://github.com/tauri-apps/tauri)

> **Warning**
> 
> This is only tested on Windows 11 x64 with NVIDIA RTX 2060. Feel free to create an issue for any bugs you might encounter.

# Installation

## With installer (recommended)

1. Download the [.msi installer](https://github.com/rcsaquino/resrgan-gui/releases/download/v0.1.0/Real.ESRGAN.GUI_0.1.0_x64_en-US.msi) from the releases section
2. Double click the MSI installer
3. Follow the on screen prompts

## Portable zip

> **Warning**
> 
> Portable version is still unstable (windows 11 and *some* windows 10 users only). Try the .msi installer if you encounter problems.

1. Download the [portable zip](https://github.com/rcsaquino/resrgan-gui/releases/download/v0.1.0/resrgan-gui-portable_0.1.0_x64_en-US.zip) from the releases section
2. Extract to your preferred directory

## Compile it yourself

### Prerequisites

- [NodeJS](https://nodejs.org/)
- [Rust](https://www.rust-lang.org/)
- [Tauri](https://tauri.app/)
- [Real ESRGAN Portable (NCNN)](https://github.com/xinntao/Real-ESRGAN/releases/download/v0.2.5.0/realesrgan-ncnn-vulkan-20220424-windows.zip)
- [FFmpeg](https://www.gyan.dev/ffmpeg/builds/)

1. Clone repo and install node module dependencies

   ```bash
   git clone https://github.com/rcsaquino/resrgan-gui.git
   cd resrgan-gui
   npm install
   ```

2. Extract `Real ESRGAN Portable (NCNN)` to `~/resrgan-gui/src-tauri/resrgan`

3. Extract `FFmpeg` then copy `ffmpeg.exe` to `~/resrgan-gui/src-tauri/resrgan`

4. Build from `./resrgan-gui`

   ```bash
   cargo tauri build
   ```
5. Your build should be on `~/resrgan-gui/src-tauri/target/release`

You can customize it how you like. Just edit `~/resrgan-gui/src/App.svelte` for UI customization, and `~/resrgan-gui/src-tauri/src/main.rs` for other backend stuff. You can also run `cargo tauri dev` for hot reloading.

# Acknowledgement

Thank you to all the contributors:

- [xinntao's Real-ESRGAN](https://github.com/xinntao/Real-ESRGAN)
- Icon by [Sunder Muthukumaran](https://unsplash.com/@sunder_2k25?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText) on [Unsplash](https://unsplash.com/s/photos/icon?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText)

# Contact

If you have any question, please email `rcsaquino.md@gmail.com`

# Support My Work

If you like resrgan-gui, then please support my work and enthusiasm by buying me a coffee on [https://ko-fi/rcsaquino](https://ko-fi/rcsaquino)😊.

<a href='https://ko-fi.com/rcsaquino' target='_blank'><img height='72' style='border:0px;height:72px;' src='https://storage.ko-fi.com/cdn/kofi2.png?v=3' border='0' alt='Buy Me a Coffee at ko-fi.com' /></a>
