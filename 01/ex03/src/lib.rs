fn largest_group<'a>(haystack: &'a [u32], needle: &[u32]) -> &'a [u32] {
    if haystack.is_empty() || needle.is_empty() {
        return &[];
    }
    let x: usize = haystack.len();
    let y: usize = needle.len();
    let mut start: usize = 0;
    let mut save_start: usize = 0;
    let mut count: usize = 0;
    let mut save: usize = 0;
    let mut j: usize = 0;
    while j < x {
        let mut i: usize = 0;
        while i < y {
            if needle[i] == haystack[j] {
                if count == 0 {
                    start = j;
                }
                count += 1;
                break;
            } else {
                i += 1;
            }
        }
        if count > save {
            save = count;
            save_start = start;
        } else {
            count = 0;
        }
        j += 1;
    }
    if save > 0 {
        return &haystack[save_start..save_start + save];
    }
    &[]
}

/*
fn main() {
    let haystack = [1, 2, 3, 2, 1];
    let result;
    let needle = [2, 3];
    result = largest_group(&haystack, &needle);
    assert_eq!(result, &[2, 3, 2]);
}

#[test]
#[cfg(test)]
fn test_lifetimes() {
    let haystack = [1, 2, 3, 2, 1];
    let result;

    {
        let needle = [2, 3];
        result = largest_group(&haystack, &needle);
    }

    assert_eq!(result, &[2, 3, 2]);
}
*/