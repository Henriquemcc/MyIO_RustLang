[Versão em Português](README.md)

# MyIO Library

This repository contains the MyIO library for the Rust programming language.

## Functions

### fn read_string() -> String

This function is used to read a string from standard input.

#### Example

```
mod myio;

fn main()
{
    println!("Reading string: ");
    let st = myio::read_string();
    println!("{}" ,st);
}
```

### fn read_u8() -> u8

This function is used to read an 8-bit unsigned integer from standard input.

#### Example

```
mod myio;

fn main()
{
    println!("Reading u8: ");
    let u = myio::read_u8();
    println!("{}", u);
}
```

### fn read_u16() -> u16

This function is used to read a 16-bit unsigned integer from standard input.

#### Example

```
mod myio;

fn main()
{
    println!("Reading u16: ");
    let u = myio::read_u16();
    println!("{}" ,u);
}
```

### fn read_u32() -> u32

This function is used to read a 32-bit unsigned integer from standard input.

#### Example

```
mod myio;

fn main()
{
    println!("Reading u32: ");
    let u = myio::read_u32();
    println!("{}" ,u);
}
```

### fn read_u64() -> u64

This function is used to read a 64-bit unsigned integer from standard input.

#### Example

```
mod myio;

fn main()
{
    println!("Reading u64: ");
    let u = myio::read_u64();
    println!("{}" ,u);
}
```

### fn read_u128() -> u128

This function is used to read a 128-bit unsigned integer from standard input.

#### Example

```
mod myio;

fn main()
{
    println!("Reading u128: ");
    let u = myio::read_u128();
    println!("{}" ,u);
}
```

### fn read_usize() -> usize

This function is used to read a unsigned integer from standard input with the size of the target processor.

#### Example

```
mod myio;

fn main()
{
    println!("Reading usize: ");
    let u = myio::read_usize();
    println!("{}" ,u);
}
```

### fn read_i8() -> i8

This function is used to read a 8-bit signed integer from standard input.

#### Example

```
mod myio;

fn main()
{
    println!("Reading i8: ");
    let i = myio::read_i8();
    println!("{}" ,i);
}
```

### fn read_i16() -> i16

This function is used to read a 16-bit signed integer from standard input.

#### Example

```
mod myio;

fn main()
{
    println!("Reading i16: ");
    let i = myio::read_i16();
    println!("{}" ,i);
}
```

### fn read_i32() -> i32

This function is used to read a 32-bit signed integer from standard input.

#### Example

```
mod myio;

fn main()
{
    println!("Reading i32: ");
    let i = myio::read_i32();
    println!("{}" ,i);
}
```

### fn read_i64() -> i64

This function is used to read a 64-bit signed integer from standard input.

#### Example

```
mod myio;

fn main()
{
    println!("Reading i64: ");
    let i = myio::read_i64();
    println!("{}" ,i);
}
```

### fn read_i128() -> i128

This function is used to read a 128-bit signed integer from standard input.

#### Example

```
mod myio;

fn main()
{
    println!("Reading i128: ");
    let i = myio::read_i128();
    println!("{}" ,i);
}
```

### fn read_isize() -> isize

This function is used to read a signed integer from standard input with the size of the target processor.

#### Example

```
mod myio;

fn main()
{
    println!("Reading isize: ");
    let i = myio::read_isize();
    println!("{}" ,i);
}
```

### fn read_f32() -> f32

This function is used to read a 32-bit floating-point number from standard input.

#### Example

```
mod myio;

fn main()
{
    println!("Reading f32: ");
    let f = myio::read_f32();
    println!("{}" ,f);
}
```

### fn read_f64() -> f64

This function is used to read a 64-bit floating-point number from standard input.

#### Example

```
mod myio;

fn main()
{
    println!("Reading f64: ");
    let f = myio::read_f64();
    println!("{}" ,f);
}
```

### fn read_char() -> char

This function is used to read a character from standard input.
#### Example

```
mod myio;

fn main()
{
    println!("Reading char: ");
    let c = myio::read_char();
    println!("{}" ,c);
}
```

### fn read_bool() -> bool

This function is used to read a boolean from standard input.

#### Example

```
mod myio;

fn main()
{
    println!("Reading bool: ");
    let b = myio::read_bool();
    println!("{}" ,b);
}
```

## License

This library is licensed under [MIT License](LICENSE).