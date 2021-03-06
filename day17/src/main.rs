use std::time::Instant;


fn main() {
    let start = Instant::now();
    println!("Part 1: {}", part1(207, -115, 263, -63)); // 6555
    println!("Part 2: {}", part2(207, -115, 263, -63)); // 4973
    let elapsed = start.elapsed();
    println!("Elapsed: {}µs", elapsed.as_micros());
}


fn part1(x1: i32, y1: i32, x2: i32, y2: i32) -> usize {

    let mut vx = 0;
    loop {
        if x1 > 0 {
            if vx * (vx + 1) / 2 > x1 {
                break;
            }
        }
        vx += 1;
    }
    //println!("Vx = {}", vx);

    //println!("Y speeds from {} to {}", i32::min(y1.abs(), y2.abs()) + 1, i32::max(y1.abs(), y2.abs()));
    let mut vy_max: usize = 0;
    for vy0 in (i32::min(y1.abs(), y2.abs()) + 1)..=i32::max(y1.abs(), y2.abs()) {
        let mut vy = 0 - vy0;
        let mut yi = 0;
        while yi >= i32::min(y1, y2) {
            if yi <= i32::max(y1, y2) {
                //println!("vy = {} is a solution at y = {}", vy0, yi);
                vy_max = vy0 as usize;
                break;
            }
            vy -= 1;
            yi += vy;
        }
    }
    //println!("Vy = {}", vy_max);
    //println!("Ymax = {}", vy_max * (vy_max + 1) / 2);
    vy_max * (vy_max + 1) / 2
}

fn part2(x1: i32, y1: i32, x2: i32, y2: i32) -> usize {

    let mut solutions = 0;

    let xmin = i32::min(x1, x2);
    let xmax = i32::max(x1, x2);
    let ymin = i32::min(y1, y2);
    let ymax = i32::max(y1, y2);

    for vx0 in 0..(xmax + 1 / 2) {
        for vy0 in (ymax + 1)..(i32::max(y1.abs(), y2.abs())) {
            let mut x = 0;
            let mut y = 0;

            let mut vx = vx0;
            let mut vy = vy0;

            let mut step = 0;

            while y > ymin {
                x += vx;
                y += vy;
                step += 1;
                if xmin <= x && x <= xmax && ymin <= y && y <= ymax {
                    //println!("Solution v0 = ({}, {}) after step {}", vx0, vy0, step);
                    solutions += 1;
                    break;
                }
                vx -= i32::signum(vx);
                vy -= 1;
            }

        }
    }

    solutions += (xmax - xmin + 1) * (ymax - ymin + 1);
    solutions as usize
}


#[test]
fn test_part1a() {
    assert_eq!(part1(20, -10, 30, -5), 45);
}

#[test]
fn test_part2a() {
    assert_eq!(part2(20, -10, 30, -5), 112);
}
