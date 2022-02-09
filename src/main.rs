use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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

// funcao que converte as coordenadas de um vetor do espaco referente a area de desenho
// (veja detalhamento nos comentarios da funcao draw_line) para o espaco referente a imagem
// propriamente dita.
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
        matrix: vec![bg_color; (&w * &h) as usize], //cria um array linear com w * h posições
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


// funcao que desenha uma linha conectando os pontos representados pelos vetores v1 e v2,
// com a cor especificada. A area de desenho compreende o retangulo formado pelos seguintes
// cantos, independente da dimensao real da imagem em pixels:
//
//   (-1,  1): canto superior esquerdo
//   ( 1,  1): canto superior direito
//   ( 1, -1): canto inferior direito
//   (-1, -1): canto inferior esquerdo
//
// Logo, espera-se que as coordenadas dos vetores estejam dentro destes limites

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

        let start = a.x.clone() as i64;
        let end = b.x.clone() as i64;

        for i in start..end{
            x = i.clone() as f64;
            //if x > b.x.clone() { break }

            y = ((&x - &a.x) / &deltaX) * (&b.y - &a.y) + &a.y;
            x = (&x + 1.).round();

            //implementar set_pixel
            set_pixel(image, x, y, color.clone());

        }

    }
    else {
        if p1.y < p2.y { a = p1.clone() }
        else { a = p2.clone() }

        if(p1.y < p2.y){ b = p2.clone() }
        else { b = p1.clone() }

        let start = a.x.clone() as i64;
        let end = b.x.clone() as i64;

        for j in start..end{
            y = j.clone() as f64;
            //if y <= b.y { break }

            x = ((&y - &a.y) / &deltaY) * (&b.x - &a.x) + &a.x;
            y = (&y + 1.).round();

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
        m: vec![0.; (&n * &m) as usize],
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

fn invert3x3(mat: &Matrix) -> Option<Matrix>{

    let det = det3x3(mat);

    let mut inverse: Matrix = create_matrix(3, 3);

    if ZERO!(det, f64) {
        println!("Singular Matrix");
        return None;
    }

    for i in 0..(&inverse.lin -1) {
        for j in 0..(&inverse.col -1) {

            inverse.m.push(cofactor3x3(mat, i, j) / &det)
        }

    }

    return Some(inverse);
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//                                                                                                                //
// Funcoes relacionadas ao tipo Matrix que precisam ser implementadas para o programa funcionar da forma esperada //
//                                                                                                                //
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////



////////////////////////////////////////////////////////////////////////////
//                                                                        //
// Programa principal. Le um arquivo de entrada com a seguinte estrutura: //
//                                                                        //
//	IMAGE_WIDTH IMAGE_HEIGHT BG_COLOR                                 //
//	OBSERVER_X, OBSERVER_Y, DIRECTION_X, DIRECTION_Y                  //
//	N_SHAPES                                                          //
//	N_VERTICES_SHAPE0 X_0 Y_0 X_1 Y_1 ... X_(N-1) Y_(N-1)             //
//	N_VERTICES_SHAPE1 X_0 Y_0 X_1 Y_1 ... X_(N-1) Y_(N-1)             //
//	N_VERTICES_SHAPE2 X_0 Y_0 X_1 Y_1 ... X_(N-1) Y_(N-1)             //
//	...                                                               //
//	<DRAW_COMMAND_0>                                                  //
//	<DRAW_COMMAND_1>                                                  //
//	<DRAW_COMMAND_2>                                                  //
//	...                                                               //
//	END                                                               //
//                                                                        //
// Sendo que cada linha referente a um comando de desenho pode ser:       //
//                                                                        //
//	DRAW_SHAPE SHAPE_ID COLOR ROTATION SCALE T_X T_Y                  //
//                                                                        //
// OU                                                                     //
//                                                                        //
//	DRAW_SHAPE_BASE SHAPE_ID COLOR E1_X E1_Y E2_X E2_Y T_X T_Y        //
//                                                                        //
// E gera uma imagem a partir das configurações e comandos especificados. //
//                                                                        //
////////////////////////////////////////////////////////////////////////////

fn fn_main(argv: [&'static str; 3]) -> i64 {

    ////////////////
    //            //
    // Variaveis: //
    //            //
    ////////////////

    let command: &'static str;
    let input_file_name: &'static str;
    let output_file_name: &'static str;

    let width: i64;     // largura da imagem a ser gerada
    let height: i64;    // altura da imagem a ser gerada
    let background_color: i64;      // cor de fundo da imagem a ser gerada

    let n_shapes: i64;

    let observer: Vector;
    let direction: Vector;
    let mut v: Vector;

    let mut shapes: Vec<Vector> = Vec::new();

    //let mut path: Path;

    let mut img: Image;

    ///////////////////////////////////////////
    //                                       //
    // Programa principal propriamente dito: //
    //                                       //
    ///////////////////////////////////////////

   /* if (!assert_eq!(argv.len(), 3)) {
        println!("uso: % <input_file_name> <output_file_name>");
        return 1;
    }*/

    input_file_name = &argv[1];
    output_file_name = &argv[2];

    // abertura do arquivo de entrada, e leitura dos parametros fixos (parametros da imagem e do observador, quantidade de shapes):

    let path: &Path = Path::new(input_file_name);
    let feedback = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Não foi possivel abrir o arquivo {}", feedback),
        Ok(file) => file,
    };



    return 0;
}

fn main() {
    println!("Hello, world!");

    let mut image: Image = create_image(16, 16, 255);

    let vetor1: Vector = create_vector(-1., -1.);
    let vetor2: Vector = create_vector(0.4, 0.33);

    draw_line(&mut image, &vetor1, &vetor2, 0);

    let mut matrix: Matrix = create_matrix(4, 4);


}
