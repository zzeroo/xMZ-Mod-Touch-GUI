use glib::object::Cast;
use gtk::{self, Type, Widget};
use gtk::prelude::{
    BoxExt, ContainerExt, ScrolledWindowExt, TreeModelExt,
    TreeViewSignals, WidgetExt};

use server::*;
use notebook;
use sensor::Sensor;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;


pub struct SensorIndex {
    pub tree: gtk::TreeView,
    pub scroll: gtk::ScrolledWindow,
    pub info_button: gtk::Button,
    pub vertical_layout: gtk::Box,
    pub list_store: gtk::ListStore,
    pub columns: Vec<gtk::TreeViewColumn>,
}

impl SensorIndex {
    pub fn new(sensor_list: &Vec<Sensor>, notebook: &mut notebook::Notebook) -> Self {
        let tree = gtk::TreeView::new();
        let scroll = gtk::ScrolledWindow::new(None, None);
        let info_button = gtk::Button::new_with_label("Information");
        let info_button1 = info_button.clone();

        scroll.set_min_content_width(1024);
        scroll.set_min_content_height(600);

        let mut columns: Vec<gtk::TreeViewColumn> = Vec::new();

        let list_store = gtk::ListStore::new(&[
            Type::String,       // name
            Type::I32,          // adc_value
        ]);

        append_column("name", &mut columns, &tree);
        append_column("adc_value", &mut columns, &tree);

        // for sensor in sensor_list {
        //     create_and_fill_model(&list_store, &sensor.name, sensor.adc_value as u32);
        // }

        tree.set_model(Some(&list_store));
        tree.set_headers_visible(true);
        scroll.add(&tree);
        let vertical_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let Horizontal_layout = gtk::Grid::new();

        tree.connect_cursor_changed(move |tree_view| {
            let selection = tree_view.get_selection();
            if let Some((model, iter)) = selection.get_selected() {
                info_button1.set_sensitive(true);
            } else {
                info_button1.set_sensitive(false);
            }
        });
        info_button.set_sensitive(false);

        vertical_layout.pack_start(&scroll, true, true, 0);
        Horizontal_layout.attach(&info_button, 0, 0, 2, 1);
        Horizontal_layout.set_column_homogeneous(true);
        vertical_layout.pack_start(&Horizontal_layout, false, true, 0);

        let vertical_layout: Widget = vertical_layout.upcast();
        notebook.create_tab("Sensoren Liste", &vertical_layout);

        SensorIndex {
            tree: tree,
            scroll: scroll,
            info_button: info_button,
            vertical_layout: vertical_layout.downcast::<gtk::Box>().unwrap(),
            list_store: list_store,
            columns: columns,
        }
    }
}

fn append_column(title: &str, v: &mut Vec<gtk::TreeViewColumn>, tree: &gtk::TreeView) {
    let id = v.len() as i32;
    let renderer = gtk::CellRendererText::new();

    let column = gtk::TreeViewColumn::new();
    column.set_title(title);
    column.set_resizable(true); // TODO: Fixed column widths
    column.pack_start(&renderer, true);
    column.add_attribute(&renderer, "text", id);
    column.set_clickable(true);
    column.set_sort_column_id(id);
    tree.append_column(&column);
    v.push(column);
}

pub fn create_and_fill_model(list_store: &gtk::ListStore, name: &str, adc_value: u32) {
    list_store.insert_with_values(None,
                                &[0, 1],
                                &[&name,
                                  &adc_value
                                ]);
}
