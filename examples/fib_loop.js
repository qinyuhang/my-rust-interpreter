let fib = (n) => {
    let a = 0;
    let b = 1;
    let temp = 0;
    if (n == 0) return a;
    for (let c = 0; c < n ; c++) {
        temp = a + b;
        a = b;
        b = temp;
    }
    return b;
}

let r = fib(30);
console.log(r);