use std::collections::HashSet;

fn realloc(old: Vec<u32>) -> Vec<u32> {
    let i =  old.iter().enumerate().fold((0,old[0]),|acc,x|{
            if x.1 > &acc.1 {
                (x.0,*x.1)
            } else {
                acc
            }
        }).0;
    let mut pool = old[i];
    let mut new = old.clone();
    new[i] = 0;
    for ind in (0..old.len()).cycle().skip(i+1){
        if pool == 0 {
            break;
        }
        new[ind] += 1;
        pool -= 1;
    }
    new
}

fn part_1(input: &str) -> u32 {
    let mut vals = input.split_whitespace()
                    .map(|x| x.parse::<u32>().unwrap())
                     .collect::<Vec<u32>>();
    let mut configs = HashSet::new();
    loop {
        vals = realloc(vals);
        let new = configs.insert(vals.clone());
        if !new {
            break;
        }
    }
    configs.len() as u32 + 1
}

fn part_2(){
    unimplemented!();
}

fn main() {
    let input = "5	1	10	0	1	7	13	14	3	12	8	10	7	12	0	6";
    assert!(part_1("0 2 7 0") == 5);
    println!("{:?}", part_1(input));
}
