use std::io;

const INPUT: &str = include_str!("input.txt");

fn main() -> io::Result<()> {
  // Stage 1: 1071734
  let mut count = 0;
  let mut gamma = 0;
  let mut bits: Vec<i32> = vec![0; 12];
  INPUT.lines().for_each(|l| {
    count += 1;
    l.chars().enumerate().for_each(|(i, bit)| {
      if bit == '1' { bits[i] += 1 }
    })
  });

  bits.iter().for_each(|bit| {
    gamma = gamma << 1;
    if *bit > (count / 2) { gamma = gamma | 1 }
  });

  let epsilon = gamma ^ 0b111111111111;
  let result = epsilon * gamma;

  println!("Stage 1: {}", result);

  // Stage 2: 6124992
  let mut diagnostics: Vec<usize> = INPUT.lines().map(|l| usize::from_str_radix(&l, 2).unwrap()).collect();
  let mut current_bit = 11;
  while diagnostics.len() > 1 {
    let usage = bit_usage(&diagnostics, current_bit);
    let bit = (usage * 2 >= diagnostics.len()) as usize;

    diagnostics = diagnostics.into_iter().filter(|l| (l >> current_bit) & 1 == bit).collect();
    current_bit -= 1;
  }
  let oxygen_rating = diagnostics[0];

  let mut diagnostics: Vec<usize> = INPUT.lines().map(|l| usize::from_str_radix(&l, 2).unwrap()).collect();
  let mut current_bit = 11;
  while diagnostics.len() > 1 {
    let usage = bit_usage(&diagnostics, current_bit);
    let bit = (usage * 2 < diagnostics.len()) as usize;

    diagnostics = diagnostics.into_iter().filter(|l| (l >> current_bit) & 1 == bit).collect();
    current_bit -= 1;
  }
  let scrubber_rating = diagnostics[0];

  let result = oxygen_rating * scrubber_rating;
  println!("Stage 2: {}", result);

  Ok(())
}

fn bit_usage(reports: &Vec<usize>, bit: i8) -> usize {
  reports.iter().fold(0, |acc, rec| acc + ((rec >> bit) & 1 == 1) as usize)
}
