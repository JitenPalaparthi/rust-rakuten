fn main() {
    let b1 = biggest1(10, 12);
    println!("Biggest among of i and j:{}", b1);

    let i1: Option<i32> = Some(20);
    let j1: Option<i32> = Some(15);

    let (b2, err) = biggest2(i1, j1);

    if err != "".to_string() {
        println!("Error:{}", err);
    } else {
        println!("Biggest among of i and j:{}", b2);
    }

    let i2: Option<i32> = None;
    let j2: Option<i32> = Some(15);

    let result = biggest3(i2, j2);

    match result {
        Err(e) => println!("---->>>>>{}", e),
        Ok(v) => println!("The biggest value of i2 and j2 is {}", v),
    }


    let i3: Option<i32> = Some(25);
    let j3: Option<i32> = Some(15);

    let result = biggest3(i3, j3);

    match result {
        Err(e) => println!("{}", e),
        Ok(v) => println!("The biggest value of i3 and j3 is {}", v),
    }

    let i4: Option<i32> = Some(25);
    let j4: Option<i32> = Some(32);

    let result = biggest3(i4, j4);

    let b5 =result.expect("some error most probably None value passed"); // this panics the entire application

    println!("The biggest value of i4 and j4 is {}", b5);

    let i5: Option<i32> = Some(25);
    let j5: Option<i32> = Some(45);

    let result =  biggest5(i5,j5);

    match result{
     Err(e)=>println!("{}",e),
     Ok(())=> (),
    }


    let b6 = biggest4(i5, j5);

    println!("The biggest value of i5 and j5 is {}", b6);


}

fn biggest1(i: i32, j: i32) -> i32 {
    if i > j {
        return i;
    }
    j
}

// if i or j has Some(T) then return the biggest
// if i or j is None then return an error as invalid input value
fn biggest2(i: Option<i32>, j: Option<i32>) -> (i32, String) {
    let mut i1 = 0;
    let mut j1: i32 = 0;
    match i {
        None => {
            return (-1, "invalid first parameter.None value".to_string());
        }
        Some(v) => {
            i1 = v;
        }
    }

    match j {
        None => {
            return (-1, "invalid second parameter. None value".to_string());
        }
        Some(v) => {
            j1 = v;
        }
    }

    if i1 > j1 {
        return (i1, "".to_string());
    }

    (j1, "".to_string())
}

// return value is an enum
// send either Ok or Err
fn biggest3(i: Option<i32>, j: Option<i32>) -> Result<i32, String> {
    let mut i1 = 0;
    let mut j1: i32 = 0;
    match i {
        None => {
            return Err("invalid first parameter.None value".to_string());
        }
        Some(v) => {
            i1 = v;
        }
    }

    match j {
        None => {
            //  eprintln!("invalid second parameter.None value")
            return Err("invalid second parameter.None value".to_string());
        }
        Some(v) => {
            j1 = v;
        }
    }

    if i1 > j1 {
        return Ok(i1);
    }
    Ok(j1)
}


fn biggest4(i: Option<i32>, j: Option<i32>) -> i32 {
    let mut i1 = 0;
    let mut j1: i32 = 0;
    match i {
        None => {
            panic!("-->><><><>invalid first parameter.None value");
        }
        Some(v) => {
            i1 = v;
        }
    }

    match j {
        None => {
            //  eprintln!("invalid second parameter.None value")
            panic!("-->><><><>invalid second parameter.None value");
        }
        Some(v) => {
            j1 = v;
        }
    }

    if i1 > j1 {
       return i1;
    }
 j1
}








fn biggest5(i: Option<i32>, j: Option<i32>) -> Result<(), String> {
    let mut i1 = 0;
    let mut j1: i32 = 0;
    match i {
        None => {
            return Err("invalid first parameter.None value".to_string());
        }
        Some(v) => {
            i1 = v;
        }
    }

    match j {
        None => {
            //  eprintln!("invalid second parameter.None value")
            return Err("invalid second parameter.None value".to_string());
        }
        Some(v) => {
            j1 = v;
        }
    }

    if i1 > j1 {
        println!("biggest is {}",i1);
    }else{
        println!("biggest is {}",j1);
    }

    Ok(()) // nothing is returne upon successful execution
}

// errors are just values in rust

// errors are just values in rust
// if an error can be handled then handle it
// if not panic it
