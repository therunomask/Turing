//borrow checker/ ownership model
// closure / iterators
use std::ops;
//fn matrix_vector_mult()

fn main() {
    let mut mat = Mat64 {
        dat: [
            [0., 1., 1., 1., 0., 0.],
            [0., 0., 1., 0., 1., -1.],
            [2., 2., 1., 1., -2., -1.],
            [-2., -1., -1., 0., 1., 2.],
        ],
    };
    mat = onerun(5, mat);
    mat.print();


    /*let mut testband = Band::new(667);
    print!("The Band is: ");
    testband.print_band();
    let mut state_vector: Vec<bool> = vec![true, true, false, false, true, true];
    let mut terminated = false;
    while terminated == false {
        let (terminatedbla, state_vectorbla) = organiser(&state_vector, &mat, &mut testband);
        state_vector = state_vectorbla;
        terminated = terminatedbla;
    }*/
}




struct Mat66 {
    dat: [[f32; 6]; 6],
}

struct Mat64 {
    dat: [[f32; 6]; 4],
}

impl Mat66 {
    pub fn new() -> Mat66 {
        Mat66 {
            dat: [
                [0., 0., 0., 0., 0., 0.],
                [0., 0., 0., 0., 0., 0.],
                [0., 0., 0., 0., 0., 0.],
                [0., 0., 0., 0., 0., 0.],
                [0., 0., 0., 0., 0., 0.],
                [0., 0., 0., 0., 0., 0.],
            ],
        }
    }
    pub fn vec_to_mat(vec: &Vec<f32>, vec2: &Vec<f32>) -> Mat66 {
        let mut out = Mat66::new();
        for i in 0..6 {
            for j in 0..6 {
                out.dat[i][j] = vec[i] * vec2[j];
            }
        }
        out
    }
    pub fn print(&self) {
        for i in 0..6 {
            for j in 0..6 {
                print!("{} ", self.dat[i][j]);
            }
            print!("\n");
        }
    }
}



impl Mat64 {
    pub fn new() -> Mat64 {
        Mat64 {
            dat: [
                [0., 0., 0., 0., 0., 0.],
                [0., 0., 0., 0., 0., 0.],
                [0., 0., 0., 0., 0., 0.],
                [0., 0., 0., 0., 0., 0.],
            ],
        }
    }
    pub fn print(&self) {
        for i in 0..4 {
            for j in 0..6 {
                print!("{} ", self.dat[i][j]);
            }
            print!("\n");
        }
    }
    pub fn matpro(a: &Mat64, b: Vec<f32>) -> Vec<f32> {
        let mut out: Vec<f32> = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        for i in 0..4 {
            for j in 0..6 {
                out[i] += a.dat[i][j] * b[j];
            }
        }
        out
    }
    pub fn mult_on_u(&self, U: &Mat66) -> Mat64 {
        let mut out = Mat64::new();
        for i in 0..4 {
            for p in 0..6 {
                for k in 0..6 {
                    out.dat[i][p] += self.dat[i][k] * U.dat[k][p];
                }
            }
        }
        out
    }
    pub fn heavy(&self) -> Mat64 {
        let mut out = Mat64::new();
        for i in 0..4 {
            for p in 0..6 {
                if self.dat[i][p] > 0.0 {
                    out.dat[i][p] = 1.0;
                } else {
                    out.dat[i][p] = 0.0;
                }
            }
        }
        out
    }
    pub fn vec_to_mat(vec: &Vec<f32>, vec2: &Vec<f32>) -> Mat64 {
        let mut out = Mat64::new();
        for i in 0..4 {
            for j in 0..6 {
                out.dat[i][j] = vec[i] * vec2[j];
            }
        }
        out
    }
    pub fn minus(mat1: &Mat64, mat2: &Mat64) -> Mat64 {
        let mut out = Mat64::new();
        for i in 0..4 {
            for p in 0..6 {
                out.dat[i][p] = mat1.dat[i][p] - mat2.dat[i][p];
            }
        }
        out
    }
    pub fn plus(mat1: &Mat64, mat2: &Mat64) -> Mat64 {
        let mut out = Mat64::new();
        for i in 0..4 {
            for p in 0..6 {
                out.dat[i][p] = mat1.dat[i][p] + mat2.dat[i][p];
            }
        }
        out
    }
}


impl ops::Add<Mat64> for Mat64 {
    type Output = Mat64;
    fn add(self, right: Mat64) -> Mat64 {
        let mut out = Mat64::new();
        for i in 0..4 {
            for p in 0..6 {
                out.dat[i][p] = self.dat[i][p] + right.dat[i][p];
            }
        }
        out
    }
}

impl ops::Sub<Mat64> for Mat64 {
    type Output = Mat64;
    fn sub(self, right: Mat64) -> Mat64 {
        let mut out = Mat64::new();
        for i in 0..4 {
            for p in 0..6 {
                out.dat[i][p] = self.dat[i][p] - right.dat[i][p];
            }
        }
        out
    }
}

impl ops::Sub<Mat66> for Mat66 {
    type Output = Mat66;
    fn sub(self, right: Mat66) -> Mat66 {
        let mut out = Mat66::new();
        for i in 0..6 {
            for p in 0..6 {
                out.dat[i][p] = self.dat[i][p] - right.dat[i][p];
            }
        }
        out
    }
}

impl ops::Mul<f32> for Mat64 {
    type Output = Mat64;
    fn mul(self, right: f32) -> Mat64 {
        let mut out = Mat64::new();
        for i in 0..4 {
            for p in 0..6 {
                out.dat[i][p] = right * self.dat[i][p];
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
    ve
}

fn logic(v: Vec<bool>) -> Vec<bool> {
    if v.len() != 4 {
        panic!("The Vector should have length 4!");
    }
    let mut out: Vec<bool> = v.clone();
    if (v[0] == true) | (v[1] == true) {
        out.push(true);
    } else {
        out.push(false);
    }
    if (v[0] == true) | (v[2] == true) {
        out.push(true);
    } else {
        out.push(false);
    }

    out
}


fn vbool_to_vf32(v: &Vec<bool>) -> Vec<f32> {
    let mut out = vec![];
    for k in v {
        if k == &true {
            out.push(1.0);
        } else {
            out.push(0.0);
        }
    }
    out
}

fn after_matrix_cast(v: Vec<f32>) -> (bool, Vec<bool>) {
    let mut right: bool;
    if v[0] > 0.5 {
        right = true;
    } else {
        right = false;
    }
    let mut out = vec![false, false, false];
    if v[1] >= v[2] {
        if v[1] >= v[3] {
            out[0] = true;
        } else {
            out[2] = true;
        }
    } else {
        if v[2] >= v[3] {
            out[1] = true;
        } else {
            out[2] = true;
        }
    }
    (right, out)
}

fn success(zahl: i64, result: Vec<bool>) -> bool {
    let mut index = 0;
    for state in 0..result.len() {
        if result[state] {
            index = state;
        }
    }
    zahl % 3 == index as i64
}


fn dot(u1: Vec<f64>, u2: Vec<f64>) -> f64 {
    let out: f64 = u1.iter()
        .zip(u2.iter())
        .map(|(a, b)| a * b)
        .fold(0.0, |a, b| a + b);
    out
}

fn organiser(
    mut state_vector: &Vec<bool>,
    mat: &Mat64,
    mut testband: &mut Band,
) -> (bool, Vec<bool>) {
    //0 bandwert 1,2,3 zustand der turing machine 5,6 logic darauf angewendet
    let mut state_vector_float = vbool_to_vf32(&state_vector);
    let mut new_state = Mat64::matpro(&mat, state_vector_float);
    let mut direction: bool;
    let mut new_state_bool: Vec<bool>;
    let (direction, new_state_bool) = after_matrix_cast(new_state);
    let mut terminated = testband.mover(direction);
    let mut new_state_return: Vec<bool>;
    if terminated == true {
        print!("The final states are: ");
        println!("{:?}", new_state_bool);
        new_state_return = new_state_bool;
    } else {
        let vec = vec![
            testband.get_value(),
            new_state_bool[0],
            new_state_bool[1],
            new_state_bool[2],
        ];
        new_state_return = logic(vec);
    }

    (terminated, new_state_return)
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
            if self.position == 0 {
                self.position = self.band.len();
            } else {
                self.position -= 1;
            }
        }
        if self.position < self.band.len() {
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

fn boolvec_to_f32(uvec: &Vec<bool>) -> Vec<f32> {
    let mut out = vec![];
    for i in uvec {
        out.push((*i as i32) as f32);
    }
    out
}

fn onerun(number: i64, mat: Mat64) -> Mat64 {
    let mut return_mat = Mat64::new();
    let mut update_success = Mat64::new();
    let mut update_fail = Mat64::new();
    let mut testband = Band::new(number);
    let mut state_vector: Vec<bool> = vec![true, true, false, false, true, true];
    let mut terminated = false;
    let mut n = 0.;
    let mut toolong = false;
    while terminated == false {
        println!("{}", n);
        n += 1.;
        let U = Mat66::vec_to_mat(
            &boolvec_to_f32(&state_vector),
            &boolvec_to_f32(&state_vector),
        );
        let mu = mat.mult_on_u(&U);
        let theta = Mat64::minus(
            &mu,
            &Mat64::vec_to_mat(&vec![0.5, 0.5, 0.5, 0.5], &boolvec_to_f32(&state_vector)),
        ).heavy();
        update_success = Mat64::plus(&Mat64::minus(&update_success, &mu), &theta);
        update_fail = update_fail - mu +
            Mat64::vec_to_mat(&vec![1.0, 1.0, 1.0, 1.0], &boolvec_to_f32(&state_vector)) -
            theta;
        let (terminatedbla, state_vectorbla) = organiser(&state_vector, &mat, &mut testband);
        state_vector = state_vectorbla;
        terminated = terminatedbla;
        if n > 10.0 * (testband.band.len() as f32) {
            terminated = true;
            toolong = true;
        }
    }
    println!("{:?}", state_vector);
    let mut suc = success(
        number,
        vec![state_vector[0], state_vector[1], state_vector[2]],
    );
    println!("{}", suc);
    if toolong {
        suc = false;
    }
    if !suc {
        return_mat = mat + update_fail * (0.5 / n);
    } else {
        return_mat = mat + update_success * (0.5 / n);
    }
    return_mat
}
