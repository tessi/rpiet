/*
 * A BlockExit saves potential exit positions for a block of codels.
 * Depending on the DP and CC pointers, blocks have different exit points, which
 * we cache in this structure instead of calculating them on the fly.
 */

#[derive(Debug)]
pub struct BlockExit {
    pub exits: [[(usize, usize); 2]; 4],
}

impl BlockExit {
    pub fn from_coords(coords: &Vec<(usize, usize)>) -> BlockExit {
        let top_coords = topmost_coords(coords);
        let right_coords = rightmost_coords(coords);
        let bottom_coords = bottommost_coords(coords);
        let left_coords = leftmost_coords(coords);

        BlockExit {
            exits: [
                [top_coords[0], top_coords[1]],
                [right_coords[0], right_coords[1]],
                [bottom_coords[0], bottom_coords[1]],
                [left_coords[0], left_coords[1]],
            ],
        }
    }
}

fn topmost_coords(coords: &Vec<(usize, usize)>) -> [(usize, usize); 2] {
    let first = coords.first().unwrap();
    let iter = coords.iter();
    iter.fold([*first, *first], |[a, b], &coord| {
        if coord.1 < a.1 {
            [coord, coord]
        } else if coord.1 == a.1 {
            if coord.0 < a.0 {
                [coord, b]
            } else if coord.0 > b.0 {
                [a, coord]
            } else {
                [a, b]
            }
        } else {
            [a, b]
        }
    })
}

fn bottommost_coords(coords: &Vec<(usize, usize)>) -> [(usize, usize); 2] {
    let first = coords.first().unwrap();
    let iter = coords.iter();
    iter.fold([*first, *first], |[a, b], &coord| {
        if coord.1 > a.1 {
            [coord, coord]
        } else if coord.1 == a.1 {
            if coord.0 < a.0 {
                [coord, b]
            } else if coord.0 > b.0 {
                [a, coord]
            } else {
                [a, b]
            }
        } else {
            [a, b]
        }
    })
}

fn rightmost_coords(coords: &Vec<(usize, usize)>) -> [(usize, usize); 2] {
    let first = coords.first().unwrap();
    let iter = coords.iter();
    iter.fold([*first, *first], |[a, b], &coord| {
        if coord.0 > a.0 {
            [coord, coord]
        } else if coord.0 == a.0 {
            if coord.1 < a.1 {
                [coord, b]
            } else if coord.1 > b.1 {
                [a, coord]
            } else {
                [a, b]
            }
        } else {
            [a, b]
        }
    })
}

fn leftmost_coords(coords: &Vec<(usize, usize)>) -> [(usize, usize); 2] {
    let first = coords.first().unwrap();
    let iter = coords.iter();
    iter.fold([*first, *first], |[a, b], &coord| {
        if coord.0 < a.0 {
            [coord, coord]
        } else if coord.0 == a.0 {
            if coord.1 < a.1 {
                [coord, b]
            } else if coord.1 > b.1 {
                [a, coord]
            } else {
                [a, b]
            }
        } else {
            [a, b]
        }
    })
}
