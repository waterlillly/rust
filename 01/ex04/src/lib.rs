fn sort_boxes(boxes: &mut [[u32; 2]]) {
    if boxes.is_empty() {
        panic!("Invalid");
    }
    let mut swapped = true;
	let mut x = 100;
    while swapped && x > 0 {
        swapped = false;
        for i in 0..boxes.len() - 1 {
            if boxes[i][0] < boxes[i + 1][0] || boxes[i][1] < boxes[i + 1][1] {
                boxes.swap(i, i + 1);
                swapped = true;
            }
        }
		x -= 1;
    }
	if x == 0 {
		panic!("Invalid");
	}
}

/*
fn main() {
    let mut boxes = [[3, 3], [4, 3], [1, 0], [5, 7], [3, 3]];
    sort_boxes(&mut boxes);
    assert_eq!(boxes, [[5, 7], [4, 3], [3, 3], [3, 3], [1, 0]]);
}
*/