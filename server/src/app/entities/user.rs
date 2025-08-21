pub struct User {
    id: Option<String>,
    email: String,
    username: String,
    password: String,
}

impl User {
    pub fn new(email: String, username: String, password: String) -> Self {
        Self {
            id: None,
            email,
            username,
            password,
        }
    }

    // Getters
    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    // Setters
    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }
}
