fn main() {
    let mut count=1;
    loop{
        println!("counter:{}",count);
        if count==10{
            break;
        }
        count+=1;
    }

    // nested loops 

    let mut count1 = 1;
    
    'outer:loop{     
        let mut icount=1;  
        loop{   
            println!("count1:{}-->icount:{}",count1,icount);
            
            if icount==10 && count1==10 {
                break 'outer;
            }else if icount==10{
                break;
            }
            icount+=1;
        }
        count1+=1;
    }
}


// match ,loop, for ,
// loop {}
// while
// for in 
