//x^2-4
fn f(x: f64) -> f64 {
    x * x - 4.0
}

fn df(x: f64) -> f64 {
    2.0 * x
}

//iteracyjna z pętlą for (z ewentualnymi break continue return)
fn newton_for_loop(x0: f64, eps: f64, n: u128) -> f64 {
    let mut x: f64 = x0;

    for _i in 0..n {
        let fx: f64 = f(x);
        let dfx: f64 = df(x);
        let dx: f64 = fx / dfx;

        x -= dx;

        if dx.abs() < eps {
            return x;
        }
    }
    x
}

//rekurencyjna
fn newton_recursive(x0: f64, epsilon: f64, iterations: u128, counter: u128, x: f64) -> f64 {
    if counter == iterations {
        return x;
    }
    let fx: f64 = f(x);
    let dfx: f64 = df(x);

    let dx: f64 = fx / dfx;
    if dx.abs() < epsilon {
        return newton_recursive(x0, epsilon, iterations, iterations, x-dx)
    }
    return newton_recursive(x0, epsilon, iterations, counter + 1, x-dx);
}

//iteracyjna z pętlą while (bez żadnych break continue return);
fn newton_while(x0: f64, epsilon: f64, iterations: u128) -> f64 {
    let mut x: f64 = x0;

    let mut counter: u128 = 0;
    while counter < iterations {
        counter+=1;

        let fx: f64 = f(x);
        let dfx: f64 = df(x);

        let dx: f64 = fx / dfx;

        x-=dx;

        if dx.abs() < epsilon {
            counter = iterations;
        }
    }
    x
}


//iteracyjna z pętlą loop (z ewentualnymi break continue return);
fn newton_loop(x0: f64, epsilon: f64, iterations: u128) -> f64 {
    let mut x: f64 = x0;

    let mut counter: u128 = 0;
    loop {
        if counter == iterations {
            break;
        }
        counter += 1;

        let fx: f64 = f(x);
        let dfx: f64 = df(x);

        let dx: f64 = fx / dfx;

        x -= dx;

        if dx.abs() < epsilon {
            return x;
        }
    }
    x
}

fn main() {
    //start:
    let x0: f64 = 3.0;

    //dozwolona tolerancja bledu:
    let eps: f64 = 0.0001;

    //liczba iteracji:
    let n: u128 = 100;

    let counter: u128 = 0;
    let x: f64 = x0;
    let result = newton_recursive(x0, eps, n, counter, x);
    println!("Przyblizone miejsce zerowe: {}", result);
}
