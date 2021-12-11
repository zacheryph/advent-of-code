use std::io;

const INPUT: &str = include_str!("input.txt");

fn main() -> io::Result<()> {
    let mut lifespans: Vec<u64> = INPUT.split(',').map(|n| n.trim().parse().unwrap()).fold(
        vec![0 as u64; 9],
        |mut acc, n: usize| {
            acc[n] += 1;
            acc
        },
    );

    for _ in 0..256 {
        lifespans =
            lifespans
                .iter()
                .enumerate()
                .fold(vec![0 as u64; 9], |mut acc, (days, units)| {
                    match days {
                        1.. => acc[days - 1] += units,
                        0 => {
                            acc[6] += units;
                            acc[8] += units;
                        }
                        _ => panic!(),
                    }
                    acc
                })
    }

    // Stage 1 (0..80):  366057
    // Stage 2 (0..256): 1653559299811
    println!("Stage: {}", lifespans.iter().fold(0, |acc, v| acc + v));

    Ok(())
}
