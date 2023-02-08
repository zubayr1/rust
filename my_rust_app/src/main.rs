const NAME: &str = "sarah ulfat";
const AGE: u8 = 20;

fn refs(x: u32) 
{
    let xr = &x;
    println!("ref is {}", xr);
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

    let animals = vec!["bugs", "daffy", "tom", "jerry"];

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
}
