fn main() {
    let s = "hello";
    let anchor_col = 5;
    let cursor_col = 3;
    let start_col = cursor_col.min(anchor_col);
    let end_col = cursor_col.max(anchor_col);
    let result = &s[start_col..end_col];
    println!("start_col: {}, end_col: {}, result: {:?}", start_col, end_col, result);
}
