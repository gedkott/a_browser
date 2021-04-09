pub type Layout = Vec<(f32, f32, char)>;

pub fn layout(body: &str, width: f32, height: f32, scroll: f32) -> Layout {
    let mut display_list = vec![];
    let hstep = 10f32;
    let vstep = 10f32;
    let mut cursor_x = 15f32;
    let mut cursor_y = 0f32;
    body.chars().for_each(|c| {
        if cursor_y > scroll + height {
            return;
        }
        if cursor_y + vstep > scroll + height {
            return;
        }
        display_list.push((cursor_x, cursor_y, c));
        if c == '\n' {
            cursor_y = cursor_y + (vstep * 2.0);
            cursor_x = 0f32;
        }

        if cursor_x >= width - hstep {
            cursor_y = cursor_y + vstep;
            cursor_x = 0f32;
        }
        cursor_x = cursor_x + hstep;
    });
    display_list
}
