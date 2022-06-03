use gdk_pixbuf::{InterpType, Pixbuf, PixbufLoader};
use gtk::{
    CellLayoutExt, CellRendererPixbuf, CellRendererText, ListStore, ListStoreExt,
    ListStoreExtManual, StaticType, ToValue, TreeIter, TreeModelExt, TreeSelectionExt, TreeView,
    TreeViewColumn, TreeViewColumnExt, TreeViewExt, Type, WidgetExt,
};
use id3::Tag;
use std::path::Path;

const THUMBNAIL_COLUMN: u32 = 0;
const TITLE_COLUMN: u32 = 1;
const ARTIST_COLUMN: u32 = 2;
const ALBUM_COLUMN: u32 = 3;
const GENRE_COLUMN: u32 = 4;
const YEAR_COLUMN: u32 = 5;
const TRACK_COLUMN: u32 = 6;
const PATH_COLUMN: u32 = 7;
const PIXBUF_COLUMN: u32 = 8;
const IMAGE_SIZE: i32 = 256;
const THUMBNAIL_SIZE: i32 = 64;

pub struct Playlist {
    model: ListStore,
    treeview: TreeView,
}
