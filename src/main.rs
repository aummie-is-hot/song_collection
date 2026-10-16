/*
By: <Your Name Here>
Date: 2026-10-15
Program Details: <Program Description Here>
*/

mod ui;
mod utils;

use macroquad::prelude::*;
 use crate::utils::preload_image::TextureManager;
    use crate::utils::preload_image::LoadingScreenOptions;
    use crate::ui::text_input::TextInput;
    use crate::ui::text_button::TextButton;
    use crate::ui::still_image::StillImage;
    use crate::ui::label::Label;
    use crate::ui::listview::ListView;
    use crate::ui::messagebox::MessageBox;
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
    let mut items = vec!["TNT".to_string(), "I love rock and roll".to_string()];
    let mut list_view = ListView::new(&items, 1400.0, 100.0, 30);
     list_view.with_max_visible_items(5);
    let tm = TextureManager::new();
     //tm.preload_with_loading_screen(&["assets/image1.png","assets/image2.png"], None, None).await;
      let mut btn_AddSong = TextButton::new(50.0, 350.0, 200.0, 60.0, "Add Song", WHITE, RED, 30);
    let mut btn_SortAbc = TextButton::new(50.0, 450.0, 250.0, 60.0, "Sort Alphabetically", WHITE, RED, 30);
     let mut btn_ReverseAbc = TextButton::new(50.0, 550.0, 350.0, 60.0, "Sort Reverse Alphabetically", WHITE, RED, 30);    
    let mut btn_ClearList = TextButton::new(50.0, 750.0, 200.0, 60.0, "Clear List", WHITE, RED, 30);
     
    let mut btn_RemoveSong = TextButton::new(50.0, 650.0, 200.0, 60.0, "Remove Song", WHITE, RED, 30);
    let mut btn_exit = TextButton::new(1450.0, 650.0, 200.0, 60.0, "Exit", WHITE, RED, 30);
let mut info_box = MessageBox::info("Information", "Invaild song name");
     btn_exit.with_text_color(BLACK); // Sets the normal text color
    btn_exit.with_hover_text_color(WHITE);
    btn_AddSong.with_text_color(BLACK); // Sets the normal text color
    btn_AddSong.with_hover_text_color(WHITE);
    btn_SortAbc.with_text_color(BLACK); // Sets the normal text color
    btn_SortAbc.with_hover_text_color(WHITE);
    btn_ClearList.with_text_color(BLACK); // Sets the normal text color
    btn_ClearList.with_hover_text_color(WHITE);
    btn_RemoveSong.with_text_color(BLACK); // Sets the normal text color
    btn_RemoveSong.with_hover_text_color(WHITE);
        btn_ReverseAbc.with_text_color(BLACK); // Sets the normal text color
    btn_ReverseAbc.with_hover_text_color(WHITE);
   
    let mut lbl_text = Label::new(
        "Input Song Name Here to add or remove from the list, just click the buttons\n below to perform the action after typing the song name in the input box",
        10.0,
        100.0,
        30,
    );
    let mut input_Song = TextInput::new(400.0, 300.0, 500.0, 50.0, 30.0);

    loop {
        clear_background(WHITE);

       
        if btn_ClearList.click() {
            items.clear();
            list_view.clear();
            list_view.add_items(&items);
        }
        if btn_RemoveSong.click() {
            let mut found = false;

           for i in 0..items.len(){
            if items[i] == input_Song.get_text().to_string(){
                items.remove(i);
                list_view.clear();
                found = true;
                list_view.add_items(&items);
                break;
            }
           }
           if found == false{
            info_box.show(); 
           }
        }
        if btn_AddSong.click() {
            
            items.push(input_Song.get_text().to_string()); 
            list_view.clear();
            list_view.add_items(&items);
            input_Song.set_text("");
            
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
        
        lbl_text.draw();
        input_Song.draw();
      
         if btn_exit.click() {
            break;
        }
        info_box.draw();
        list_view.draw();
        next_frame().await;
    }
}
