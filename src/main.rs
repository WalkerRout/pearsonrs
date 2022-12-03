
type ResBErr<T> = Result<T, Box<dyn std::error::Error + 'static>>;

fn load_data(path: &str) -> ResBErr<Vec<Vec<f64>>> {
  use std::io::BufRead;
  
  let file = std::fs::File::open(path)?;
  let reader = std::io::BufReader::new(file);

  let mut vectors = vec![];
  
  for line in reader.lines() {
    let vec = line?
      .as_str()
      .split(' ')
      .map(|x| x.parse::<f64>().unwrap())
      .collect::<Vec<f64>>();
    vectors.push(vec);
  }
  
  Ok(vectors)
}

fn mean(vec: &Vec<f64>) -> f64 {
  vec.iter().copied().sum::<f64>() / vec.len() as f64
}

fn pearson_coefficient(xs: &Vec<f64>, ys: &Vec<f64>) -> f64 {
  let x_mean = mean(&xs);
  let y_mean = mean(&ys);
  
  let top: f64 = xs.iter().zip(ys.iter())
    .map(|(x, y)| {
      (x - x_mean) * (y - y_mean)
    }).sum();
  
  let bot: f64 = {
    let x_part: f64 = xs.iter()
      .map(|x| {
        (x - x_mean).powf(2.0)
      }).sum();
    
    let y_part: f64 = ys.iter()
      .map(|y| {
        (y - y_mean).powf(2.0)
      }).sum();
    
    x_part * y_part
  };
  
  top / bot.sqrt()
}

fn main() -> ResBErr<()> {
  let test = load_data("data.txt")?;

  let xs = &test[0];
  let ys = &test[1];
  
  println!("Pearson coefficient: {}", pearson_coefficient(xs, ys));
  
  Ok(())
}
