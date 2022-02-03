
use crate::serve::Page;

pub struct LoginPage {
    pub page: Page,
}

impl LoginPage {

    pub fn new() -> LoginPage {

        let body = "<form action=\"/login\">\
                            <label for=\"fname\">Name:</label><br>\
                            <input type=\"text\" id=\"fname\" name=\"fname\" pattern=\"[A-Za-z]\" required><br>\
                            <input type=\"submit\" value=\"Get Link\">\
                        </form>";

        return LoginPage{ page: Page{ title: "Main".to_string(), 
                                     body: body.to_string() } }
    }

    pub fn new2() -> LoginPage {

        let body = "<form action=\"/login\">\
                            <label for=\"fname\">Name:</label><br>\
                            <input type=\"text\" id=\"fname\" name=\"fname\" value=\"**Invalud Name**\"><br>
                            <input type=\"submit\" value=\"Get Link\">\
                        </form>\
                        \
                        <p style=\"color:red\">Names must contain of only alphanumeric characters</p>";

        return LoginPage{ page: Page{ title: "Main".to_string(), 
                                     body: body.to_string() } }
    }
}