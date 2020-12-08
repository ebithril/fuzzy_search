use std::ffi::CString;

pub fn fuzzy_match_simple(pattern: &String, string: &String) -> bool {
    let c_pattern = CString::new(pattern.as_bytes()).unwrap();
    let c_string = CString::new(string.as_bytes()).unwrap();

    return fuzzy_match_simple_internal(c_pattern.as_ptr(), c_string.as_ptr());
}

pub fn fuzzy_match(pattern: &String, string: &String) -> (bool, i32) {
    return (fuzzy_match_simple(pattern, string), 1);
}

pub fn fuzzy_search(pattern: &String, strings: &Vec<String>, result: & mut Vec<String>) {
    let mut scores: Vec<i32> = Vec::new();

    for string in strings {
        let (matching, score) = fuzzy_match(pattern, string);

        if matching {
            let mut insert_pos = scores.len();
            for i in 0..scores.len() {
                if score > scores[i] {
                    insert_pos = i;
                    break;
                }
            }

            result.insert(insert_pos, string.to_string());
            scores.insert(insert_pos, score);
        }
    }
}

fn fuzzy_match_simple_internal(pattern: *const i8, string: *const i8) -> bool {
    let mut p = pattern;
    let mut s = string;

    unsafe {
        while *p != '\0' as i8 && *s != '\0' as i8 {
            if *p == *s {
                p = p.add(1);
            }

            s = s.add(1);
        }

        if *p == '\0' as i8 {
            return true;
        } else {
            return false;
        }
    }
}
