mod editor;
mod terminal;
mod document;
mod row;
pub use terminal::Terminal;
pub use editor::Position;
pub use document::Document;
pub use row::Row;
use editor::Editor;


fn main() {
   Editor::default().run();
}
