#[derive(Debug, Clone)]
pub struct Avatar {
    id: String,
    user_id: String,
    file_key: String,
    mime_type: String,
}

impl Avatar {
    pub fn new(id: String, user_id: String, file_key: String, mime_type: String) -> Self {
        Self {
            id,
            user_id,
            file_key,
            mime_type,
        }
    }

    // Getters
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn file_key(&self) -> &str {
        &self.file_key
    }

    pub fn mime_type(&self) -> &str {
        &self.mime_type
    }

    // Setters
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_user_id(&mut self, user_id: String) {
        self.user_id = user_id;
    }

    pub fn set_file_key(&mut self, file_key: String) {
        self.file_key = file_key;
    }

    pub fn set_mime_type(&mut self, mime_type: String) {
        self.mime_type = mime_type;
    }
}
