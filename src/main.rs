/*
By: <Your Name Here>
Date: 2026-10-15
Program Details: <Program Description Here>
*/

mod ui;
mod utils;

use std::fmt::format;

use crate::ui::label::Label;
use crate::ui::listview::ListView;
use crate::ui::messagebox::{MessageBox, MessageBoxResult};
use crate::ui::still_image::StillImage;
use crate::ui::text_button::TextButton;
use crate::ui::text_input::TextInput;
use crate::utils::preload_image::LoadingScreenOptions;
use crate::utils::preload_image::TextureManager;

use macroquad::prelude::*;
use miniquad::date;
/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "song collection software".to_string(),
        window_width: 1700,
        window_height: 1000,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(date::now() as u64);
    let mut items = vec!["TNT".to_string(), "I love rock and roll".to_string()];
    let mut list_view = ListView::new(&items, 50.0, 300.0, 30);
    list_view.with_max_visible_items(5);
    //let tm = TextureManager::new();
    //tm.preload_with_loading_screen(&["assets/image1.png","assets/image2.png"], None, None).await;
    let mut btn_AddSong = TextButton::new(50.0, 650.0, 200.0, 60.0, "Add Song", WHITE, GREEN, 30);
    let mut btn_SortAbc = TextButton::new(350.0, 650.0, 250.0, 60.0, "Sort Alphabetically", WHITE, BROWN, 30);
    let mut btn_ReverseAbc = TextButton::new(650.0, 650.0, 350.0, 60.0, "Sort Reverse Alphabetically", WHITE, BLUE, 30);
    let mut btn_ClearList = TextButton::new(350.0, 850.0, 200.0, 60.0, "Clear List", WHITE, LIGHTGRAY, 30);
    let mut btn_Random_Song = TextButton::new(1050.0, 850.0, 200.0, 60.0, "Random Song", WHITE, PURPLE, 30);
    let mut btn_RemoveSong = TextButton::new(50.0, 850.0, 200.0, 60.0, "Remove Song", WHITE, RED, 30);
    let mut btn_exit = TextButton::new(650.0, 850.0, 200.0, 60.0, "Exit", WHITE, LIGHTGRAY, 30);
    let mut info_SongNotFound = MessageBox::info("Information", "Invaild song name, Song Cant Be Found in List");
    let mut info_InvalidName = MessageBox::info("Information", "Invaild song name, Text input cannot be blank as that isnt a song name");

    let mut confirm_Clear = MessageBox::confirm("Confirm Action", "Are you sure you want to clear");
    btn_exit.with_text_color(BLACK); // Sets the normal text color
    btn_exit.with_hover_text_color(BLACK);
    btn_AddSong.with_text_color(BLACK); // Sets the normal text color
    btn_AddSong.with_hover_text_color(WHITE);
    btn_SortAbc.with_text_color(BLACK); // Sets the normal text color
    btn_SortAbc.with_hover_text_color(WHITE);
    btn_ClearList.with_text_color(BLACK); // Sets the normal text color
    btn_ClearList.with_hover_text_color(BLACK);
    btn_RemoveSong.with_text_color(BLACK); // Sets the normal text color
    btn_RemoveSong.with_hover_text_color(WHITE);
    btn_ReverseAbc.with_text_color(BLACK); // Sets the normal text color
    btn_ReverseAbc.with_hover_text_color(WHITE);
    btn_Random_Song.with_text_color(BLACK); // Sets the normal text color
    btn_Random_Song.with_hover_text_color(WHITE);
    let mut Random = 0;
    let mut lbl_text = Label::new(
        "Input Song Name Here to add or remove from the list, just click the buttons\n below to perform the action after typing the song name in the input box",
        10.0,
        100.0,
        30,
    );
    let mut lbl_SongsCount = Label::new("Total Songs: ", 1160.0, 400.0, 30);
list_view
        .with_colors(BLACK, Some(WHITE), Some(RED));              // Fixed width (auto-calculated if not set)
    let mut input_Song = TextInput::new(250.0, 500.0, 500.0, 50.0, 30.0);
    input_Song.set_prompt("Input song name here...");
    loop {
        clear_background(WHITE);
        lbl_SongsCount.set_text(&format!("Song Count: {}", items.len()));
        let is_empty = items.is_empty();
        if is_empty == true {
            btn_ClearList.enabled = false;
        } else {
            btn_ClearList.enabled = true;
        }

        if btn_ClearList.click() {
            confirm_Clear.show();
            
        }
        if let Some(result) = confirm_Clear.draw() {
                match result {
                    MessageBoxResult::ButtonPressed(0) => {
                        // "Yes" button pressed
                        items.clear();
                        list_view.clear();
                        list_view.add_items(&items);
                    }
                    #[allow(unused)]
                    MessageBoxResult::ButtonPressed(_) => {
                        
                    }
                    MessageBoxResult::Closed => {
                        // "No" button pressed
                        // Continue without saving...
                    }
                }
            }
        if btn_RemoveSong.click() {
              let selected = list_view.selected_item(); 
              if is_empty == true{
                
              }
            let mut found = false;
            if !input_Song.get_text().is_empty() {
                for i in 0..items.len() {
                    if items[i] == input_Song.get_text().to_string() {
                        items.remove(i);
                        list_view.clear();
                        found = true;
                        list_view.add_items(&items);
                        break;
                    }
                }
            } else {
                info_InvalidName.show();
            }

            if found == false {
                info_SongNotFound.show();
            }
        }
        if btn_AddSong.click() {
            if !input_Song.get_text().is_empty() {
                items.push(input_Song.get_text().to_string());
                list_view.clear();
                list_view.add_items(&items);
                input_Song.set_text("");
            } else {
                info_InvalidName.show();
            }
        }
        if btn_SortAbc.click() {
            items.sort();
            list_view.clear();
            list_view.add_items(&items);
        }
        if btn_ReverseAbc.click() {
            items.sort();
            items.reverse();
            list_view.clear();
            list_view.add_items(&items);
        }
        if btn_Random_Song.click() {
            Random = rand::gen_range(0, items.len());
            lbl_text.set_text(&format!("Random song:  {}", items[Random]));
        }
        lbl_text.draw();
        input_Song.draw();
        lbl_SongsCount.draw();
        if btn_exit.click() {
            break;
        }
        confirm_Clear.draw();
        info_SongNotFound.draw();
        info_InvalidName.draw();
        list_view.draw();
        next_frame().await;
    }
}
