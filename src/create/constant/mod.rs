mod html;
mod prism;
mod script;
mod style;

pub use html::{HTML_BOTTOM, HTML_TOP_END, HTML_TOP_START};
pub use prism::{PRISM_CSS, PRISM_JS};
pub use script::SCRIPT;
pub use style::STYLE;

pub const TRIM_PATTERN: [char; 3] = ['/', '*', ' '];
