fn main() {
    let arr = [7, 5, 3, 2]; // unsorted array
    let mut is_sorted = false; // checks if array is sorted

    while !is_sorted {
        // while not sorted
        is_sorted = true; // assume it is sorted
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                // but if it's still unsorted
                is_sorted = false; // change back to false
                break;
            }
        }

        if !is_sorted {
            // any time now...
            waitForMiracle();
        }
    }
}

fn waitForMiracle() {
    // we pray :prayer_emoji:
}
