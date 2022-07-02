use crate::playlist::PAUSE_ICON;
use crate::playlist::PLAY_ICON;
use crate::gtk::ImageExt;
use gtk::Image;
use crate::gtk::DialogExt;
use crate::gtk::FileChooserExt;
use crate::gtk::FileFilterExt;
use gtk::{ApplicationWindow, FileChooserAction, FileChooserDialog, FileFilter}; // Open files with a file dialog
use gtk::{ContainerExt, SeparatorToolItem, ToolButton, ToolButtonExt, Toolbar, WidgetExt};
use std::path::PathBuf; // Open files with a file dialog

use super::App;
use super::Playlist;

// mod playlist;

use gtk_sys::{GTK_RESPONSE_ACCEPT, GTK_RESPONSE_CANCEL};
const RESPONSE_ACCEPT: i32 = GTK_RESPONSE_ACCEPT as i32;
const RESPONSE_CANCEL: i32 = GTK_RESPONSE_CANCEL as i32;

const PLAY_STOCK:  &str = "gtk-media-play";
const PAUSE_STOCK: &str = "gtk-media-pause";

pub struct Model {
    pub play_image: Image,
}

pub struct MusicToolbar {
    pub open_button: ToolButton,
    next_button: ToolButton,
    play_button: ToolButton,
    previous_button: ToolButton,
    quit_button: ToolButton,
    remove_button: ToolButton,
    pub stop_button: ToolButton,
    toolbar: Toolbar,
    pub play_image: Image,
}

impl MusicToolbar {
    pub fn new() -> Self {
        let toolbar = Toolbar::new();
        let open_button = ToolButton::new_from_stock("gtk-open");
        toolbar.add(&open_button);
        let next_button = ToolButton::new_from_stock("gtk-next");
        toolbar.add(&next_button);
        let play_button = ToolButton::new_from_stock("gtk-play");
        toolbar.add(&play_button);
        let previous_button = ToolButton::new_from_stock("gtk-previous");
        toolbar.add(&previous_button);
        let remove_button = ToolButton::new_from_stock("gtk-remove");
        toolbar.add(&remove_button);
        let stop_button = ToolButton::new_from_stock("gtk-stop");
        toolbar.add(&stop_button);
        let quit_button = ToolButton::new_from_stock("gtk-quit");
        toolbar.add(&quit_button);

        let play_image = Image::new_from_file("gtk-image");

        MusicToolbar {
            open_button,
            next_button,
            play_button,
            previous_button,
            quit_button,
            remove_button,
            stop_button,
            toolbar,
            play_image,
        }
    }
    pub fn toolbar(&self) -> &Toolbar {
        &self.toolbar
    }
}

fn new_icon(icon: &str) -> Image {
    Image::new_from_file(format!("assets/{}.png", icon))
}
fn model() -> Model {
    Model {
        play_image: new_icon(PLAY_ICON),
    }
}

impl App {
    pub fn connect_toolbar_events(&self) {
        let window = self.window.clone();
        self.toolbar.quit_button.connect_clicked(move |_| {
            window.destroy();
        });
        let play_button = self.toolbar.play_button.clone();
        self.toolbar.play_button.connect_clicked(move |_| {
            if play_button.get_stock_id() == Some(PLAY_STOCK.to_string()) {
                play_button.set_stock_id(PAUSE_STOCK);
            } else {
                play_button.set_stock_id(PLAY_STOCK);
            }
        });

        let parent = self.window.clone();
        let playlist = self.playlist.clone();
        self.toolbar.open_button.connect_clicked(move |_| {
            let file = show_open_dialog(&parent);
            if let Some(file) = file {
                playlist.add(&file);
            }
        });

        let playlist = self.playlist.clone();
        self.toolbar.remove_button.connect_clicked(move |_| {
            playlist.remove_selection();
        });

        let playlist = self.playlist.clone();
        let cover = self.cover.clone();
        let play_button = self.toolbar.play_button.clone();
        self.toolbar.play_button.connect_clicked(move |_| {
            if play_button.get_stock_id() == Some(PLAY_STOCK.to_string()) {
                play_button.set_stock_id(PAUSE_STOCK);
                set_cover(&cover, &playlist);
            } else {
                play_button.set_stock_id(PLAY_STOCK);
            }
        });

// FIX IT
        let playlist = self.playlist.clone();
		let play_image = self.toolbar.play_image.clone();
		let cover = self.cover.clone();
		let state = self.state.clone();
		self.toolbar.play_button.connect_clicked(move |_| {
			if state.lock().unwrap().stopped {
				if playlist.play() {
					// set_image_icon(&play_image, PAUSE_ICON);
					set_cover(&cover, &playlist);
				}
				} else {
                    playlist.pause();
					// set_image_icon(&play_image, PLAY_ICON);
				}
		});
        fn new_icon(icon: &str) -> Image {
            Image::new_from_file(format!("assets/{}.png", icon))
        }
        fn model() -> Model {
            Model {
                play_image: new_icon(PLAY_ICON),
            }
        }
        fn set_cover(cover: &Image, playlist: &Playlist) {
            cover.set_from_pixbuf(playlist.pixbuf().as_ref());
            cover.show();
        }
        
        fn set_image_icon(icon: &Image, playlist: &Playlist) {
            icon.set_from_pixbuf(playlist.pixbuf().as_ref());
            icon.show();
        }
    }
}

// Open files with a file dialog
fn show_open_dialog(parent: &ApplicationWindow) -> Option<PathBuf> {
    let mut file = None;
    let dialog = FileChooserDialog::new(
        Some("Select an MP3 audio file"),
        Some(parent),
        FileChooserAction::Open,
    );
    let filter = FileFilter::new();
    filter.add_mime_type("audio/mp3");
    filter.set_name("MP3 audio file");
    dialog.add_filter(&filter);
    dialog.add_button("Cancel", RESPONSE_CANCEL);
    dialog.add_button("Accept", RESPONSE_ACCEPT);
    let result = dialog.run();
    if result == RESPONSE_ACCEPT {
        file = dialog.get_filename();
    }
    dialog.destroy();
    file
}
