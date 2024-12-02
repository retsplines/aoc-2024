#[derive(PartialEq)]
pub enum UnsafeReason {
    Magnitude,
    Slope
}

#[derive(PartialEq)]
pub enum Safety {
    Safe,
    Unsafe(UnsafeReason)
}

pub fn check(levels: &Vec<i32>) -> Safety {

    let mut last_diff = 0;
    for window in levels.as_slice().windows(2) {

        let diff: i32 = window[0] - window[1];
        if diff.abs() < 1 || diff.abs() > 3 {
            // This line is unsafe due to difference size
            return Safety::Unsafe(UnsafeReason::Magnitude);
        }

        // Verify the direction is consistent
        // The first difference is not significant
        if 
            last_diff != 0 &&
            (last_diff > 0 && diff < 0) || (last_diff < 0 && diff > 0) 
        {
            // This line is unsafe due to direction change
            return Safety::Unsafe(UnsafeReason::Slope);
        }

        last_diff = diff;
    }

    return Safety::Safe;

}