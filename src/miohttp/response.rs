#[derive(Debug)]
pub struct Response {
    message : String,
}

impl Response {
    
    //from_text(numer, str)
    
    //np. from_text(400, "błąd parsowania")
	
	pub fn from_string(mess: String) -> Response {
		Response {
			message : mess
		}
	}
}

