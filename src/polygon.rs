pub struct Polygon {
    points: Vec<(f64, f64)>
}

impl Polygon{
    pub fn new(points: Vec<(f64, f64)>) -> Polygon{
        Polygon { points }
    }

    pub fn point_inside(&self, coor: &(f64, f64)) -> bool {
        let mut intersecting_lines: i64 = 0;

        for i in 1..self.points.len() {
            let point_1: &(f64, f64) = &self.points[i-1];
            let point_2: &(f64, f64) = &self.points[i];

            if (point_1.1 >= coor.1 && point_2.1 <= coor.1) || (point_2.1 >= coor.1 && point_1.1 <= coor.1) {
                //gradient
                let m: f64 = (point_2.1 - point_1.1) / (point_2.0 - point_1.0);
                //y intercept
                let c: f64 = point_1.1 - (m * point_1.0);

                if coor.1 == (m * coor.0) + c {
                    intersecting_lines += 1;
                }
            }
        }

        //returns true if point is inside the polygon, 
        //i.e. a horizontal line from the coor intersects an odd number of the polygon's edges
        return (intersecting_lines % 2) == 1
    }

    pub fn overlapping(&self, other: &Polygon) -> f64 {
        let mut num_points_inside: i64 = 0;

        for coor in other.points.iter() {
            if self.point_inside(coor) {
                num_points_inside += 1;
            }
        }

        //ratio of points in the other polygon inside to outside the self polygon
        return (num_points_inside as f64) / (other.points.len() as f64)
    }
}