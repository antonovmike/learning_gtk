#![allow(unused)]
extern crate gdk_pixbuf;
extern crate gio;
extern crate gtk;
extern crate gtk_sys; // open mp3 files
extern crate id3;
extern crate crossbeam;
extern crate pulse_simple;
extern crate simplemad;

use std::env;
use std::rc::Rc; // reference counter
use std::time::Duration;

use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};
use gtk::{
    Adjustment, Application, ApplicationWindow, Box, ContainerExt, GtkWindowExt, Image, ImageExt,
    Inhibit, Scale, ScaleExt, SeparatorToolItem, ToolButton, ToolButtonExt, Toolbar, WidgetExt,
};

use crate::playlist::Playlist;
use gtk::Orientation::{Horizontal, Vertical};
use toolbar::MusicToolbar;

mod playlist;
mod toolbar;
mod mp3;

const PLAY_STOCK: &str = "gtk-media-play";
const PAUSE_STOCK: &str = "gtk-media-pause";

struct App {
    adjustment: Adjustment,
    cover: Image,
    playlist: Rc<Playlist>,
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

        let playlist = Rc::new(Playlist::new()); // Paylist is wrapped inside an reference counter
        vbox.add(playlist.view());

        let cover = Image::new();
        // cover.set_from_file("src/image/atpharkfall.jpg");
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
            playlist,
            toolbar,
            window,
        };
        app.connect_events();
        app.connect_toolbar_events();
        app
    }
    fn connect_events(&self) {}
}

fn to_millis(duration: Duration) -> u64 {
	duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1_000_000
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
