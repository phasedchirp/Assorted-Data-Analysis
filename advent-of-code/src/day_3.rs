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

#[derive(Debug)]
struct Pt {
    x: i32,
    y: i32,
    val: i32
}

impl Pt {
    fn new(x: i32, y: i32) -> Pt {
        Pt{x: x, y: y, val: 0}
    }
    fn get_neighbors(&self) -> Vec<(i32,i32)> {
        vec![(1,1),(0,1),(1,-1),(1,0),(0,-1),(-1,1),(-1,0),(-1,-1)]
            .iter().map(|z| (z.0+self.x, z.1+self.y))
            .collect()
    }

    fn set_val(&mut self, world: &HashMap<(i32,i32),Pt>) {
        let v = self.get_neighbors().iter()
                    .fold(0,|acc,x|{
                        match world.get(x) {
                            Some(z) => acc + z.val,
                            None    => acc
                        }});
        self.val = v;
    }
}

fn coords(n: i32) -> (i32,i32) {
    let (i,l,m) = (0i32..).map(|x| (x,2*x+1, (2*x + 1).pow(2)))
        .skip_while(|x| x.2 < n).nth(0).unwrap();
    let pos = (m - 1 - n) / (l - 1); // which side of the square?
    let d = (m - n) % (l-1); // distance from nearest corner?
    let sign = ((-1i32).pow(pos as u32),((-1i32).pow((pos+1) as u32)));
    sign
    // (l/2 + (pos % 2)*d,l/2 + (1 + pos % 2)*d)
}

fn part_2(input: i32) -> i32 {
    let mut world = HashMap::new();
    let mut result = 0;
    for x in (1i32..) {
        let shell = (x,2*x+1, (2*x + 1).pow(2));
        let coords = coords(x);
        let mut z = Pt::new(shell.0,shell.1);
        z.set_val(&world);
        if z.val > input {
            let result = z.val;
            break;
        }
    }
    result
}

fn main() {
    assert!(part_1(1) == 0);
    assert!(part_1(12) == 3);
    assert!(part_1(23) == 2);
    assert!(part_1(1024) == 31);
    println!("{:?}", part_1(289326));
    println!("26: {:?}", coords(26));
    println!("34: {:?}", coords(34));
    println!("41: {:?}", coords(41));
    println!("5: {:?}", coords(5));
    println!("6: {:?}", coords(6));
    println!("7: {:?}", coords(7));
    println!("8: {:?}", coords(8));
    println!("9: {:?}", coords(9));
    println!("18: {:?}", coords(18));
    println!("17: {:?}", coords(17));
    println!("16: {:?}", coords(16));
    println!("15: {:?}", coords(15));
    println!("14: {:?}", coords(14));
    println!("13: {:?}", coords(13));
    println!("12: {:?}", coords(12));
    println!("11: {:?}", coords(11));
    println!("10: {:?}", coords(10));
}
