fn main() {
    let  name = String::from("Georgie");
    let age = 32;
    let wallet = 7800.0;
    info(name, age, wallet);

    let a =  {
        let b = 0; 
        b + 1
    };
    println!("{}", a);

    let (a, b) = mul(12, 9);
    println!("{}, {}", a, b);

}

// fn hello(a: i32, b:i32) {
//     println!("Sum is {}", a + b);
// }

fn info(name: String, age: i32, wallet: f64)
{
    println!("{} is {} year old, wallet: ${}", name, age, wallet);
}

fn mul(a:i32, b:i32) -> (i32, i32)
{
   (a * b, a + b)
}



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

//    let mut a = String::new();
//    let mut b = String::new();
//     let mut c =String::new();

//     println!("Введите а: ");
//     match io::stdin().read_line(&mut a) {
//         Ok(_) => {},
//         Err(e) => {
//             println!("Error: {}", e);
//         }
//     }
//     println!("Введите b: ");
//     match io::stdin().read_line(&mut b) {
//         Ok(_) => {},
//         Err(e) => {
//             println!("Error: {}", e);
//         }
//     }
//     println!("Введите c: ");
//     match io::stdin().read_line(&mut c) {
//         Ok(_) => {},
//         Err(e) => {
//             println!("Error: {}", e);
//         }
//     }

//     let a: f64 = a.trim().parse().unwrap();
//     let b: f64 = b.trim().parse().unwrap();
//     let c: f64 = c.trim().parse().unwrap();
//     let d: f64 = (b*b) - 4.0 * (a*c);
//     if d > 0.0 {
//         let x1 = ((-b) + d.sqrt()) / (2.0 * a) ;
//         let x2 = ((-b) - d.sqrt()) / (2.0 * a) ;
        
//         println!("Дискриминант равен {}", d);
//         println!("Решено!\nЕсть 2 корня: {}, {}", x1 , x2);
      
//     } 
//     else if d == 0.0 {
//         let x = (-b) / (2.0*a);
//         println!("Дискриминант равен {}", d);
//     println!("Решено!\n Есть 1 корень: {}", x); } 
//     else if d <= 0.0 {
//         println!("Дискриминант равен {}", d);
//         println!("Уравнение не имеет корней!");
//     }

// let   array = [1,2,2,3,4,5,5];
// array[1] = 24;
// println!("{:?}", array);

// let arr = [2; 10];
// println!("{:?}", arr);
// for i in arr.iter() {
//     println!("{}", i);
// }

// let mut i = 0;
// while i < array.len() {
//     println!("{}", array[i]);
//     i += 1;
// }

    // let mut i = 0;
    // while i < array.len() {
    //     let mut j  = i + 1;
    //     while j < array.len() {
    //         if array[i] == array[j]
    //         {
    //             println!("{}", array[i]);
    //         }
    //         j += 1;
    //     }
    //     i += 1;
    // }




