fn main(){
    let mut x =10;

    println!("x is a number: {}",x);
    {
        let x=12;

    println!("the value of x now is : {}",x);
    }
    let x =" is now a string";
    println!("x is now a string : {}",x);
    let x=true;
    
    println!("x is now a boolean : {}",x);
}