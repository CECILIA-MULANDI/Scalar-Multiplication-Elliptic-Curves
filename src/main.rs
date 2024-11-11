#[derive(Debug)]
struct Point {
    x: i128,
    y: i128,
}

#[derive(Debug)]
struct EllipticCurve {
    a: i128,
    b: i128,
    p: i128,
}

fn double_add_algorithm(point_p: Point, mut scalar: u128, curve: &EllipticCurve) -> Point {
    if scalar == 0 {
        return Point { x: 0, y: 0 };
    }
    
    let mut point_q = point_p;
    let mut point_r = Point { x: 0, y: 0 };
    
    while scalar > 0 {
        if scalar % 2 == 1 {
            point_r = point_addition(&point_r, &point_q, curve);
        }
        point_q = point_doubling(&point_q, curve);
        scalar = scalar/2;
    }
    point_r
}

fn point_addition(point1: &Point, point2: &Point, curve: &EllipticCurve) -> Point {
    if point1.x == 0 && point1.y == 0 {
        return Point { x: point2.x, y: point2.y };
    }
    if point2.x == 0 && point2.y == 0 {
        return Point { x: point1.x, y: point1.y };
    }
    if point1.x == point2.x && point1.y == point2.y {
        return point_doubling(point1, curve);
    }

    let dy = ((point2.y - point1.y) % curve.p + curve.p) % curve.p;
    let dx = ((point2.x - point1.x) % curve.p + curve.p) % curve.p;
    
    let dx_inverse = match mod_inverse(dx, curve.p) {
        Some(val) => val,
        None => return Point { x: 0, y: 0 },
    };
    
    let slope = (dy * dx_inverse) % curve.p;
    let x3 = ((((slope * slope) % curve.p - point1.x - point2.x) % curve.p + curve.p) % curve.p);
    let y3 = ((slope * (point1.x - x3) - point1.y) % curve.p + curve.p) % curve.p;

    Point { x: x3, y: y3 }
}

fn point_doubling(point: &Point, curve: &EllipticCurve) -> Point {
    if point.x == 0 && point.y == 0 || point.y == 0 {
        return Point { x: 0, y: 0 };
    }

    let dy = ((3 * point.x * point.x + curve.a) % curve.p + curve.p) % curve.p;
    let dx = (2 * point.y % curve.p + curve.p) % curve.p;
    
    let dx_inverse = match mod_inverse(dx, curve.p) {
        Some(val) => val,
        None => return Point { x: 0, y: 0 },
    };
    
    let slope = (dy * dx_inverse) % curve.p;
    let x3 = ((slope * slope - 2 * point.x) % curve.p + curve.p) % curve.p;
    let y3 = ((slope * (point.x - x3) - point.y) % curve.p + curve.p) % curve.p;

    Point { x: x3, y: y3 }
}

fn extended_euclidean_algorithm(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x1, y1) = extended_euclidean_algorithm(b, a % b);
        let x = y1;
        let y = x1 - (a / b) * y1;
        (gcd, x, y)
    }
}

fn mod_inverse(a: i128, b: i128) -> Option<i128> {
    let (gcd, x, _) = extended_euclidean_algorithm(a, b);
    if gcd == 1 {
        Some((x % b + b) % b)
    } else {
        None
    }
}

fn main() {
    let curve = EllipticCurve { a: 497, b: 1768, p: 9739 };
    let result = double_add_algorithm(Point { x: 5323, y: 5438 }, 1337, &curve);
    println!("Result: {:?}", result);
}