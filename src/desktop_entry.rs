use std::path::PathBuf;
use trimmer::Context;
use trimmer::Parser;

pub struct ApplicationDesktopEntry<'a> {
    executable: &'a PathBuf,
    name: Option<&'a str>
}

impl<'a> ApplicationDesktopEntry<'a> {
    pub fn create_for(executable: &'a PathBuf, name: Option<&'a str>) -> Self {
        ApplicationDesktopEntry { executable, name }
    }

    fn get_program_name(&self) -> &str {
        let file_name = self.executable.file_name().unwrap().to_str();
        self.name.or(file_name).unwrap()
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
Type=Application"#;

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

        let desktop_entry = ApplicationDesktopEntry::create_for(&path, None);

        let file_contents = desktop_entry.create_file_contents();
        let expected_file_contents = r#"[Desktop Entry]
Name=baz.sh
Exec=/foo/bar/baz.sh
Type=Application
"#;

        assert_eq!(file_contents, expected_file_contents);
    }

    #[test]
    fn should_create_path_to_desktop_file() {
        let path = PathBuf::from("/foo/bar/baz.sh");

        let desktop_entry = ApplicationDesktopEntry::create_for(&path, None);

        let home = PathBuf::from("/home/thomas");

        let path = desktop_entry.get_path(home);

        assert_eq!(
            path,
            PathBuf::from("/home/thomas/.local/share/applications/baz.sh.desktop")
        );
    }

    #[test]
    fn should_derive_program_name_from_executable() {
        let path = PathBuf::from("/foo/bar/baz.sh");

        let desktop_entry = ApplicationDesktopEntry::create_for(&path, None);

        let program_name = desktop_entry.get_program_name();

        assert_eq!(
            program_name,
            "baz.sh"
        );
    }

    #[test]
    fn should_override_name_if_present() {
        let path = PathBuf::from("/foo/bar/baz.sh");

        let desktop_entry = ApplicationDesktopEntry::create_for(&path, Some("baz"));

        let program_name = desktop_entry.get_program_name();

        assert_eq!(
            program_name,
            "baz"
        );
    }
}
