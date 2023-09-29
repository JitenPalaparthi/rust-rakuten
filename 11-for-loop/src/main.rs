fn main() {
    for i in 1.. {
        if i==20{
            break;
        }

        if i%2!=0{
            continue
        }
        print!("{} ",i);
    }


    'outer:for i in 1..{

        for j in 2..{

            if i*4 == j*2{
                print!("i:{} j:{}",i,j);
                break 'outer;
            }


        }
    }

}
