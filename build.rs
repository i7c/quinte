use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .whitelist_function("notmuch_database_destroy")
        .whitelist_function("notmuch_database_open_verbose")
        .whitelist_function("notmuch_message_destroy")
        .whitelist_function("notmuch_message_get_date")
        .whitelist_function("notmuch_message_get_filename")
        .whitelist_function("notmuch_message_get_header")
        .whitelist_function("notmuch_message_get_message_id")
        .whitelist_function("notmuch_messages_get")
        .whitelist_function("notmuch_messages_move_to_next")
        .whitelist_function("notmuch_messages_valid")
        .whitelist_function("notmuch_query_create")
        .whitelist_function("notmuch_query_destroy")
        .whitelist_function("notmuch_query_search_messages")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    bindings
        .write_to_file("bindings.rs")
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=notmuch");
}
