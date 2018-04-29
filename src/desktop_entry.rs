use std::path::PathBuf;
use trimmer::Context;
use trimmer::Parser;

pub struct ApplicationDesktopEntry<'a> {
    executable: &'a PathBuf
}

impl<'a> ApplicationDesktopEntry<'a> {

    pub fn create_for(executable: &'a PathBuf) -> Self {
        ApplicationDesktopEntry {
            executable
        }
    }

    fn get_program_name(&self) -> &str {
        self.executable
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
    }

    pub fn create_file_contents(&self) -> String {

        let exec = self.executable.to_str().unwrap().to_string();
        let program_name = self.get_program_name().to_owned();

        let mut context = Context::new();
        context.set("name", &program_name);
        context.set("exec", &exec);

        let template = r#"[Desktop Entry]
Name={{name}}
Exec={{exec}}
Type=Application
"#;

        Parser::new()
            .parse(template)
            .unwrap()
            .render(&context)
            .unwrap()
    }

    pub fn get_path(&self, mut home: PathBuf) -> PathBuf {
        home.push(".local");
        home.push("share");
        home.push("applications");
        home.push(format!("{}.desktop", self.get_program_name()));

        home
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_create_correct_file_contents() {
        let path = PathBuf::from("/foo/bar/baz.sh");

        let desktop_entry = ApplicationDesktopEntry::create_for(&path);

        let file_contents = desktop_entry.create_file_contents();
        let expected_file_contents = r#"[Desktop Entry]
Name=baz.sh
Exec=/foo/bar/baz.sh
Type=Application
"#;

        assert_eq!(file_contents, expected_file_contents);
    }

}