use std::io;

fn main() {
    println!("Welcome to rust-int");

    let mut xmin = String::new();
    let mut xmax = String::new();
    let xmin: f64 = read_val(&mut xmin);
    let xmax: f64 = read_val(&mut xmax);

    let npoints: i32 = 100;
    let range = gen_range(xmin, xmax, npoints);
    println!("{:?}",range);
}

fn read_val(val: &mut String) -> f64 {
    println!("enter value");
    io::stdin().read_line(val).expect("failed to read");
    let val: f64 = val.trim().parse().expect("must be numeric");
    val

}

fn gen_range(xmin: f64, xmax: f64, npoints: i32) -> Vec<f64>  {
    if xmin >= xmax {
        panic!("xmin is LTE xmax")
    }
    println!("xmin is {}", xmin);
    println!("xmax is {}", xmax);
    let diff: f64 = xmax - xmin;
    let elem: f64 = diff / f64::from(npoints);

    let mut range = Vec::with_capacity(npoints.try_into().unwrap());
    for i in 0..npoints {
        let val = xmin + f64::from(i)*elem;
        range.push(val);
    }
    range
}

fn eval_on_range(range: Vec<f64>, func: &dyn Fn(f64) -> f64) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::with_capacity(range.len());
    for point in range {
        result.push(func(point));
    }
    result
}
