fn apply_fn<F>(f: F)
where
    F: for<'a> Fn(&'a i32),
{
    let x = 42;
    f(&x);
}

fn call_with_ref<'a, T, F>(value: &'a T, func: F)
where
    F: FnOnce(&'a T),
{
    func(value);
}

fn main() {
    let print_fn = |x: &i32| println!("x: {}", x);
    apply_fn(print_fn);

    let x = 42;
    let y = &x;

    // This closure borrows y, but we can still use it with the call_with_ref function.
    let print_fn = |z: &i32| println!("z is pointing to: {}", y);
    call_with_ref(&x, print_fn);
}
