use crate::response;

pub fn lex(body: &response::Body) -> String {
    let mut in_angle = false;
    let mut in_body = false;
    let mut tag = "".to_string();
    let mut text_only = "".to_string();
    body.body_buffer.chars().for_each(|c| {
        if c == '<' {
            in_angle = true;
        } else if c == '>' {
            if tag == "body" {
                in_body = true;
            }
            if tag == "/body" {
                in_body = false;
            }
            in_angle = false;
            tag = "".to_string();
        } else {
            if in_angle {
                tag = format!("{}{}", tag, c).to_string();
            } else {
                if in_body {
                    text_only = format!("{}{}", text_only, c).to_string();
                }
            }
        }
    });

    text_only
}
