use aoc_core::read_file;
use std::collections::BinaryHeap;
use std::time::Instant;

fn calorie_heap(data: &String) -> BinaryHeap<u64> {
    let mut b_heap: BinaryHeap<u64> = BinaryHeap::new();

    data.lines().fold(0, |acc, x| {
        match x.trim() {
            "" => {
                b_heap.push(acc);
                0
            },
            num => {
                acc + num.parse::<u64>().unwrap()
            }
        }
    });
    b_heap
}

fn calorie_list(data: &String) -> Vec<u64> {
    let mut cal_list = Vec::with_capacity(250);
    let mut _sum = 0;
    data.lines().for_each(|x| {
        match x.trim() {
            "" => {
                cal_list.push(_sum);
                _sum = 0;
            },
            num => {
                _sum += num.parse::<u64>().unwrap();
            }
        }
    });
    cal_list.sort();
    cal_list
}

fn main() -> Result<(), std::io::Error> {
    let data = read_file(01).unwrap();
    let mut sol: (u64, u64) = (u64::MIN, u64::MIN);
    
    let time = Instant::now();
    let mut b_heap = calorie_heap(&data);
    println!("b_heap time: {} micros", time.elapsed().as_micros());

    sol.0 = b_heap.pop().unwrap();
    sol.1 = b_heap.pop().unwrap() + b_heap.pop().unwrap() + sol.0;
    println!("0: {}", sol.0);
    println!("1: {}", sol.1);
    let time = Instant::now();
    let mut list = calorie_list(&data);
    println!("list time: {} micros", time.elapsed().as_micros());
    println!("0: {}", list[list.len() - 1]);
    let mut x = 0;
    (0..3)
        .for_each(|_| x += list.pop().unwrap());
    println!("1: {}", x);
    Ok(())
}


#[test]
fn test_1() {
    let data = 
        "2832
        2108
        3082
        4328
        6843
        5121
        2869
        1366
        2358
        1680
        4980
        1161

        70000

        8026
        2154
        4242
        1023
        2744
        3162
        4093
        1150
        5397
        2738
        5657";
    let mut sol = u64::MIN;
    let sol = data.lines().fold(0, |acc, x| {
        println!("acc: {}, sol: {}", acc, sol);
        match x.trim() {
            "" => {
                if acc > sol {
                    sol = acc;
                }
                0
            },
            x => {
                acc + x.parse::<u64>().unwrap()
            }
        }
    });
    assert_eq!(sol, 70000);
}
