use std::cmp;

struct DPMatrix {
    v: Vec<usize>,
    rows: usize,
    cols: usize,
}
impl DPMatrix {
  fn new(n: usize, m: usize) -> Self {
    let mut matrix = DPMatrix {
        v: Vec::with_capacity(n*m),
        rows: n,
        cols: m,
    };
    matrix.v.resize(n*m, 0);
    for i in 0..n {
        matrix.v[i] = i;
        matrix.v[i*m] = i;
    }
    matrix
  }  
  fn at(&self, i: usize, j: usize) -> &usize {
    &self.v[i + j*self.cols]
  }

  fn compute(&mut self, x: Vec<i32>, y: Vec<i32>) {
    for i in 0..self.rows-1 {
        for j in 0..self.cols-1 {
            let a = self.at(i, j);
            let b = self.at(i, j+1);
            let c = self.at(i+1, j);
            let mut delta = 1;
            if x[i] == y[j] {
                delta = 0;
            }
            
            self.v[i+1 + (j+1)*self.cols] = cmp::min(a+delta, cmp::min(b+1, c+1));
            // self.at(i+1, j+1) = cmp::min(a+delta, cmp::min(b+1, c+1));
        }
    }
  }

}

fn main() {
    let x = Vec::from([0, 1, 2, 3, 0, 1, 1, 0]);//"ACGTACCA";
    let y = Vec::from([0, 3, 1, 3, 0, 1, 3, 0]);//"ATCTACTA";
    let n = x.len() + 1;
    let m = y.len() + 1;
    let mut matrix = DPMatrix::new(n, m);
    matrix.compute(x, y);
    for i in  0..n {
        for j in  0..m {
            print!("{}  ", matrix.at(i, j))
            
        }
        println!();
    }

    matrix.at(0,0) = 1;

    println!("Distance: {}", matrix.at(n-1,m-1));
}
