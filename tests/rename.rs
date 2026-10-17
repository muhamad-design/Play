use play::io::{clipboard, file};
use play::project::format::{Manifest, PageData, page_from_json, page_to_json};

#[test]
fn existing_documents_and_projects_survive_rename() {
    let doc = play::demo::showcase().doc;
    let old_file = file::to_string(&doc, None).replace("\"play\"", "\"paper-rs\"");
    assert_eq!(file::from_str(&old_file).unwrap().0.nodes, doc.nodes);
    let page = PageData {
        id: "test".into(),
        name: doc.name.clone(),
        doc,
        camera: None,
    };
    let old_page = page_to_json(&page).replace("play-page", "paper-rs-page");
    assert_eq!(page_from_json(&old_page).unwrap(), page);
    let manifest = Manifest::new("Existing project");
    let old_manifest = manifest
        .to_json()
        .replace("play-project", "paper-rs-project");
    assert_eq!(Manifest::from_json(&old_manifest).unwrap(), manifest);
    assert!(matches!(
        page_from_json(&old_page.replace("\"version\": 3", "\"version\": 99")),
        Err(play::project::format::FormatError::TooNew { .. })
    ));
}

#[test]
fn existing_library_and_clipboard_survive_rename() {
    let path = std::env::temp_dir().join(format!("play-rename-{}.json", fastrand::u64(..)));
    let library = play::library::starter();
    let old = serde_json::to_string(&library)
        .unwrap()
        .replace("play-library", "paper-rs-library");
    std::fs::write(&path, old).unwrap();
    let loaded = play::library::Library::load(&path);
    assert_eq!(loaded.items, library.items);
    assert_eq!(loaded.format, "play-library");
    std::fs::remove_file(path).unwrap();
    assert!(matches!(
        clipboard::decode(r#"{"type":"paper-rs/clipboard","nodes":[],"roots":[],"world":[]}"#),
        clipboard::Content::Layers(_)
    ));
}
