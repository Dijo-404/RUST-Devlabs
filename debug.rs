// this is simple program to print debug info in rust

#[derive(Debug)]
struct Cool{ //rust reccommends using upper case for struct names
    x:i32,
    y:i32,
}
fn main(){
    let z=Cool{x:1,y:2};
    println!("{:?}",z);
}