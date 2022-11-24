mod ml_data;
use crate::ml_data::{Node, read_ml_json};
use std::path::Path;
use std::string::String;
use std::collections::HashMap;
use core::cmp::max;


fn main() {

    println!("hello world");

    let path1 = Path::new("resources/1663154348643_8ZGUJJLLWV/ml_data/1663154348643_8ZGUJJLLWV.json");
    let path2 = Path::new("resources/1663154348643_8ZGUJJLLWV/ml_data/1663154348643_8ZGUJJLLWV.json");

    let data1 = read_ml_json(&path1);
    let data2 = read_ml_json(&path2);

    let data2_node_num:usize  = data2.element_statistics.nodes.len();
    println!("length data2 is {}",data2_node_num);


    //1.- find node with XX=true

    let mut xx_node:i32 = 0;    
    let mut count = 0;
    for node_i in &data1.element_statistics.nodes{
            //println!("Checking node {}",node_i.i);
            if node_i.a.contains_key("XX"){
                //xx_node = node_i.i.clone();
                //println!("{}",count);
                xx_node = count;
            }
            count +=1;
    }

    println!("XX at index {} i value is {}",xx_node,&data1.element_statistics.nodes[xx_node as usize].i); //xx node is

    //2.- clone node to temporary hash map
    let mut reference_hashmap:HashMap<String,String>= HashMap::new();
    let keepout:[String;5]= [ //keys to ignore
        "XX".to_string(),
        "WH".to_string(),
        "HT".to_string(),
        "TP".to_string(),
        "LT".to_string()
        ];
    
    //copies into reference hashmap
    for (k, v) in data1.element_statistics.nodes[xx_node as usize].a.iter() {
        //println!("k={}, v={}", k, v);
        if !keepout.contains(k){
            println!("AA");
            reference_hashmap.insert(k.to_string(),v.to_string()); //only keep keys that we care about    
        }
        println!("k={}, v={}", k, v);
        
    }
    
    //3.- compute ~correlation~ list

    let mut corr_vec: Vec<i64> = Vec::new(); //we must use a vector since arrays require constant length to be defined at compilation time
    let mut coinc_count:i64 = 0;

    for node_i in &data2.element_statistics.nodes{
        //checks all the nodes in data 2
        coinc_count = 0;

        //compare with reference hashmap
        for (k, v) in reference_hashmap.iter() {
            if node_i.a.contains_key(k){
                if &node_i.a[k]==v{
                    coinc_count += 1;
                }
            }
        }

        corr_vec.push(coinc_count.clone());
    }

    println!("{:?}",corr_vec);
    

    let mut max_val:i64 = -1;
    for i in corr_vec.iter(){
        if (*i>max_val){
            max_val = *i;
        }
    }

    println!("max value {}",&max_val);

    //new vector with normalized values
    let mut corr_vec_norm: Vec<f64> = Vec::new(); //we must use a vector since arrays require constant length to be defined at compilation time
    let mut tmp:f64 =0.0;
    for i in corr_vec.iter(){
        tmp = (i.clone() as f64)/(max_val as f64);
        corr_vec_norm.push(tmp);
    }

    println!("Normalized correlation list:");
    println!("{:?}",corr_vec_norm);


}

fn node_correlation(node1: Node,node2: Node) ->f64{
    todo!();
}

fn consume_s(s: String) -> usize {
    s.len()
}

enum State<T, Q = i32> {
    ON(Q),
    OFF(T),
}



mod topology {
    pub struct Point {
        x: f64,
        y: f64,
    }

    pub struct Square {
        p_tl: Point,
        p_br: Point,
    }

    impl Point {
        pub fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }

        pub fn x(&self) -> f64 {
            self.x
        }
        pub fn y(&self) -> f64 {
            self.y
        }
    }

    impl Square {
        pub fn new(p1: Point, p2: Point) -> Self {
            let min_x = p1.x.min(p2.x);
            let max_x = p1.x.max(p2.x);
            let min_y = p1.y.min(p2.y);
            let max_y = p1.y.max(p2.y);
            Self {
                p_tl: Point::new(min_x, min_y),
                p_br: Point::new(max_x, max_y),
            }
        }
        
        pub fn lower(&self) -> &Point {
            &self.p_tl
        }
        pub fn upper(&self) -> &Point {
            &self.p_br
        }

        pub fn height(&self) -> f64 {
            f64::abs(self.p_br.y - self.p_tl.y)
        }
        pub fn width(&self) -> f64 {
            f64::abs(self.p_br.x - self.p_tl.x)
        }

        pub fn area(&self) -> f64 {
            self.width() * self.height()
        }

        pub fn erosion(&mut self, d: f64) {
            self.p_tl.x = self.p_tl.x + d;
            self.p_tl.y = self.p_tl.y + d;
            self.p_br.x = self.p_br.x - d;
            self.p_br.y = self.p_br.y - d;
        }

        pub fn dilate(&mut self, d: f64) -> () {
            self.p_tl.x = self.p_tl.x - d;
            self.p_tl.y = self.p_tl.y - d;
            self.p_br.x = self.p_br.x + d;
            self.p_br.y = self.p_br.y + d;
        }
        pub fn intersection(&self, other: &Self) -> Self {
            let x1 = self.p_tl.x.max(other.p_tl.x);
            let y1 = self.p_tl.y.max(other.p_tl.y);
            let x2 = self.p_br.x.min(other.p_br.x);
            let y2 = self.p_br.y.min(other.p_br.y);

            if x1 > x2 || y1 > y2 {
                Square::new(Point::new(0.0, 0.0), Point::new(0.0, 0.0))
            } else {
                Square::new(Point::new(x1, y1), Point::new(x2, y2))
            }
        }

        pub fn union(&self, other: &Self) -> Self {
            let x1 = self.p_tl.x.min(other.p_tl.x);
            let y1 = self.p_tl.y.min(other.p_tl.y);
            let x2 = self.p_br.x.max(other.p_br.x);
            let y2 = self.p_br.y.max(other.p_br.y);
            Square::new(Point::new(x1, y1), Point::new(x2, y2))
        }

        pub fn dilate_x(&mut self, d: f64) -> () {
            let wth = self.width() * 0.5 * d;
            let mid_x = (self.p_br.x - self.p_tl.x) * 0.5;
            self.p_tl.x = mid_x - wth;
            self.p_br.x = mid_x + wth;
        }

        pub fn dilate_y(&mut self, d: f64) -> () {
            let wth = self.height() * 0.5 * d;
            let mid_y = (self.p_tl.y - self.p_tl.y) * 0.5;
            self.p_tl.y = mid_y + wth;
            self.p_br.y = mid_y - wth;
        }

        pub fn erosion_x(&mut self, d: f64) -> () {
            self.dilate_x(1.0 / d);
        }

        pub fn erosion_y(&mut self, d: f64) -> () {
            self.dilate_y(1.0 / d);
        }

        pub fn has_point(&self, p1: &Point) -> bool {
            todo!()
        }

        pub fn has_square(&self, sq: &Square) -> bool {
            todo!()
        }

        pub fn manhattan_distance(&self, sq: &Square) -> f64 {
            todo!()
        }
    }
}
#[cfg(test)]
mod test {
    use crate::topology::{Point, Square};

    #[test]
    fn point_test() {
        let p = Point::new(10.0, 10.0);
        assert_eq!(p.y(), 10.0);
        assert_eq!(p.x(), 10.0);
    }

    #[test]
    fn sq_test() {
        let p1: Point = Point::new(0.0, 0.0);
        let p2: Point = Point::new(1.0, 2.0);
        let sq: Square = Square::new(p1, p2);
        //assert_eq!(sq.area(),2.0);
        assert!(sq.lower().x() < sq.upper().x());
        assert!(sq.lower().y() < sq.upper().y());
    }

    #[test]
    fn dilate_test() {
        let p1: Point = Point::new(0.0, 2.0);
        let p2: Point = Point::new(1.0, 0.0);
        let mut sq: Square = Square::new(p1, p2);

        sq.dilate(2.0);

        assert_eq!(sq.area(), 30.0);
    }

    #[test]
    fn intersection_test_corner() {
        let s1: Square = Square::new(Point::new(0.0, 0.0), Point::new(3.0, 3.0));
        let s2: Square = Square::new(Point::new(1.0, 1.0), Point::new(4.0, 4.0));
        let s3 = s1.intersection(&s2);
        assert_eq!(s3.lower().x(), 1.0);
        assert_eq!(s3.lower().y(), 1.0);
        assert_eq!(s3.upper().x(), 3.0);
        assert_eq!(s3.upper().y(), 3.0);
    }

    #[test]
    fn intersection_test_cross() {
        let s1: Square = Square::new(Point::new(0.0, 0.0), Point::new(3.0, 3.0));
        let s2: Square = Square::new(Point::new(1.0, -1.0), Point::new(2.0, 4.0));
        let s3 = s1.intersection(&s2);
        assert_eq!(s3.lower().x(), 1.0);
        assert_eq!(s3.lower().y(), 0.0);
        assert_eq!(s3.upper().x(), 2.0);
        assert_eq!(s3.upper().y(), 3.0);
    }

    #[test]
    fn intersection_test_out() {
        let s1: Square = Square::new(Point::new(0.0, 0.0), Point::new(3.0, 3.0));
        let s2: Square = Square::new(Point::new(5.0, 5.0), Point::new(10.0, 10.0));
        let s3 = s1.intersection(&s2);
        assert_eq!(s3.lower().x(), 0.0);
        assert_eq!(s3.lower().y(), 0.0);
        assert_eq!(s3.upper().x(), 0.0);
        assert_eq!(s3.upper().y(), 0.0);
    }

    #[test]
    fn union_test_cross() {
        let s1: Square = Square::new(Point::new(0.0, 0.0), Point::new(3.0, 3.0));
        let s2: Square = Square::new(Point::new(1.0, -1.0), Point::new(2.0, 4.0));
        let s3 = s1.union(&s2);
        assert_eq!(s3.lower().x(), 0.0);
        assert_eq!(s3.lower().y(), -1.0);
        assert_eq!(s3.upper().x(), 3.0);
        assert_eq!(s3.upper().y(), 4.0);
    }
    fn erosion_test() {
        let p1: Point = Point::new(0.0, 4.0);
        let p2: Point = Point::new(4.0, 0.0);
        let mut sq: Square = Square::new(p1, p2);

        sq.erosion(0.5);

        assert_eq!(sq.area(), 9.0);
    }
}
