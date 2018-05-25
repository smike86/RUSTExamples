fn main() {
    let s = String::from("A perfect circle");

    borrow_string(&s);
    borrow_string(&s);
    //use_string(s);
    //e1:
    //use_string(s);

    

    let r1 = &s;
    let r2 = &s;
    borrow_string(&s);
    borrow_string(r1);
    borrow_string(r2);

    //e2:
    /*
    {
        let rs = &s; // s is not more valid
        borrow_string(&s);
        borrow_string(&s);
    }
    //s is valid again
    */
}

fn borrow_string(s: &String){
    println!("borrow s is {}",s);
}

fn use_string(s: String){
    println!("  used s is {}",s);
    
}

