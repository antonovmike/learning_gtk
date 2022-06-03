#![allow(unused)]
extern crate gio;
extern crate gtk;

use std::env;

use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};
use gtk::{
    Application, ApplicationWindow, ContainerExt, GtkWindowExt, Inhibit, SeparatorToolItem,
    ToolButton, ToolButtonExt, Toolbar, WidgetExt, Box,
    Adjustment, Image, ImageExt, Scale, ScaleExt,
};

use toolbar::MusicToolbar;
use gtk::Orientation::{Horizontal, Vertical};

mod toolbar;

const PLAY_STOCK: &str = "gtk-media-play";
const PAUSE_STOCK: &str = "gtk-media-pause";

struct App {
    adjustment: Adjustment,
	cover: Image,
    toolbar: MusicToolbar,
    window: ApplicationWindow,
}

// this constructor creates window and MusicToolbar
impl App {
    fn new(application: Application) -> Self {
        let window = ApplicationWindow::new(&application);
        window.set_title("Works");
        let toolbar = MusicToolbar::new();

        let vbox = gtk::Box::new(Vertical, 0);
        window.add(&vbox);

// toolbar widgets
        let toolbar = MusicToolbar::new();
        vbox.add(toolbar.toolbar());
        let cover = Image::new();
        cover.set_from_file("src/image/atpharkfall.jpg");
        vbox.add(&cover);
// cursor widget
        let adjustment = Adjustment::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
        let scale = Scale::new(Horizontal, &adjustment);
        scale.set_draw_value(false);
        vbox.add(&scale);

        window.show_all();
		let app = App {
			adjustment,
			cover,
			toolbar,
			window,
		};

        app.connect_events();
        app.connect_toolbar_events();
        app
    }
    fn connect_events(&self) {}
}

fn main() {
    let application = Application::new("com.github.rust-by-example", ApplicationFlags::empty())
        .expect("Application initialization failed");

    application.connect_startup(|application| {
        let window = App::new(application.clone()).window;
        window.set_title("My first GUI");
        window.show();
        window.connect_delete_event(|_, _| Inhibit(false));
        let toolbar = Toolbar::new();
        
        window.add(&toolbar);
        let open_button = ToolButton::new_from_stock("gtk-open");
        toolbar.add(&open_button);

        toolbar.add(&SeparatorToolItem::new());

        let previous_button = ToolButton::new_from_stock("gtk-media-previous");
        toolbar.add(&previous_button);

        let play_button = ToolButton::new_from_stock(PLAY_STOCK);
        toolbar.add(&play_button);

        let stop_button = ToolButton::new_from_stock("gtk-media-stop");
        toolbar.add(&stop_button);

        let next_button = ToolButton::new_from_stock("gtk-media-next");
        toolbar.add(&next_button);

        toolbar.add(&SeparatorToolItem::new());

        let remove_button = ToolButton::new_from_stock("gtk-remove");
        toolbar.add(&remove_button);

        toolbar.add(&SeparatorToolItem::new());

        let quit_button = ToolButton::new_from_stock("gtk-quit");
        toolbar.add(&quit_button);

        // window.show_all();
    }); // Creates window
    application.connect_activate(|_| {});
    application.run(&env::args().collect::<Vec<_>>()); // starts the gtk event loop
}
