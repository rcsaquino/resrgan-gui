#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use native_dialog::FileDialog;
use new_mime_guess;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::api::process::Command;
use tauri::Window;

struct ConstantPaths {
    resrgan_path: String,
    model_path: String,
    ffmpeg_path: String,
    output_path: String,
    tmp_in_path: String,
    tmp_out_path: String,
}

impl ConstantPaths {
    fn new() -> Self {
        Self {
            resrgan_path: ".\\resrgan\\realesrgan-ncnn-vulkan.exe".to_string(),
            model_path: ".\\resrgan\\models".to_string(),
            ffmpeg_path: ".\\resrgan\\ffmpeg.exe".to_string(),
            output_path: format!(
                "{}\\resrgan-gui",
                path_to_string(tauri::api::path::picture_dir().unwrap())
            ),
            tmp_in_path: format!(
                "{}\\resrgan-gui\\tmp_input",
                path_to_string(tauri::api::path::picture_dir().unwrap())
            ),
            tmp_out_path: format!(
                "{}\\resrgan-gui\\tmp_output",
                path_to_string(tauri::api::path::picture_dir().unwrap())
            ),
        }
    }
}

fn path_to_string(x: PathBuf) -> String {
    x.to_str().unwrap().to_string()
}

#[tauri::command]
fn open_output_folder(win: Window) {
    let paths = ConstantPaths::new();
    let cmd = Command::new("explorer").args([&paths.output_path]).output();
    match cmd {
        Err(e) => return win.emit("output-log", format!("Error: {e}")).unwrap(),
        _ => win.emit("output-log", "Opening output folder...").unwrap(),
    }
}

#[tauri::command]
fn select_files(win: Window) -> Result<(Vec<PathBuf>, String), String> {
    win.emit("output-log", "Please select image/s or video/s...")
        .unwrap();

    let input_files = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("All files", &["*"])
        .show_open_multiple_file()
        .unwrap();

    // Check if there are files are selected
    if input_files.len() == 0 {
        return Err("No file/s selected.".to_string());
    }

    // Verify if file types are the same
    let mut file_type = String::new();
    for file in &input_files {
        // Guess the file type
        let type_guessed = new_mime_guess::from_path(file).first_raw().unwrap_or("");

        // If first file of the loop, set file type which should be similar to the succeeding files
        if file == &input_files[0] {
            // Match the guessed type to an array of allowed type
            let allowed_types = ["image", "video"];
            let matched_type = allowed_types.iter().find(|&&x| type_guessed.contains(x));

            // If matched, set accordingly; else return
            match matched_type {
                Some(x) => file_type = x.to_string(),
                _ => {
                    return Err(format!(
                        "Error: Invalid file: {:?}",
                        file.file_name().unwrap()
                    ));
                }
            }
        } else if !type_guessed.contains(&file_type) {
            return Err("Error: Different file types detected.".to_string());
        }
    }
    return Ok((input_files, file_type));
}

#[tauri::command]
fn convert(win: Window, file_paths: Vec<&str>, file_type: &str, style: &str, scale: &str) {
    // Run functions accordingly
    let res = match file_type {
        "image" => enhance_images(win.clone(), file_paths, style),
        "video" => enhance_videos(win.clone(), file_paths, scale),
        _ => {
            win.emit(
                "output-log",
                format!("Error: Invalid file type: {file_type}"),
            )
            .unwrap();
            false
        }
    };
    if !res {
        return;
    };
    win.emit("output-log", "Done. Saved results to output folder.")
        .unwrap();
}

fn enhance_images(win: Window, images: Vec<&str>, style: &str) -> bool {
    let paths = ConstantPaths::new();
    let mut img_args: Vec<&str> = vec!["-m", &paths.model_path];

    // Set Model
    let model;
    if style == "Anime" {
        model = "realesrgan-x4plus-anime";
    } else {
        model = "realesrgan-x4plus"
    }
    win.emit(
        "output-log",
        format!("Processing image/s with model {model}..."),
    )
    .unwrap();
    img_args.push("-n");
    img_args.push(model);

    // Execute command per image in input folder
    let total_images = images.len();
    let mut image_counter = 0;
    for image_path in images {
        image_counter += 1;
        let file_name = image_path.split("\\").collect::<Vec<&str>>().pop().unwrap();
        win.emit(
            "output-log",
            format!("[{image_counter}/{total_images}] Enhancing {file_name}..."),
        )
        .unwrap();
        let cmd = Command::new(&paths.resrgan_path)
            .args(&img_args)
            .args([
                "-i",
                &image_path,
                "-o",
                &format!("{}\\4x_{file_name}", &paths.output_path),
            ])
            .output();
        match cmd {
            Err(e) => {
                win.emit("output-log", format!("Error: {e}")).unwrap();
                return false;
            }
            _ => (),
        }
    }
    return true;
}

fn enhance_videos(win: Window, videos: Vec<&str>, scale: &str) -> bool {
    let paths = ConstantPaths::new();
    let mut vid_args = vec![
        "-i",
        &paths.tmp_in_path,
        "-o",
        &paths.tmp_out_path,
        "-f",
        "jpg",
        "-m",
        &paths.model_path,
    ];

    // Set Scale
    win.emit("output-log", format!("Setting output scale to {scale}..."))
        .unwrap();
    vid_args.push("-s");
    vid_args.push(scale);

    // Set model
    let model = &format!("realesr-animevideov3-x{scale}");
    vid_args.push("-n");
    vid_args.push(model);

    // Execute command per video in input folder
    let total_videos = videos.len();
    let mut video_counter = 0;
    for video_path in videos {
        let paths = ConstantPaths::new();
        // Delete temp folders if they exist
        if Path::new(&paths.tmp_in_path).exists() {
            fs::remove_dir_all(&paths.tmp_in_path).unwrap();
        }
        if Path::new(&paths.tmp_out_path).exists() {
            fs::remove_dir_all(&paths.tmp_out_path).unwrap();
        }

        // Create temp folders
        fs::create_dir(&paths.tmp_in_path).unwrap();
        fs::create_dir(&paths.tmp_out_path).unwrap();

        video_counter += 1;
        let file_name = video_path.split("\\").collect::<Vec<&str>>().pop().unwrap();
        win.emit(
            "output-log",
            format!("[{video_counter}/{total_videos}] Enhancing {file_name}..."),
        )
        .unwrap();

        // FFMPEG -i onepiece_demo.mp4 -qscale:v 1 -qmin 1 -qmax 1 -vsync 0 tmp_frames/frame%08d.jpg
        win.emit("output-log", "Extracting frames...").unwrap();
        let cmd = Command::new(&paths.ffmpeg_path)
            .args([
                "-i",
                video_path,
                "-qscale:v",
                "1",
                "-qmin",
                "1",
                "-qmax",
                "1",
                "-vsync",
                "0",
                &format!("{}\\frame%08d.jpg", &paths.tmp_in_path),
            ])
            .output();
        match cmd {
            Err(e) => {
                win.emit("output-log", format!("Error: {e}")).unwrap();
                return false;
            }
            _ => (),
        }

        // ./realesrgan-ncnn-vulkan.exe -i tmp_frames -o out_frames -n realesr-animevideov3 -s 2 -f jpg
        win.emit("output-log", "Processing extracted frames...")
            .unwrap();
        let cmd = Command::new(&paths.resrgan_path).args(&vid_args).output();
        match cmd {
            Err(e) => {
                win.emit("output-log", format!("Error: {e}")).unwrap();
                return false;
            }
            _ => (),
        }

        // FFMPEG -i out_frames/frame%08d.jpg -i onepiece_demo.mp4 -map 0:v:0 -map 1:a:0 -c:a copy -c:v libx264 -r 23.98 -pix_fmt yuv420p output_w_audio.mp4
        win.emit("output-log", "Finalizing...").unwrap();
        let cmd = Command::new(&paths.ffmpeg_path)
            .args([
                "-i",
                &format!("{}\\frame%08d.jpg", &paths.tmp_out_path),
                "-i",
                video_path,
                "-map",
                "0:v:0",
                "-map",
                "1:a:0",
                "-c:a",
                "copy",
                "-c:v",
                "libx264",
                &format!("{}\\x{scale}_{file_name}", &paths.output_path),
            ])
            .output();
        match cmd {
            Err(e) => {
                win.emit("output-log", format!("Error: {e}")).unwrap();
                return false;
            }
            _ => (),
        }

        // Delete temp folders
        fs::remove_dir_all(&paths.tmp_in_path).unwrap();
        fs::remove_dir_all(&paths.tmp_out_path).unwrap();
    }
    return true;
}

fn main() {
    let paths = ConstantPaths::new();
    // Create output folder if none exist
    if !Path::new(&paths.output_path).exists() {
        fs::create_dir_all(&paths.output_path).unwrap();
    }
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_output_folder,
            select_files,
            convert
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
