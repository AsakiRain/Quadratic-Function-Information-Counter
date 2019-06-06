use std::io;

fn main() {

    println!("Quadratic Function Information Counter");
    println!("Produced by BiDuang | Version: 1.45 InPreview"); //Program Entrance

    println!("Please input quadratic function's data: a b c");
    println!("Quadratic function looks like y=ax²+bx+c and a cannot be 0"); //Input Notice

    let mut a = String::new();  
    let mut b = String::new();  
    let mut c = String::new();
    let mut mode = String::new();
    let mut ux = String::new();
    let mut uy = String::new();  //Creative Vars

    const upd:i32 = -1;

    println!("a:");
    io::stdin().read_line(&mut a)
        .expect("Failed to read a correct line!-a");
    println!("b:");
    io::stdin().read_line(&mut b)
        .expect("Failed to read a correct line!-b");
    println!("c:");
    io::stdin().read_line(&mut c)
        .expect("Failed to read a correct line!-c"); //Read vars
    
    let a:u32 = match a.trim().parse()  {
        Ok(num) => num,
        Err(_) => 0,
    };  
    let b:u32 = match b.trim().parse()  {
        Ok(num) => num,
        Err(_) => 0,
    };
    let c:u32 = match c.trim().parse()  {
        Ok(num) => num,
        Err(_) => 0,
    }; //Strings to Numbers
    
    println!("Your quadratic function data has been read , counting..."); //Notice

    if a == 0 {
        println!("Quadratic function's a cannot be 0. Incorrect data!");
        println!("Because your data is incorrect, program will be shutdown."); //Data check
    }else{

        println!("Your quadratic function is y={}x²+{}x+{}",a,b,c);
        let a = a as i32;
        let b = b as i32;
        let c = c as i32; //u32 To i32
        let mut tx:i32 = b/(2*a)*upd;
        let mut ty:i32 = a*tx*tx+b*tx+c;
        let mut xps:i32 = b*b-4*a*c;
        println!("Top site is [{},{}]",tx,ty); 
        if xps == 0 {
            println!("Quadratic function has only 1 point in x-axis");
        }else{
            if xps>0 {
                println!("Quadratic function has 2 points in x-axis");
            }else{
                println!("The quadratic function has no intersection with the x-axis");
            }
        }; //Quadratic function information

        println!("Please select mode (x mode input 1) [Y mode is unavailable.]");
        io::stdin().read_line(&mut mode)
            .expect("Failed to read a correct line!-mode");
        let mode:u32 = match mode.trim().parse()  {
                Ok(num) => num,
                Err(_) => 0,
            }; //Mode Translate

        if mode == 1 {
            println!("You selected x mode. Please input x, point in y={}x²+{}x+{} will come out",a,b,c);

            io::stdin().read_line(&mut ux)
                .expect("Failed to read a correct line!-ux");
            let ux:i32 = match ux.trim().parse()  {
                Ok(num) => num,
                Err(_) => 0,
            }; //Read input x

            let mut oy:i32 = a*ux*ux+ux*b+c; //Counting
            
            println!("Point at [{},{}]",ux,oy); //Result
        }else{
            println!("Unavailable mode!");
        };
    }; 

    println!("Thanks for your use.Program will quit automatic."); //Program quit

}

