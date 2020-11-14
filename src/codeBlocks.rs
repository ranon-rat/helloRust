fn main(){
    let m=0;
    {
        // only we can use the variable y in this block , if you uses outside of this block you gona see a error
        let y=5;
        println!("m : {}, y : {}",m,y)
    }
}