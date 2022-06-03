use gtk::{ContainerExt, SeparatorToolItem, ToolButton, ToolButtonExt, Toolbar, WidgetExt};

use super::App;

const PLAY_STOCK: &str = "gtk-media-play";
const PAUSE_STOCK: &str = "gtk-media-pause";

pub struct MusicToolbar {
    open_button: ToolButton,
    next_button: ToolButton,
    play_button: ToolButton,
    previous_button: ToolButton,
    quit_button: ToolButton,
    remove_button: ToolButton,
    stop_button: ToolButton,
    toolbar: Toolbar,
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
        MusicToolbar {
            open_button,
            next_button,
            play_button,
            previous_button,
            quit_button,
            remove_button,
            stop_button,
            toolbar,
        }
    }
    pub fn toolbar(&self) -> &Toolbar {
        &self.toolbar
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
    }
}
