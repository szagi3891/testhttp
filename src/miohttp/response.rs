#[derive(Debug)]
pub struct Response {
    pub message : String,
}

impl Response {
    
    //from_text(numer, str)
    
    //np. from_text(400, "błąd parsowania")
	
	pub fn from_string(mess: String) -> Response {
		Response {
			message : mess
		}
	}
	
	pub fn as_bytes(&self) -> &[u8] {
		self.message.as_bytes()
	}
}

