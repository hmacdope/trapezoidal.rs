use std::io;

fn main() {
    println!("Welcome to rust-int, a toy program for trapezoidal integration");

    let (xmin, xmax, npoints) = read_inputs();
    
    let integral = trapz(&three_x_squared, xmin, xmax, npoints);

    println!("integral value is {}",integral);

}

// toy functions to integrate
fn two_x(x: f64) -> f64 {
    x * 2.0
}

fn ident(x: f64) -> f64 {
    x 
}

fn three_x_squared(x: f64) -> f64 {
    3.0*x*x
}

fn read_inputs() -> (f64, f64, i32) {
    println!("enter xmin value");
    let mut xmin =  String::new();
    io::stdin().read_line(&mut xmin).expect("failed to read");
    let xmin: f64 = xmin.trim().parse().expect("must be numeric");
    println!("enter xmax value");
    let mut xmax =  String::new();
    io::stdin().read_line(&mut xmax).expect("failed to read");
    let xmax: f64 = xmax.trim().parse().expect("must be numeric");
    println!("enter npoints value");
    let mut npoints =  String::new();
    io::stdin().read_line(&mut npoints).expect("failed to read");
    let npoints: i32 = npoints.trim().parse().expect("must be integer");
    (xmin, xmax, npoints)
}

fn read_int(val: &mut String) -> i32 {
    println!("enter value");
    io::stdin().read_line(val).expect("failed to read");
    let val: i32 = val.trim().parse().expect("must be numeric");
    val

}

fn gen_range(xmin: f64, xmax: f64, npoints: i32) -> Vec<f64>  {
    // generates a range of  npoints+1 values between xmin and xmax 
    if xmin >= xmax {
        panic!("xmin is >= xmax, panic!")
    }
    let elem: f64 = (xmax - xmin) / f64::from(npoints);
    let ninter: i32 = npoints + 1;
    let mut range = Vec::with_capacity(ninter.try_into().unwrap());
    for i in 0..ninter {
        let val = xmin + f64::from(i)*elem;
        range.push(val);
    }
    range
}

fn eval_on_range(range: &Vec<f64>, func: &dyn Fn(f64) -> f64) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::with_capacity(range.len());
    for point in range {
        result.push(func(*point));
    }
    result
}



fn trapz(func: &dyn Fn(f64) -> f64, xmin: f64, xmax: f64,  npoints: i32) -> f64 {
    // trapezoidal integration of function Fn over domain xmin -> xmax
    // with npoints quadrature points
    println!("xmin is {}", xmin);
    println!("xmax is {}", xmax);
    println!("npoints is {}", npoints);

    let range = gen_range(xmin, xmax, npoints);
    let end = range.len();

    let eval = eval_on_range(&range, &func);

    let eval_right = &eval[1..end];
    let eval_left = &eval[0..end-1];


    let dx = (xmax - xmin) / f64::from(npoints);

    println!("dx is {}\n", dx);
    let mut sum: f64 = 0.0;

    for (l, r) in eval_left.iter().zip(eval_right.iter()) {
        sum += dx* ((l + r)/2.0);
    }
    sum
}

