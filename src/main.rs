#![allow(unused)]
extern crate gio;
extern crate gtk;
extern crate gdk_pixbuf;
extern crate id3;

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
mod playlist;

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
        window.set_title("My first GUI");

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

        window.connect_delete_event(|_, _| Inhibit(false));
    }); // Creates window
    application.connect_activate(|_| {});
    application.run(&env::args().collect::<Vec<_>>()); // starts the gtk event loop
}
