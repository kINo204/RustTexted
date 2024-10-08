use editer::Editor;

mod editer;

fn main() {
    let mut editor = Editor::default();
    if let Err(e) = editor.run() {
        println!("Editore error: {e}");
    }
}