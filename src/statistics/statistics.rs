use std::collections::HashMap;

fn mean(slice: &[isize]) -> f64 {
    let mut sum = 0;
    for each in slice {
        sum += each;
    }
    sum as f64 / slice.len() as f64
}

fn median(slice: &[isize]) -> f64 {
    let mut sl = slice.to_owned();
    sl.sort_unstable();
    let idx = sl.len() / 2;
    if sl.len() % 2 == 0 {
        (sl[idx - 1] + sl[idx]) as f64 / 2.0
    } else {
        sl[idx] as f64
    }
}

fn mode(slice: &[isize]) -> Vec<isize> {
    let mut map = HashMap::new();
    let mut vector: Vec<isize> = Vec::new();
    let mut max: usize = 0;
    for each in slice {
        let count = map.entry(each).or_insert(0);
        *count += 1;
        if *count > max {
            vector.clear();
            vector.push(*each);
            max = *count;
        } else if *count == max {
            vector.push(*each);
        }
    }
    vector.shrink_to_fit();
    vector
}

pub fn stats(slice: &[isize]) -> (f64, f64, Vec<isize>) {
    (mean(slice), median(slice), mode(slice))
}
