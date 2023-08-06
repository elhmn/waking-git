//collision box color
pub const COL_COLOR: &str = "26a64166";
pub const DEBUG: bool = false;

pub fn get_col_color() -> String {
    if DEBUG {
        COL_COLOR.to_string()
    } else {
        "00000000".to_string()
    }
}
