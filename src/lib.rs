// and outgoing http response
pub struct HTTPRes {
    // line one, space seperated in order
    HTTP_version: f64,
    Status_code: u64,
    Reason_Phrase: String,

    // all of the headers
    headers: Vec<String>,
    
    // the response body
    body: String
}

impl HTTPRes {
    pub fn make_response (
        http_version: f64,
        status_code: u64,
        reason_phrase: String,
        headers: Vec<String>,
        body: String
    ) -> HTTPRes
    {
        HTTPRes {
            HTTP_version: http_version,
            Status_code: status_code,
            Reason_Phrase: reason_phrase,
            headers,
            body,
        }
    }

    pub fn format_response_string (&self) -> String {
        let space: String = String::from(" ");
        let crlf: String = "\r\n".to_string();

        // line 1
        let http_ver = self.HTTP_version.to_string();
        let status_code = self.Status_code.to_string();
        let reason_phrase = &self.Reason_Phrase;

        // line 2
        let header_str = self.headers.join(" ");

        // line 3
        let body = &self.body;

        http_ver + &space + &status_code + &space + reason_phrase + &crlf + &header_str + &crlf + body
    }
}
