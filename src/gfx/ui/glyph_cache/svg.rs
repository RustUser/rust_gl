pub const DEFAULT_VERSION: &'static str = "1.1";
pub const DEFAULT_XMLNS: &'static str = "http://www.w3.org/2000/svg";

pub type T = dyn ToString;

pub struct SVG {
    width: u32,
    height: u32,
    xmlns: String,
    version: String,
    contents: String
}

impl SVG {
    pub fn new(width: u32, height: u32) -> SVG {
        Self {
            width,
            height,
            xmlns: DEFAULT_XMLNS.to_string(),
            version: DEFAULT_VERSION.to_string(),
            contents: "".to_string()
        }
    }

    pub fn set_contents(&mut self, contents: &T) {
        self.contents = contents.to_string();
    }

    pub fn push_contents(&mut self, contents: &T) {
        self.contents.push_str(contents.to_string().as_str());
    }

    pub fn with_contents(mut self, contents: &T) -> SVG {
        self.set_contents(contents);
        self
    }
}

impl ToString for SVG {
    fn to_string(&self) -> String {
        let header = format!("<svg version=\"{}\" width=\"{}\" height=\"{}\" xmlns=\"{}\">", self.version, self.width, self.height, self.xmlns);

        let contents = format!("<path d=\"{}\"/>", self.contents);

        let footer = "</svg>";

        format!("{}\n{}\n{}", header, contents, footer)
    }
}