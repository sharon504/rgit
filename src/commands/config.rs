use crate::errors::RgitError;
use ini::Ini;
use std::path::Path;

#[derive(Debug)]
pub struct Config {
    _user_name: String,
    _user_email: String,
}

impl Config {
    pub fn load(file: &Path) -> Result<Self, RgitError> {
        if !file.exists() {
            return Ok(Self {
                _user_name: String::new(),
                _user_email: String::new(),
            });
        }

        let conf = Ini::load_from_file(file).map_err(|_| RgitError::ConfigError)?;

        let user_name = conf
            .section(Some("User"))
            .and_then(|s| s.get("name"))
            .unwrap_or("")
            .to_string();

        let user_email = conf
            .section(Some("User"))
            .and_then(|s| s.get("email"))
            .unwrap_or("")
            .to_string();

        Ok(Self {
            _user_name: user_name,
            _user_email: user_email,
        })
    }

    pub fn set(&self, file: &Path) -> Result<(), RgitError> {
        let mut conf = Ini::new();
        conf.with_section(Some("User"))
            .set("name", &self._user_name)
            .set("email", &self._user_email);
        conf.write_to_file(file)?;
        Ok(())
    }

    pub fn user_name(&mut self, user_name: String) -> Result<&mut Self, RgitError> {
        self._user_name = user_name;
        Ok(self)
    }

    pub fn user_email(&mut self, user_email: String) -> Result<&mut Self, RgitError> {
        self._user_email = user_email;
        Ok(self)
    }

    pub fn get_user_name(&self) -> Result<&str, RgitError> {
        Ok(&self._user_name)
    }

    pub fn get_user_email(&self) -> Result<&str, RgitError> {
        Ok(&self._user_email)
    }
}
