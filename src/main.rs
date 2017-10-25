//borrow checker/ ownership model
// closure / iterators

//fn matrix_vector_mult()

fn main() {
    //vecto(1200);
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
    //let inpu: Vec<f32> = vec![1.0, 2.0, 3.0, 5.0, 1.0, 2.0];
    //println!("{:?}", Mat6::matpro(mat, inpu));
    let mut testband = Band::new(40);
    println!("{}", testband.get_value());
    testband.mover(true);
    testband.overwrite(true);
    testband.print_band();
    let mut state_vector: Vec<bool>=vec![true ,true, false, false, true, true];
    organiser(state_vector,mat,testband)
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



fn vecto(mut zahl: i64) -> Vec<bool> {
    let mut ve = vec![];
    while zahl > 0 {
        ve.push((zahl % 2) == 1);
        zahl = zahl / 2;
    }
    ve.reverse();
    for i in 0..ve.len() {
        println!("{:?}", ve[i])
    }
    println!("{:?}", ve);
    ve
}

fn logic(v: Vec<i64>) -> Vec<i64> {
    let mut out: Vec<i64> = v.clone();
    if (v[0] == 1) | (v[1] == 1) {
        out.push(1);
    } else {
        out.push(0);
    }
    if (v[0] == 1) | (v[2] == 1) {
        out.push(1);
    } else {
        out.push(0);
    }

    out
}

fn vi64_to_vf32(v: Vec<i64>) -> Vec<f32> {
    let mut out = vec![];
    for k in v {
        out.push(k as f32);
    }
    out
}

fn dot(u1: Vec<f64>, u2: Vec<f64>) -> f64 {
    let out: f64 = u1.iter()
        .zip(u2.iter())
        .map(|(a, b)| a * b)
        .fold(0.0, |a, b| a + b);
    out
}

fn organiser(mut state_vector:Vec<bool>,mat:Mat6,mut band:Band)->(bool,Vec<bool>){
    let mut state_vector_float=vbool_to_vf32(state_vector);
    let mut new_state=Mat6::matpro(mat,state_vector_float);
    let mut direction:bool;
    let mut new_state_bool:Vec<bool>;
    (direction,new_state_bool)=after_matrix_cast(new_state);
    let mut terminated=band.mover(direction);
    if not terminated{
        println("{:?}",new_state_bool);
    }
    let vec=vec![band.get_value,new_state_bool[:]];
    (terminated,logic(vec))

}

struct Band {
    band: Vec<bool>,
    position: usize,
}

impl Band {
    pub fn get_value(&self) -> bool {
        self.band[self.position]
    }
    pub fn mover(&mut self, direction: bool) -> bool {
        if direction {
            self.position += 1;
        } else {
            self.position -= 1;
        }
        if self.position > self.band.len() {
            false
        } else {
            true
        }
    }
    pub fn overwrite(&mut self, new_value: bool) {
        self.band[self.position] = new_value;
    }
    pub fn new(number: i64) -> Band {
        Band {
            band: vecto(number),
            position: 0,
        }
    }
    pub fn print_band(&self) {
        println!("{:?}", self.band);
    }
}
