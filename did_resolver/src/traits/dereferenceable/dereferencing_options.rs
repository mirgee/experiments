type MediaType = String;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DIDDereferencingOptions {
    accept: Option<MediaType>,
}

impl DIDDereferencingOptions {
    pub fn new() -> Self {
        Self { accept: None }
    }

    pub fn set_accept(mut self, accept: String) -> Self {
        self.accept = Some(accept);
        self
    }

    pub fn accept(&self) -> Option<&String> {
        self.accept.as_ref()
    }
}
