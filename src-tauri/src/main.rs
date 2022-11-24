#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use leptess;
use screenshots::Screen;
use std::thread;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json;

static mut ON: bool = false;

#[derive(Serialize, Deserialize)]
struct DisplayInfo {
    id: u32,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    is_primary: bool,
    rotation: f32,
    scale_factor: f32,
}

#[tauri::command]
fn get_screens_name() -> Vec<String> {
    let mut infos: Vec<String> = vec![];
    let screens = Screen::all().unwrap();

    for screen in screens {
        let display_info = screen.display_info;

        let id = display_info.id;
        let x = display_info.x;
        let y = display_info.y;
        let width = display_info.width;
        let height = display_info.height;
        let is_primary = display_info.is_primary;
        let rotation = display_info.rotation;
        let scale_factor = display_info.scale_factor;

        let di = DisplayInfo {
            id,
            x,
            y,
            width,
            height,
            is_primary,
            rotation,
            scale_factor,
        };

        let testeDi = DisplayInfo {
            id,
            x,
            y,
            width,
            height,
            is_primary,
            rotation,
            scale_factor,
        };


        let serialized: String = serde_json::to_string(&di).unwrap();
        let serializedTest: String = serde_json::to_string(&testeDi).unwrap();

        infos.push(serialized);
        infos.push(serializedTest);
    }

    infos
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn read_pos_on(x: i32, y: i32, width: u32, height: u32) {
    let screen = Screen::all().unwrap()[0];

    unsafe {
        ON = !ON;
    }

    thread::spawn(move || loop {
        println!("Working...");
        thread::sleep(Duration::from_millis(1000));
        unsafe {
            if !ON {
                println!("Terminating.");
                break;
            }
        }
        let image = screen.capture_area(x, y, width, height).unwrap();
        let image_buffer = image.buffer();

        let mut lt = leptess::LepTess::new(None, "por").unwrap();
        let _ = lt.set_image_from_mem(image_buffer);
        let text = lt.get_utf8_text().unwrap();
        println!("{}", text);
    });
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_pos_on, get_screens_name])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
