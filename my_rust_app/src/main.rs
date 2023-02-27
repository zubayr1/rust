use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::env;

const NAME: &str = "sarah ulfat";
const AGE: u8 = 20;

struct Rectangle
{
    width: u32,
    height: u32
}

trait HasDouble
{
    fn isdouble(&self) -> bool;
}

impl HasDouble for Rectangle
{
    fn isdouble(&self) -> bool
    {
        if self.width > self.height
        {
            return true;
        }
        return false;
    }
}

impl Rectangle
{
    fn print_desc(&self)
    {
        println!("{}, {}", self.width, self.height);
    }

    fn issquare(&self) -> bool
    {
        if self.width == self.height
        {
            return true;
        }
        return false;
    }
    
}

fn rect()
{
    let myrect = Rectangle{width: 10, height: 5};

    myrect.print_desc();

    println!("{}",myrect.issquare());

    // println!("{}",Rectangle.isdouble());
}

struct Color<'a>
{
    red: &'a str,
    green: &'a str,
    blue: &'a str
}

fn arr(){
    let numbers = [1,2,3,4,5];

    
    println!("{}", numbers[0]);

    println!("{}", numbers.len());

    for i in numbers.iter()
    {
        println!("{}", i);
    }


}

fn refs(mut x: u32) 
{
    let xr = &x;

    println!("ref is {}", xr);

    let mutxr = &mut x;   

    *mutxr+=1;
    println!("ref is {}", mutxr);
}


fn tupl()
{
    let tup1 = (1,2,3,(4,5,6));

    //destructuring
    let (a,b,c,d) = tup1;
    println!("{}", tup1.2);
    println!("{}", (tup1.3).1);
}

fn forloop() -> i32
{
    let mut m = 0;
    for a in 1..10
    {
        m+=1;
        println!("the number is {}", m);
    }

    let mut animals = vec!["bugs", "daffy", "tom", "jerry"];

    animals.push("scooby");

    animals.push("dd");

    animals.remove(5);

    for i in animals.iter()
    {
        println!("the loony animal is {}", i);
    }

    for (id, name) in animals.iter().enumerate()
    {
        println!("the loony animal id, name is {}, {}", id, name);
    }

    return m;
}

fn whileloop(mut a: i32) -> i32
{
    let mut m = 0;
    while a>=0
    {
        m+=10;
        a-=1;
    }
    return m;
}

fn looping(mut a: u32) -> i32
{
    let mut m = 0;

    loop {
        m+=10;

        if a==0
        {
            break;
        }
        a-=1;
    }
    return m;
}

fn ifelse(m: i64,n: i64)
{
    if m<n
    {
        println!("less");
    }
    else if m==n
    {
        println!("equal");
    }
    else
    {
        println!("greater");
    }
}

fn test()
{
    let x: i64 = -69; //i64

    let y: u64 = 40; //u64

    let z: f32 = 3.4; //f32

    let w: bool = false; //bool

    println!("{} {}", x, y);
}

fn main() {
    println!("Hello, world!");

    //user input
    let mut input = String::new();

    println!("say something");

    match io::stdin().read_line(&mut input)
    {
        Ok(_) => {
            println!("success {}", input);
        },
        Err(e) => {
            println!("error {}", e);
        }
    }

    let mut x=45; //i32
    let y="man";

    x+=1;

    println!("{} {}", x,y);

    test();

    ifelse(10, 10);
    
    println!("{}", looping(5));

    println!("{}", whileloop(5));
    
    println!("{}", forloop());

    println!("{}", NAME);

    tupl();

    refs(20);

    let bg = Color{red:"red", green:"green", blue:"blue"};

    let mystrforbg = String::from("red and green and blue");


    println!("{}", bg.red);

    rect();

    //string
    let mystring = String::from("hows it going?");

    println!("length: {}", mystring.len());
    println!("is empty: {}", mystring.is_empty());

    for t in mystring.split_whitespace()
    {
        print!("{}", t);
    }

    //switch match

    let number =13;

    println!("");

    match number
    {
        1 => println!("1"),

        2 => println!("2"),

        10 | 11 => println!("10 oder 11"),

        _ => println!("no match")
    }


    let mut file = File::open("../../info.txt").expect("cant open the file");

    let mut contents = String::new();


    

    // //write to file
    // file.write_all(b"fc barca").expect("could not write to fail. failed as barca's european campaign");

    // //read file
    // file.read_to_string(&mut contents).expect("cant read..");

    // println!("\n{}", contents);


    // let args:: Vec<String> = env::args().collect();

    // for arg in args.iter()
    // {
    //     println!("{}", arg);
    // }


   


}
