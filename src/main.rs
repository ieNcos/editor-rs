//use std::io::Write;
use std::io;

mod editor;
use editor::Editor;




fn main() -> io::Result<()>{
    Editor::default().run()
}
