pub struct Region {
    name: String,
    //(x, y)
    coors: Vec<(f64, f64)>
}

impl Region{
    pub fn new(name: String, coors: Vec<(f64, f64)>) -> Region{
        Region { name, coors }
    }

    pub fn from_json(data: String) -> Region {
        
    }

    pub fn overlapping(&self, other: &Region) -> f64 {
        let mut num_points_inside: i64 = 0;

        for coor in other.coors.iter() {
            if self.point_inside(coor) {
                num_points_inside += 1;
            }
        }

        //ratio of the points of the other polygon inside the self polygon
        return (num_points_inside as f64) / (other.coors.len() as f64)
    }


    pub fn point_inside(&self, coor: &(f64, f64)) -> bool {
        let mut intersecting_lines: i64 = 0;

        //iterates through each pair of adjacent points
        for (point_1, point_2) in (0..self.coors.len()).into_iter().map(|num| {
            if num == self.coors.len() - 1 {
                return (&self.coors[num], &self.coors[0])
            }
            else {
                return (&self.coors[num], &self.coors[num + 1])
            }
        }) {
            //ensure that the given coordinate is vertically between the 2 points
            if (point_1.1 >= coor.1 && point_2.1 <= coor.1) || (point_2.1 >= coor.1 && point_1.1 <= coor.1) {
                //gradient of the line between the adjacent points
                let m: f64 = (point_2.1 - point_1.1) / (point_2.0 - point_1.0);

                //y intercept of the line between the adjacent points
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

}