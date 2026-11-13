/*
By: Aum Markandey
Date: 2026-10-15
Program Details: Program that lets you add, remove, sort, and randomly select songs from a list. It provides a user-friendly interface for managing your song collection. 
It also has a feature to display the total number of songs in the list. 
*/

mod ui;
mod utils;



use crate::ui::label::Label;
use crate::ui::listview::ListView;
use crate::ui::messagebox::{MessageBox, MessageBoxResult};

use crate::ui::text_button::TextButton;
use crate::ui::text_input::TextInput;


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

    let mut btn_add_song = TextButton::new(50.0, 650.0, 200.0, 60.0, "Add Song", WHITE, GREEN, 30);
    let mut btn_sort_abc = TextButton::new(350.0, 650.0, 250.0, 60.0, "Sort Alphabetically", WHITE, BROWN, 30);
    let mut btn_reverse_abc = TextButton::new(650.0, 650.0, 350.0, 60.0, "Sort Reverse Alphabetically", WHITE, BLUE, 30);
    let mut btn_clear_list = TextButton::new(350.0, 850.0, 200.0, 60.0, "Clear List", WHITE, LIGHTGRAY, 30);
    let mut btn_random_song = TextButton::new(1050.0, 850.0, 200.0, 60.0, "Random Song", WHITE, PURPLE, 30);
    let mut btn_remove_song = TextButton::new(50.0, 850.0, 200.0, 60.0, "Remove Song", WHITE, RED, 30);
    let mut btn_exit = TextButton::new(650.0, 850.0, 200.0, 60.0, "Exit", RED, RED, 30);
    let mut info_song_not_found = MessageBox::info("Information", "Invaild song name, Song Cant Be Found in List");
    let mut info_invalidname = MessageBox::info("Information", "Invaild song name, Text input cannot be blank as that isnt a song name");

    let mut confirm_clear = MessageBox::confirm("Confirm Action", "Are you sure you want to clear");
    btn_exit.with_text_color(WHITE); // Sets the normal text color
    
    btn_add_song.with_text_color(BLACK); // Sets the normal text color
    btn_add_song.with_hover_text_color(WHITE);
    btn_sort_abc.with_text_color(BLACK); // Sets the normal text color
    btn_sort_abc.with_hover_text_color(WHITE);
    btn_clear_list.with_text_color(BLACK); // Sets the normal text color
    btn_clear_list.with_hover_text_color(BLACK);
    btn_remove_song.with_text_color(BLACK); // Sets the normal text color
    btn_remove_song.with_hover_text_color(WHITE);
    btn_reverse_abc.with_text_color(BLACK); // Sets the normal text color
    btn_reverse_abc.with_hover_text_color(WHITE);
    btn_random_song.with_text_color(BLACK); // Sets the normal text color
    btn_random_song.with_hover_text_color(WHITE);
    
    let mut lbl_text = Label::new(
        "Input Song Name Here to add or remove from the list, just click the buttons\n below to perform the action after typing the song name in the input box",
        10.0,
        100.0,
        30,
    );
    let mut lbl_songscount = Label::new("Total Songs: ", 1160.0, 400.0, 30);
list_view
        .with_colors(BLACK, Some(WHITE), Some(RED));              // Fixed width (auto-calculated if not set)
    let mut input_song = TextInput::new(250.0, 500.0, 500.0, 50.0, 30.0);
    input_song.set_prompt("Input song name here...");
    loop {
        clear_background(WHITE);
        lbl_songscount.set_text(&format!("Song Count: {}", items.len()));
        let is_empty = items.is_empty();
        if is_empty == true {
            btn_clear_list.enabled = false;
        } else {
            btn_clear_list.enabled = true;
        }

        if btn_clear_list.click() {
            confirm_clear.show();
            
        }
        if let Some(result) = confirm_clear.draw() {
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
            if is_empty == true{
                btn_random_song.enabled = false;
              } else {
                btn_random_song.enabled = true;
              }
        if btn_remove_song.click() {
              
              
            let mut found = false;
            if !input_song.get_text().is_empty() {
                for i in 0..items.len() {
                    if items[i] == input_song.get_text().to_string() {
                        items.remove(i);
                        list_view.clear();
                        found = true;
                        list_view.add_items(&items);
                        break;
                    }
                }
            } else {
                info_invalidname.show();
            }

            if found == false {
                info_song_not_found.show();
            }
        }
        if btn_add_song.click() {
            if !input_song.get_text().is_empty() {
                items.push(input_song.get_text().to_string());
                list_view.clear();
                list_view.add_items(&items);
                input_song.set_text("");
            } else {
                info_invalidname.show();
            }
        }
        if btn_sort_abc.click() {
            items.sort();
            list_view.clear();
            list_view.add_items(&items);
        }
        if btn_reverse_abc.click() {
            items.sort();
            items.reverse();
            list_view.clear();
            list_view.add_items(&items);
        }
        if btn_random_song.click() {
            let random = rand::gen_range(0, items.len());
            lbl_text.set_text(&format!("Random song:  {}", items[random]));
        }
        lbl_text.draw();
        input_song.draw();
        lbl_songscount.draw();
        if btn_exit.click() {
            break;
        }
        confirm_clear.draw();
        info_song_not_found.draw();
        info_invalidname.draw();
        list_view.draw();
        next_frame().await;
    }
}
