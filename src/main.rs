mod myio;

fn main()
{
    println!("Testando:");

    println!("Lendo string: ");
    let st = myio::read_string();
    println!("{}" ,st);

    println!("Lendo u8: ");
    let u = myio::read_u8();
    println!("{}", u);

    println!("Lendo u16: ");
    let u = myio::read_u16();
    println!("{}" ,u);

    println!("Lendo u32: ");
    let u = myio::read_u32();
    println!("{}" ,u);

    println!("Lendo u64: ");
    let u = myio::read_u64();
    println!("{}" ,u);

    println!("Lendo u128: ");
    let u = myio::read_u128();
    println!("{}" ,u);

    println!("Lendo usize: ");
    let u = myio::read_usize();
    println!("{}" ,u);

    println!("Lendo i8: ");
    let i = myio::read_i8();
    println!("{}" ,i);

    println!("Lendo i16: ");
    let i = myio::read_i16();
    println!("{}" ,i);

    println!("Lendo i32: ");
    let i = myio::read_i32();
    println!("{}" ,i);

    println!("Lendo i64: ");
    let i = myio::read_i64();
    println!("{}" ,i);

    println!("Lendo i128: ");
    let i = myio::read_i128();
    println!("{}" ,i);

    println!("Lendo isize: ");
    let i = myio::read_isize();
    println!("{}" ,i);

    println!("Lendo f32: ");
    let f = myio::read_f32();
    println!("{}" ,f);

    println!("Lendo f64: ");
    let f = myio::read_f64();
    println!("{}" ,f);

    println!("Lendo char: ");
    let c = myio::read_char();
    println!("{}" ,c);

    println!("Lendo bool: ");
    let b = myio::read_bool();
    println!("{}" ,b);


}
