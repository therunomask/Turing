//borrow checker/ ownership model
// closure / iterators

//fn matrix_vector_mult()

fn main() {
    
    println!("Hello, world!");
    print!("gehts?");
    vecto(120);
    let mat = Mat6 {
        dat: [
            [0., 1., 1., 1., 0., 2.],
            [0., 0., 1., 0., 1., 1.],
            [2., 2., 1., 1., -2., -1.],
            [-2., -1., -1., 0., 1., 2.],
            [0., 0., 0., 0., 0., 0.],
            [0., 0., 0., 0., 0., 0.],
        ],
    };
    //mat.print();
    let inpu: Vec<i64> = vec![1, 1, 0, 1];
    println!("{:?}", Mat6::matpro(mat,Vi64ToVf32( logic( inpu)) ));



}

struct Mat6 {
    dat: [[f32; 6]; 6],
}

impl Mat6 {
    pub fn print(self) {
        for i in 0..6 {
            for j in 0..6 {
                print!("{} ", self.dat[i][j]);
            }
            print!("\n");
        }
    }
    pub fn matpro(a: Mat6, b: Vec<f32>) -> Vec<f32> {
        let mut out: Vec<f32> = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        for i in 0..6 {
            for j in 0..6 {
                out[i] += a.dat[i][j] * b[j];
            }
        }
        out
    }
}



fn vecto(mut zahl: i64) -> Vec<i64> {
    let mut ve = vec![];
    while zahl > 0 {
        ve.push(zahl % 2);
        zahl = zahl / 2;
    }
    ve.reverse();
    for i in 0..ve.len() {
        println!("{:?}", ve[i])
    }
    println!("{:?}", ve);
    ve
}

fn logic(v :Vec<i64>) ->Vec<i64>{
    if v.len()!=4 {
        panic!("The Vector should have length 4!");
    }
    let mut out: Vec<i64> = v.clone();
    if (v[0]==1)|(v[1]==1) {
        out.push(1);
    } else{
        out.push(0);
    }
     if (v[0]==1)|(v[2]==1) {
        out.push(1);
    } else{
        out.push(0);
    }
    
    out
}

fn Vi64ToVf32 (v : Vec<i64>)->Vec<f32>{
    let mut out=vec![];
    for k in v {
        out.push(k as f32);
    }
    out
}

fn dot(u1: Vec<f64>,u2:Vec<f64>) -> f64 {
    let out: f64=u1.iter().zip(u2.iter()).map(|(a, b)| a * b).fold(0.0,|a,b| a+b);
    out
}