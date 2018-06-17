pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut sp: Vec<(usize, usize)> = Vec::new();

    if input.len() == 0 || input[0].len() == 0 {
        return sp;
    }

    let mut candidates: Vec<(usize, usize)> = Vec::new();
    for ridx in 0..input.len() {
        let mut x_max = 0;
        for max in &input[ridx] {
            if *max > x_max {
                x_max = *max;
            }
        }

        for cidx in 0..input[ridx].len() {
            if input[ridx][cidx] == x_max {
                candidates.push((ridx, cidx));
            }
        }
    }

    for (xrow, xcol) in &candidates {
        let candidate_value = input[*xrow][*xcol];
        let mut valid_sp = true;
        for row in 0..input.len() {
            if row == *xrow {
                continue;
            }
            let current_value = input[row][*xcol];
            if current_value < candidate_value {
                valid_sp = false;
                break;
            }
        }
        if valid_sp {
            sp.push((*xrow, *xcol));
        }
    }

    sp
}
