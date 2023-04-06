use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from_string(s: &str) -> Result<Self, Error> {
        let mut parts = s.split(',');
        let x = parts
            .next()
            .ok_or(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Missing X coordinate",
            ))?
            .parse::<i32>()
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid X coordinate"))?;
        let y = parts
            .next()
            .ok_or(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Missing Y coordinate",
            ))?
            .parse::<i32>()
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid Y coordinate"))?;
        Ok(Point { x, y })
    }
}

struct LineSegment {
    start: Point,
    end: Point,
}

impl LineSegment {
    fn from_string(s: &str) -> Result<Self, Error> {
        let mut parts = s.split(" -> ");
        let start = Point::from_string(
            parts
                .next()
                .ok_or(Error::new(ErrorKind::InvalidInput, "Missing start point"))?,
        )?;
        let end = Point::from_string(
            parts
                .next()
                .ok_or(Error::new(ErrorKind::InvalidInput, "Missing end point"))?,
        )?;
        Ok(LineSegment { start, end })
    }

    fn get_min_x(&self) -> i32 {
        self.start.x.min(self.end.x)
    }

    fn get_max_x(&self) -> i32 {
        self.start.x.max(self.end.x)
    }

    fn get_min_y(&self) -> i32 {
        self.start.y.min(self.end.y)
    }

    fn get_max_y(&self) -> i32 {
        self.start.y.max(self.end.y)
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }
}

struct LineSegmentCollection {
    line_segments: Vec<LineSegment>,
}

impl LineSegmentCollection {
    fn from_input_file(file_path: &str) -> Result<Self, Error> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let line_segments: Vec<LineSegment> = reader
            .lines()
            .map(|line| LineSegment::from_string(&line?))
            .collect::<Result<Vec<LineSegment>, Error>>()?;
        Ok(LineSegmentCollection { line_segments })
    }

    fn iter(&self) -> impl Iterator<Item = &LineSegment> {
        self.line_segments.iter()
    }

    fn get_min_x(&self) -> i32 {
        self.iter().map(|ls| ls.get_min_x()).min().unwrap()
    }

    fn get_max_x(&self) -> i32 {
        self.iter().map(|ls| ls.get_max_x()).max().unwrap()
    }

    fn get_min_y(&self) -> i32 {
        self.iter().map(|ls| ls.get_min_y()).min().unwrap()
    }

    fn get_max_y(&self) -> i32 {
        self.iter().map(|ls| ls.get_max_y()).max().unwrap()
    }
}

struct PointGrid {
    grid: Vec<Vec<usize>>,
    offset_x: i32,
    offset_y: i32,
}

impl PointGrid {
    fn new(line_segments: &LineSegmentCollection) -> Self {
        let total_min_x = line_segments.get_min_x();
        let total_max_x = line_segments.get_max_x();
        let total_min_y = line_segments.get_min_y();
        let total_max_y = line_segments.get_max_y();
        let grid_width = total_max_x - total_min_x + 1;
        let grid_height = total_max_y - total_min_y + 1;
        PointGrid {
            grid: vec![vec![0; grid_width as usize]; grid_height as usize],
            offset_x: total_min_x,
            offset_y: total_min_y,
        }
    }

    fn draw_line_segment(&mut self, line_segment: &LineSegment) {
        let start_x = line_segment.start.x - self.offset_x;
        let end_x = line_segment.end.x - self.offset_x;
        let start_y = line_segment.start.y - self.offset_y;
        let end_y = line_segment.end.y - self.offset_y;
        let min_x = start_x.min(end_x);
        let max_x = start_x.max(end_x);
        let min_y = start_y.min(end_y);
        let max_y = start_y.max(end_y);
        if line_segment.is_vertical() {
            for y in min_y..=max_y {
                self.grid[y as usize][start_x as usize] += 1;
            }
        } else if line_segment.is_horizontal() {
            for x in min_x..=max_x {
                self.grid[start_y as usize][x as usize] += 1;
            }
        } else {
            // Assume the line segment is diagonal at 45 degrees
            let num_points = max_x - min_x + 1;
            for offset in 0..num_points {
                let x = {
                    let direction = if start_x < end_x { 1 } else { -1 };
                    start_x + offset * direction
                };
                let y = {
                    let direction = if start_y < end_y { 1 } else { -1 };
                    start_y + offset * direction
                };
                self.grid[y as usize][x as usize] += 1;
            }
        }
    }

    fn get_num_intersecting_points(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|&&x| x > 1).count())
            .sum()
    }
}

// Return the number of points where horizontal and vertical line segments intersect.
fn part1(line_segments: &LineSegmentCollection) -> usize {
    let mut point_grid = PointGrid::new(line_segments);
    for ls in line_segments
        .iter()
        .filter(|ls| ls.is_horizontal() || ls.is_vertical())
    {
        point_grid.draw_line_segment(ls);
    }
    point_grid.get_num_intersecting_points()
}

// Return the number of points where any two line segments intersect.
fn part2(line_segments: &LineSegmentCollection) -> usize {
    let mut point_grid = PointGrid::new(line_segments);
    for ls in line_segments.iter() {
        point_grid.draw_line_segment(ls);
    }
    point_grid.get_num_intersecting_points()
}

fn main() {
    let line_segments = LineSegmentCollection::from_input_file("day5/src/input.txt").unwrap();
    println!("Part 1: {}", part1(&line_segments));
    println!("Part 2: {}", part2(&line_segments));
}
