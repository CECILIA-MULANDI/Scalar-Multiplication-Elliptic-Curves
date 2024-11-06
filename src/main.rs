#[derive(Debug)]
struct Point{
    x:i128,
    y:i128
}
#[derive(Debug)]
struct EllipticCurve{
    a:i128,
    b:i128,
    p:i128
}

fn double_add_algorithm(point_p:Point,mut scalar:u128,curve:EllipticCurve)->Point{
    let mut point_q = point_p;
    let mut point_r = Point{ x:0, y:0 };
    while scalar > 0{
        if scalar % 2 == 1{
            point_r = Point{x:point_r.x + point_q.x, y:point_r.y + point_q.y};
          
        }
        point_q = point_doubling(point_q,&curve);
        scalar = scalar / 2;

    }
    return point_r;
}

fn point_doubling(point_q:Point,curve:&EllipticCurve)->Point{
    let dy = (3 * point_q.x.pow(2) + curve.a) % curve.p;
    let dx = (2 * point_q.y) % curve.p;
    let dx_inverse = mod_inverse(dx,curve.p);

    let slope = (dy * dx_inverse) % curve.p;
    let x3 = (slope * slope - point_q.x - point_q.x) % curve.p;
    let y3 = (slope * (point_q.x - x3) - point_q.y ) % curve.p;

    let new_point = Point { x: x3, y: y3};

    return new_point;
}
        
// helper functions
fn extended_euclidean_algorithm(a:i128,b:i128)->(i128,i128,i128){
    if b == 0{
        return (a,1,0);
    }else{
        let (gcd,x1,y1) = extended_euclidean_algorithm(b, a%b);
        let x = y1;
        let y = x1 - (a/b) * y1;
    
        (gcd,x,y)
    }
   
}

fn mod_inverse(a:i128,b:i128)->Option<i128>{
    let (gcd,x,_) = extended_euclidean_algorithm(a,b);
    if gcd == 1{
        return Some(x);
    }
    else{
        return None

    }
}



    


fn main() {
    let curve = EllipticCurve{ a: 2, b: 3 , p: 7};
    println!("{:?}",double_add_algorithm(Point{x:2,y:4},3,curve));
    // println!("{:?}",extended_euclidean_algorithm(5,17));

    // match mod_inverse(5,17){
    //     Some (res) => println!("{}",res ),
    //     None => println!("The number has no inverse")
    // };
}
