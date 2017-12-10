fn part_1(input: u32) -> u32 {
    if input == 1 {
        0
    } else {
        let inner = (0u32..).map(|x| (2*x+1, (2*x + 1).pow(2)))
            .take_while(|x| x.1 < input).last().unwrap();
        let outer = (inner.0+2, (inner.0+2).pow(2));
        let pos = (outer.1 - input) % (outer.0 - 1); // distance from cor
        // println!("{}: {:?}, {}, {}", input, outer, pos, outer.0/2);
        match outer.0/2 >= pos {
            true => outer.0 - 1 - pos,
            false => outer.0 - 1 - (pos/2)
        }
    }
}

fn part_2(input: &str) -> u32 {
    unimplemented!()
}

fn main() {
    assert!(part_1(1) == 0);
    assert!(part_1(12) == 3);
    assert!(part_1(23) == 2);
    assert!(part_1(1024) == 31);
    println!("{:?}", part_1(289326));

}
