fn main(){
    let mut n : i64 =0;
    loop {
        n +=1;
        println!("the value of n is : {}",n);
        if n >=30{
            break;
        }
    }
    n=0;
    println!("hellos")
    while n <=50{
        println!("the value of n is : {}",n);
        n+=1;

    }
    //integer
    let m=6..11;// this repeat 11 times
    for i in m{// we gona repeat in a list
        println!("{}",i);
    }
    // strings
    let fruits = vec!["banana ", "mango", "apple"];
    for i in fruits.into_iter(){
        println!("the fruit is : {}",i);
    }
    // integers and strings
    let fat=vec!["banana","idk","asd"];
    for (i , a) in fat.iter().enumerate(){
        println!(" the index is : {} and the fruit is : {}",i,a);
    }
}