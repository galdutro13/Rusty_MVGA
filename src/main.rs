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


fn copy_matrix(mat: Matrix) -> Matrix{

    let mut copy: Matrix = create_matrix(mat.lin, mat.col);

    copy.m = mat.m.clone();

    return copy;
}



fn main() {
    println!("Hello, world!");

    let mut image: Image = create_image(64, 64, 255);



}
