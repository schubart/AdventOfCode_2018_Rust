#![warn(clippy::pedantic)]

type Scalar = i32;
type Point = (Scalar, Scalar);
type Vector = (Scalar, Scalar);

/// ```
/// assert!(day10::part1());
/// ```
pub fn part1() -> bool {
    let mut points: Vec<(Point, Vector)> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let parts = line.split(&['<', '>', ','][..]).collect::<Vec<_>>();
            (
                (
                    parts[1].trim().parse::<Scalar>().unwrap(),
                    parts[2].trim().parse::<Scalar>().unwrap(),
                ),
                (
                    parts[4].trim().parse::<Scalar>().unwrap(),
                    parts[5].trim().parse::<Scalar>().unwrap(),
                ),
            )
        })
        .collect();

    let mut max = 20_000;
    loop {
	let min_y = points.iter().map(|p| p.0.1).min().unwrap();
	let max_y = points.iter().map(|p| p.0.1).max().unwrap();

	println!("height: {}", max_y - min_y);
	assert!(max_y - min_y > 9);

//	println!("{:?}", points[0]);
	points = points.iter().map(|(p, v)| ((p.0 + v.0, p.1 + v.1), *v)).collect();
	
	max -= 1;
	assert!(max > 0);
    }
    
    false
}
