use std::borrow::Borrow;

fn bits_to_usize<T>(bits: T) -> usize
where
    T: IntoIterator,
    T::Item: Borrow<bool>,
{
    bits.into_iter()
        .fold(0, |acc, b| (acc << 1) | *b.borrow() as usize)
}

#[derive(Debug)]
struct Counter {
    ones: Vec<usize>,
    zeroes: Vec<usize>,
}

impl Counter {
    fn new(n_bits: usize) -> Self {
        Self {
            ones: vec![0; n_bits],
            zeroes: vec![0; n_bits],
        }
    }
    fn add_item(mut self, bits: &Vec<bool>) -> Self {
        bits.iter().enumerate().for_each(|(i, v)| match v {
            true => self.ones[i] += 1,
            false => self.zeroes[i] += 1,
        });
        self
    }

    fn to_gamma(&self) -> usize {
        let bits: Vec<bool> = self
            .ones
            .iter()
            .zip(self.zeroes.iter())
            .map(|(t, f)| (t > f))
            .collect();
        bits_to_usize(bits)
    }

    fn to_epsilon(&self) -> usize {
        let bits: Vec<bool> = self
            .ones
            .iter()
            .zip(self.zeroes.iter())
            .map(|(t, f)| (t < f))
            .collect();
        bits_to_usize(bits)
    }
}

fn part_one(inputs: Vec<Vec<bool>>) {
    let res = inputs
        .iter()
        .fold(Counter::new(inputs[0].len()), Counter::add_item);
    dbg!(&res);
    println!("gamma {}, eps {}", res.to_gamma(), res.to_epsilon());
    println!("result is {}", res.to_gamma() * res.to_epsilon());
}

fn value_counts<T>(it: T) -> (usize, usize)
where
    T: Iterator<Item = bool>,
{
    it.fold((0, 0), |agg, b| (agg.0 + (!b) as usize, agg.1 + b as usize))
}

fn part_two(inputs: Vec<Vec<bool>>) {
    let mut oxyg = inputs.clone();
    let mut i = 0;
    while oxyg.len() > 1 {
        let cnts = value_counts(oxyg.iter().map(|v| v[i]));
        let cb = cnts.1 >= cnts.0;
        oxyg = oxyg.into_iter().filter(|v| v[i] == cb).collect();
        i += 1;
    }
    let oxygen_v = bits_to_usize(&oxyg[0]);
    drop(oxyg);

    let mut co2 = inputs.clone();
    let mut i = 0;
    while co2.len() > 1 {
        let cnts = value_counts(co2.iter().map(|v| v[i]));
        let cb = cnts.1 < cnts.0;
        co2 = co2.into_iter().filter(|v| v[i] == cb).collect();
        i += 1;
    }

    let co2_v = bits_to_usize(&co2[0]);
    drop(co2);

    println!("result is {}", oxygen_v * co2_v);
}

fn line_to_vec_bool(l: &str) -> Vec<bool> {
    return l
        .chars()
        .map(|b| match b {
            '0' => false,
            '1' => true,
            _ => panic!("unknown bit"),
        })
        .collect();
}

fn main() {
    let inputs: Vec<_> = include_str!("input")
        .lines()
        .map(line_to_vec_bool)
        .collect();
    part_one(inputs);
}
