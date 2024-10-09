use editor::Editor;

mod editor;

fn main() {
    let mut editor = Editor::new();
    if let Err(e) = editor.run() {
        println!("Editore error: {e}");
    }
}