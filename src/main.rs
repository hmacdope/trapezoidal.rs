use std::io;

fn main() {
    println!("Welcome to rust-int");

    let mut xmin = String::new();
    let mut xmax = String::new();
    let mut npoints = String::new();

    let xmin: f64 = read_val(&mut xmin);
    let xmax: f64 = read_val(&mut xmax);
    let npoints: i32 = read_int(&mut npoints);
    
    let integral = trapz(&three_x_squared, xmin, xmax, npoints );
    println!("integral value is {}",integral);

}

fn read_val(val: &mut String) -> f64 {
    println!("enter value");
    io::stdin().read_line(val).expect("failed to read");
    let val: f64 = val.trim().parse().expect("must be numeric");
    val

}

fn read_int(val: &mut String) -> i32 {
    println!("enter value");
    io::stdin().read_line(val).expect("failed to read");
    let val: i32 = val.trim().parse().expect("must be numeric");
    val

}

fn gen_range_augmented(xmin: f64, xmax: f64, npoints: i32) -> Vec<f64>  {
    if xmin >= xmax {
        panic!("xmin is LTE xmax, panic!")
    }
    let diff: f64 = xmax - xmin;
    let elem: f64 = diff / f64::from(npoints);
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

fn double(x: f64) -> f64 {
    x * 2.0
}

fn ident(x: f64) -> f64 {
    x 
}

fn three_x_squared(x: f64) -> f64 {
    3.0*x*x
}

fn trapz(func: &dyn Fn(f64) -> f64, xmin: f64, xmax: f64,  npoints: i32) -> f64 {
    println!("xmin is {}", xmin);
    println!("xmax is {}", xmax);
    println!("npoints is {}", npoints);

    let range = gen_range_augmented(xmin, xmax, npoints);
    let end = range.len();

    let range_right = &range[1..end];
    let range_left = &range[0..end-1];
    // println!("{:?}", range_left);
    // println!("{:?}", range_right);


    let eval = eval_on_range(&range, &func);

    let eval_right = &eval[1..end];
    let eval_left = &eval[0..end-1];
    // println!("{:?}", eval_left);
    // println!("{:?}", eval_right);


    let dx = (xmax - xmin) / f64::from(npoints);
    println!("dx is {}\n", dx);
    let mut sum: f64 = 0.0;

    for (l, r) in eval_left.iter().zip(eval_right.iter()) {
        sum += dx* ((l + r)/2.0);
    }
    sum
}

