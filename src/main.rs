const MAX_VERTICES: i64 = 100;
const MAX_VALUE: i64 = 255;

const DRAW_SHAPE: &str = "DRAW_SHAPE";
const DRAW_SHAPE_BASE: &str = "DRAW_SHAPE_BASE";

const END: &str = "END";

const SMALL: f64 = 0.000001;

macro_rules! ZERO {
    ($a:expr, i64) => { i64::abs($a) < SMALL };
    ($a:expr, f64) => { f64::abs($a) < SMALL };
}

macro_rules! EQUAL {
    ($a:expr, $b:expr, i64) => { i64::abs($a - $b) < SMALL };
    ($a:expr, $b:expr, f64) => { f64::abs($a - $b) < SMALL };
}

macro_rules! TERNARY{
    ($test:expr => $true:expr, $false:expr) => {
        if($test){ $true }
        else{ $false }
    }
}

#[derive(Copy, Clone)]
struct Vector {
    x: f64,
    y: f64,
}

struct Shape {
    n: i64,
    vertices: [Vector],
}

struct Image {
    width: i64,
    height: i64,
    matrix: Vec<i64>,
}

struct Matrix {
    m: Vec<f64>,
    lin: i64,
    col: i64,
}

fn create_vector(x: f64, y: f64) -> Vector{

    Vector{
        x: x,
        y: y,
    }
}

fn convert(v: &Vector, width: f64, height: f64) -> Vector {
    let n_x: f64 = (v.x.clone() + 1.0) * width / 2.0;
    let n_y: f64 = (1.0 - v.y.clone()) * height / 2.0;

    Vector{
        x: n_x,
        y: n_y,
    }
}

fn create_image(w: i64, h: i64, bg_color: i64) -> Image{
    //let lin: i64;
    //let col: i64;
    let mut image: Image = Image {
        height: h,
        width: w,
        matrix: vec![bg_color, &w * &h], //cria um array linear com w * h posições
    };



    return image;
}

// função que pinta um pixel da imagem com a cor especificada
fn set_pixel(image: &mut Image, x: f64, y: f64, color: i64){
    let col: i64 = f64::round(x) as i64;
    let lin: i64 = f64::round(y) as i64;

    if lin < 0 || col < 0 || col >= image.width || lin >= image.height { return; }

    image.matrix.remove(((&col * &lin) - 1) as usize);

    image.matrix.insert(((&col * &lin) - 1) as usize, color);
}

fn draw_line(image: &mut Image, v1: &Vector, v2: &Vector, color: i64){

    let a: Vector; let b: Vector;
    let mut x: f64; let mut y: f64;

    let p1: Vector = convert(v1, image.width.clone() as f64,
                             image.height.clone() as f64);

    let p2: Vector = convert(v2, image.width.clone() as f64,
                            image.height.clone() as f64);

    let deltaX: f64 = f64::abs(&p1.x - &p2.x);
    let deltaY: f64 = f64::abs(&p1.y - &p2.y);

    if deltaX >= deltaY {
        if p1.x < p2.x { a = p1.clone() }
        else { a = p2.clone() }

        if(p1.x < p2.x){ b = p2.clone() }
        else { b = p1.clone() }



        while let mut x = a.x.clone(){
            if x <= b.x { break }

            y = ((x - &a.x) / &deltaX) * (&b.y - &a.y) + &a.y;
            x = (x + 1.).round();

            //implementar set_pixel
            set_pixel(image, x, y, color.clone());

        }

    }
    else {
        if p1.y < p2.y { a = p1.clone() }
        else { a = p2.clone() }

        if(p1.y < p2.y){ b = p2.clone() }
        else { b = p1.clone() }

        while let mut y = a.y.clone(){
            if y <= b.y { break }

            x = ((y - &a.y) / &deltaY) * (&b.x - &a.x) + &a.x;
            y = (y + 1.).round();

            //implementar set_pixel
            set_pixel(image, x, y, color.clone());

        }
    }
}

/////////////////////////////////////////
//                                     //
// Funcoes relacionadas ao tipo Matrix //
//                                     //
/////////////////////////////////////////


fn create_matrix(n: i64, m: i64) -> Matrix {


    let mut matrix: Matrix = Matrix{
        m: vec![0., (&n * &m) as f64],
        lin: n,
        col: m,
    };

    return matrix;

}

fn create_matrix_with_values(n: i64, m: i64, valores: Vec<f64>) -> Matrix{

    let mut mat: Matrix = create_matrix(n, m);
    mat.m = valores;

    return mat;
}


fn copy_matrix(mat: &mut Matrix) -> Matrix{

    let mut copy: Matrix = create_matrix(mat.lin.clone(), mat.col.clone());

    copy.m = mat.m.clone();

    return copy;
}


fn create_identity(n: i64) -> Matrix{

    let mut mat: Matrix = create_matrix(n.clone(), n.clone());

    let mut var1: i64 = 0;
    let mut var2: i64 = 0;
    for x in 0..(&n * &n - 1){

        if var1 != var2 { mat.m.push(0.) }
        else { mat.m.push(1.) }

        //prestar atenção nessa parte -> manipulando matriz como um vetor
        if var1 >= (&n - 1) && var2 >= (&n -1){ break; }
        else if var2 >= (&n -1) { var2 = 0; var1 = var1 + 1; }
        else { var1 = var1 + 1; var2 = var2 + 1; }

    }

    return mat;
}

fn det2x2(mat: &Matrix) -> f64{

    if mat.lin == 2 && mat.col == 2 {

        return (mat.m[0] * mat.m[3]) - (mat.m[1] * mat.m[2]);
    }

    println!("Invalid matrix size!");

    return 0.0;
}

fn cofactor3x3(mat: &Matrix, i: i64, j: i64) -> f64{
    let cof: f64;

    let mut values: Vec<f64> = Vec::with_capacity(4);

    let mut k: i64;

    let mut tmp: Matrix;

    if mat.lin == 3 && mat.col == 3 {
        k = 0;

        for lin in 0..(&mat.lin -1) {
            for col in 0..(&mat.col -1) {

                if lin != i && col != j {
                    let index = (lin * col) as usize;
                    values.push(mat.m[index]) }

            }
        }

        tmp = create_matrix_with_values(2, 2, values);

        cof = TERNARY!((i + j) % 2 == 0 => 1., -1.) * det2x2(&tmp);

        return cof;
    }

    println!("Invalid matrix size!");
    return 0.;
}

fn det3x3(mat: &Matrix) -> f64{

    if mat.lin == 3 && mat.col == 3 {
        return mat.m[0] * cofactor3x3(mat, 0, 0) + mat.m[1] * cofactor3x3(mat, 0, 1)
        + mat.m[2] * cofactor3x3(mat, 0, 2);
    }

    println!("Invalid matrix size!");
    return 0.;
}




fn main() {
    println!("Hello, world!");

    let mut image: Image = create_image(64, 64, 255);



}
