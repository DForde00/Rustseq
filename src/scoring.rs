pub fn optional_scoring((a, b, c): (i32, i32, i32)) -> (i32, i32, i32) {
    if (a, b, c) == (0, 0, 0) {
        let x = None;
        let scores = match x {
            None => (1, -1, -1),
            Some((x, y, z)) => (x, y, z),
        };
        return scores
    } else {
        let x = Some((a, b, c));
        let scores = match x {
            None => (1, -1, -1),
            Some((x, y, z)) => (x, y, z),
        };
        return scores
    }
}
