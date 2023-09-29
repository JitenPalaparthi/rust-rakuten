fn main() {
    let v1 = vec![10, 20, 30];
    let v2 = vec![40, 50];
    let v3 = vec![0];
    let v4 = vec![100, 200];

    let v5 = vec![10, 20, 30];
    let v6 = vec![40, 50];
    let v7 = vec![0];
    let v8 = vec![100, 200];

    let mut c1: C = C { sum: 0 };
    let sum = c1
        .add(v1)
        .add(v2)
        .add(v3)
        .add(v4)
        .sub(v5)
        .sub(v6)
        .sub(v7)
        .sub(v8)
        .get();

    println!("Sum of vectors:{}", sum);
}

// add, sub
//add(vec![10,20,13,14]).add(vec![10,12,13]).get()

trait Calculate {
    fn add(&mut self, v: Vec<i32>) -> Box<&mut dyn Calculate>;
    fn sub(&mut self, v: Vec<i32>) -> Box<&mut dyn Calculate>;
    fn get(&mut self) -> i32;
}

struct C {
    sum: i32,
}

impl Calculate for C {
    fn add(&mut self, v: Vec<i32>) -> Box<&mut dyn Calculate> {
        for n in v {
            self.sum = self.sum + n;
        }
        return Box::new(self);
    }

    fn sub(&mut self, v: Vec<i32>) -> Box<&mut dyn Calculate> {
        for n in v {
            self.sum = self.sum - n;
        }
        return Box::new(self);
    }

    fn get(&mut self) -> i32 {
        return self.sum;
    }
}
