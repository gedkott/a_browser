pub type Layout = Vec<(i32, i32, char)>;

pub fn layout(body: &str, width: i32, height: i32, scroll: i32) -> Layout {
    let mut display_list = vec![];
    let hstep = 10;
    let vstep = 10;
    let mut cursor_x = 15;
    let mut cursor_y = 0;
    body.chars().for_each(|c| {
        if cursor_y > scroll + height {
            return;
        }
        if cursor_y + vstep > scroll + height {
            return;
        }
        display_list.push((cursor_x, cursor_y, c));
        if c == '\n' {
            cursor_y = cursor_y + (vstep * 2);
            cursor_x = 0
        }

        if cursor_x >= width - hstep {
            cursor_y = cursor_y + vstep;
            cursor_x = 0;
        }
        cursor_x = cursor_x + hstep;
    });
    display_list
}
