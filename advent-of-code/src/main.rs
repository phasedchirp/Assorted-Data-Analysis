use std::collections::HashMap;

fn part_1(input: i32) -> i32 {
    if input == 1 {
        0
    } else {
        let outer = (0i32..).map(|x| (2*x+1, (2*x + 1).pow(2)))
            .skip_while(|x| x.1 < input).nth(0).unwrap();
        let pos = (outer.1 - input) % (outer.0 - 1); // distance from corner
        match outer.0/2 >= pos {
            true => outer.0 - 1 - pos,
            false => outer.0 - 1 - (pos/2)
        }
    }
}

// #[derive(Debug)]
// struct Pt {
//     x: i32,
//     y: i32,
//     val: i32
// }
//
// impl Pt {
//     fn new(x: i32, y: i32) -> Pt {
//         let np = Pt{x: x, y: y, val: 0}
//     }
//     fn get_neighbors(&self) -> Vec<(i32,i32)> {
//         vec![(1,1),(0,1),(1,-1),(1,0),(0,-1),(-1,1),(-1,0),(-1,-1)]
//             .iter().map(|(x,y)| (x+self.x, y+self.y))
//             .collect()
//     }
//
//     fn set_val(&mut self, world: HashMap<(i32,i32),Pt>) {
//         self.get_neighbors().iter()
//             .fold(0,|acc,x|{
//                 match world.get(x) {
//                     Some(z) => acc + z.val,
//                     None    => acc
//             }
//         })
//     }
// }
//
// fn part_2(input: &str) -> i32 {
//     (1i32..).fold(HashMap::new(),|x|)
// }

fn main() {
    assert!(part_1(1) == 0);
    assert!(part_1(12) == 3);
    assert!(part_1(23) == 2);
    assert!(part_1(1024) == 31);
    println!("{:?}", part_1(289326));

}
