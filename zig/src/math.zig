export fn add(a: i32, b: i32) i32 {
    return a + b;
}

export fn sub(a: i32, b: i32) i32 {
    return a - b;
}

export fn mul(a: i32, b: i32) i32 {
    return a * b;
}

export fn div(a: i32, b: i32) i32 {
    return @divTrunc(a, b);
}
