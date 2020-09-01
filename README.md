[English Version](README.EN.md)

# Biblioteca MyIO

Este repositório contém a biblioteca MyIO para a linguagem Rust.

## Funções

### fn read_string() -> String

Esta função serve para ler uma string da entrada padrão.

#### Exemplo

```
mod myio;

fn main()
{
    println!("Lendo string: ");
    let st = myio::read_string();
    println!("{}" ,st);
}
```

### fn read_u8() -> u8

Esta função serve para ler um número inteiro unsigned de 8 bits da entrada padrão.

#### Exemplo

```
mod myio;

fn main()
{
    println!("Lendo u8: ");
    let u = myio::read_u8();
    println!("{}", u);
}
```

### fn read_u16() -> u16

Esta função serve para ler um número inteiro unsigned de 16 bits da entrada padrão.

#### Exemplo

```
mod myio;

fn main()
{
    println!("Lendo u16: ");
    let u = myio::read_u16();
    println!("{}" ,u);
}
```

### fn read_u32() -> u32

Esta função serve para ler um número inteiro unsigned de 32 bits da entrada padrão.

#### Exemplo

```
mod myio;

fn main()
{
    println!("Lendo u32: ");
    let u = myio::read_u32();
    println!("{}" ,u);
}
```

### fn read_u64() -> u64

Esta função serve para ler um número inteiro unsigned de 64 bits da entrada padrão.

#### Exemplo

```
mod myio;

fn main()
{
    println!("Lendo u64: ");
    let u = myio::read_u64();
    println!("{}" ,u);
}
```

### fn read_u128() -> u128

Esta função serve para ler um número inteiro unsigned de 128 bits da entrada padrão.

#### Exemplo

```
mod myio;

fn main()
{
    println!("Lendo u128: ");
    let u = myio::read_u128();
    println!("{}" ,u);
}
```

### fn read_usize() -> usize

Esta função serve para ler um número inteiro unsigned, do tamanho da palavra do processador, da entrada padrão.

#### Exemplo

```
mod myio;

fn main()
{
    println!("Lendo usize: ");
    let u = myio::read_usize();
    println!("{}" ,u);
}
```

### fn read_i8() -> i8

Esta função serve para ler um número inteiro de 8 bits da entrada padrão.

#### Exemplo

```
mod myio;

fn main()
{
    println!("Lendo i8: ");
    let i = myio::read_i8();
    println!("{}" ,i);
}
```

### fn read_i16() -> i16

Esta função serve para ler um número inteiro de 16 bits da entrada padrão.

#### Exemplo

```
mod myio;

fn main()
{
    println!("Lendo i16: ");
    let i = myio::read_i16();
    println!("{}" ,i);
}
```

### fn read_i32() -> i32

Esta função serve para ler um número inteiro de 32 bits da entrada padrão.

#### Exemplo

```
mod myio;

fn main()
{
    println!("Lendo i32: ");
    let i = myio::read_i32();
    println!("{}" ,i);
}
```

### fn read_i64() -> i64

Esta função serve para ler um número inteiro de 64 bits da entrada padrão.

#### Exemplo

```
mod myio;

fn main()
{
    println!("Lendo i64: ");
    let i = myio::read_i64();
    println!("{}" ,i);
}
```

### fn read_i128() -> i128

Esta função serve para ler um número inteiro de 128 bits da entrada padrão.

#### Exemplo

```
mod myio;

fn main()
{
    println!("Lendo i128: ");
    let i = myio::read_i128();
    println!("{}" ,i);
}
```

### fn read_isize() -> isize

Esta função serve para ler um número inteiro, do tamanho da palavra do processador ,da entrada padrão.

#### Exemplo

```
mod myio;

fn main()
{
    println!("Lendo isize: ");
    let i = myio::read_isize();
    println!("{}" ,i);
}
```

### fn read_f32() -> f32

Esta função serve para ler um número real de 32 bits da entrada padrão.

#### Exemplo

```
mod myio;

fn main()
{
    println!("Lendo f32: ");
    let f = myio::read_f32();
    println!("{}" ,f);
}
```

### fn read_f64() -> f64

Esta função serve para ler um número inteiro de 64 bits da entrada padrão.

#### Exemplo

```
mod myio;

fn main()
{
    println!("Lendo f64: ");
    let f = myio::read_f64();
    println!("{}" ,f);
}
```

### fn read_char() -> char

Esta função serve para ler um caractere da entrada padrão.

#### Exemplo

```
mod myio;

fn main()
{
    println!("Lendo char: ");
    let c = myio::read_char();
    println!("{}" ,c);
}
```

### fn read_bool() -> bool

Esta função serve para ler um valor booleano da entrada padrão.

#### Exemplo

```
mod myio;

fn main()
{
    println!("Lendo bool: ");
    let b = myio::read_bool();
    println!("{}" ,b);
}
```

## Licença

Esta biblioteca está licenciada sobre [Licença MIT](LICENSE).