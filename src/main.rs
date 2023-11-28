// ДЗ ДЕЛИТЕЛИ 2
// 124478618
/*
fn is_simple(x: i128) -> bool {
    let mut k = 0;
    for el in 2..x {
        if x % el == 0 {
            k += 1;
        } 
    }
    if k == 2 {
        return true
    }
    return false
}

fn main() {
    let mut c = 0;
    for i in 4986..32600 {
        if is_simple(i) {
            c += i;
        }
    }
    println!("{}", c)
}
*/



// 17 4913 and 19 6859
/*
fn is_simple(x: i128) -> (bool, Vec<i128>) {
    let mut k = 0;
    let mut c: Vec<i128> = Vec::new();
    for el in 2..x {
        if x % el == 0 {
            k += 1;
            c.push(el)
        } 
    }
    if k == 3 {

        return (true, c)
    }
    return (false, c)
    }
  

fn main() {
    for i in 81234..134690 {
        if is_simple(i).0 {
            println!("{:#?} {:#?}", is_simple(i).1.iter().max().unwrap(), is_simple(i).1.iter().min().unwrap());
        }
    }
}
*/

// ДЗ ПРОСТЫЕ ЧИСЛА 1

/*
7178609 1
7178617 2
7178621 3
7178623 4
7178627 5
7178653 6
7178657 7
7178659 8

fn is_simple(x: i32) -> bool {
    for el in 2..x {
        if x % el == 0 {
            return false;
        } 
    }
    return true;
}

fn main() {
    let mut k: u8 = 0;
    for i in 7178551..7178660 {
        if is_simple(i){
            k += 1;
            println!("{} {}", i, k);
        }
    }
}
*/

use std::cmp::max;
use std::cmp::min;

fn is_simple(x: i128) -> bool {
    for el in 2..((x as f64).sqrt() as i128) {
        println!("ХУЯРИМ ЪУЯРИМ");
        if x % el == 0 {
            println!("ПИЗДА ХУЦЯН ЧИСЛО");
            return false;
        } 
    }
    println!("ЗАЕБИСЬ ЧИСЛО");
    return true;
}

fn main() {

    let mut k: i32 = 10000;
    let mut x: Vec<i32> = Vec::new();

    for i in 523456..578926 {

        for i_2 in 523456..578926 {
            println!("ЗАЕБИСЬ ЦИКЛЫ ПРОШЛИ ЩА ПРОВЕРИМ ДАЛЬШЕ");
            if is_simple(i*i_2) {
                if k > max(i as i32, i_2 as i32) - min(i as i32, i_2 as i32) {
                    x.clear();
                    k = max(i as i32, i_2 as i32) - min(i as i32, i_2 as i32);
                    x.push(i as i32);
                    x.push(i_2 as i32);
                    println!("Я СРАБОТАЛА, ЖДИ ЕЩЕ ПАРУ МИНУТ {i} and {i_2}")
                }
            }
        }
    }
    println!("{:#?}", x)
}

/*
Рассматриваются целые числа, принадлежащих числовому отрезку [523456; 578925], 
которые представляют собой произведение двух различных простых делителей. 
Найдите такое из этих чисел, у которого два простых делителя меньше всего отличаются друг от друга. 
В ответе запишите простые делители этого числа в порядке возрастания. 
Если подходящих чисел несколько, запишите в ответе делители наименьшего из них.
*/