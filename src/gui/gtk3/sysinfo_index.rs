use gtk::{Builder, TextView};

pub fn setup(builder: &Builder) {
    let textview_ifconfig: TextView = builder.get_object("textview_ifconfig").unwrap();

    let ifconfig = ::sysinfo::ifconfig();
    textview_ifconfig.get_buffer().unwrap().set_text(&ifconfig);
}
