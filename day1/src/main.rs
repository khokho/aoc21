use anyhow::Result;

fn main() -> Result<()> {
    let data = include_str!("input");
    let depths: Vec<usize> = data.lines().map(|s|s.parse().unwrap()).collect();
    let agg: Vec<usize> = depths.windows(3).map(|s|s.iter().sum()).collect();
    let ans: usize = agg.windows(2).map(|w|(w[0]<w[1]) as usize).sum();
    println!("result = {}", ans);
    Ok(())
}
