use std::io;



fn main() {
    // let age: i8 = 17;
    // if age >= 18 {
    //     println!("You are welcome!");
    // } else {
    //     println!("NO!");
    // }

    // let name: String = String::from("Petrov");
    // if name == "Denis" {
    //     println!("Hi, Denis");
    // } else {
    //     println!("Hi, {}", name);
    // }

    // let is_true = false;

    // let num = if is_true {
    //     1
    // } else {
    //     0
    // };

    // println!("{}", num);
    

    // let num = 10;
    // match num {
    //     10 => println!("Num is 10"),
    //     23  => {
    //         for i in 0..100 {
    //             println!("{}", i);
    //         }
    //     },
    //     10..=50 => {
    //         println!("{}", num);
    //     },
    //     _ => {
    //         println!()
    //     }

    // }

    // let mat = match num {
    //     2 => 1,
    //     10 => 3,
    //     _ => 0
        
    // };
    // println!("{}", mat);

   let mut a = String::new();
   let mut b = String::new();
    let mut c =String::new();

    println!("Введите а: ");
    match io::stdin().read_line(&mut a) {
        Ok(_) => {},
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    println!("Введите b: ");
    match io::stdin().read_line(&mut b) {
        Ok(_) => {},
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    println!("Введите c: ");
    match io::stdin().read_line(&mut c) {
        Ok(_) => {},
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let a: f64 = a.trim().parse().unwrap();
    let b: f64 = b.trim().parse().unwrap();
    let c: f64 = c.trim().parse().unwrap();
    let d: f64 = (b*b) - 4.0 * (a*c);
    if d > 0.0 {
        let x1 = ((-b) + d.sqrt()) / (2.0 * a) ;
        let x2 = ((-b) - d.sqrt()) / (2.0 * a) ;
        
        println!("Дискриминант равен {}", d);
        println!("Решено!\nЕсть 2 корня: {}, {}", x1 , x2);
      
    } 
    else if d == 0.0 {
        let x = (-b) / (2.0*a);
        println!("Дискриминант равен {}", d);
    println!("Решено!\n Есть 1 корень: {}", x); } 
    else if d <= 0.0 {
        println!("Дискриминант равен {}", d);
        println!("Уравнение не имеет корней!");
    }

}