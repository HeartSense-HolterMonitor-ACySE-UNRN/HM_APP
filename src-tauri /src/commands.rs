use std::process::Command;
use std::str;

trait ToHTML{
    fn to_html(&self) -> String;
}

impl ToHTML for String {
    fn to_html(&self) -> String {
        self.replace("\n", "<br>")
    }
}
