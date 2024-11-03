/*
 * Questions:
 * - Wo 'lebt' die variable Vec<&str> wenn ich sie nur referenziere?
 * - gibt es den typ str (String slice) oder ist das immer eine referenz?
 */

fn split_string(input: &str) -> Vec<&str> {
    let mut snippets: Vec<&str> = Vec::new();

    let mut start: usize = 0;
    let mut end: usize = 0;

    while start < input.len() && end < input.len() {
        // Interesting, man kann eine string-slice nicht direkt als array indizieren...
        // input[start] funktioniert nicht
        if input.as_bytes()[start] == b' ' {
            start += 1;
            end = start;
            continue;
        }

        if input.as_bytes()[end] != b' ' {
            end += 1;
            continue;
        }

        snippets.push(&input[start..end]);
        end += 1;
        start = end;
    }
    snippets
}
//------------------------------------------------------------------------------

fn print_snippets(teststring: &str, snippets: &Vec<&str>) {
    println!("{teststring} has {} snippets", snippets.len());
    for s in snippets {
        println!("|{s}|");
    }
}
//------------------------------------------------------------------------------

fn test_splitstring() {
    let empty_str = "";
    let only_spaces = "    ";
    let starts_start_with_spaces = "       hello world!";
    let ends_with_spaces = "hello world!   ";

    /*
    Wo ist die variable Vec<&str> von der ich mir die referenz hole?
    */
    print_snippets(empty_str, &split_string(empty_str));
    print_snippets(only_spaces, &split_string(only_spaces));
    print_snippets(
        starts_start_with_spaces,
        &split_string(starts_start_with_spaces),
    );
    print_snippets(ends_with_spaces, &split_string(ends_with_spaces));
}
//------------------------------------------------------------------------------

fn pass_through<T>(name: &str, var: T) -> T {
    println!("pass through variable {name}");
    var
}

//------------------------------------------------------------------------------
fn sink_var<T>(name: &str, _var: T) {
    println!("variable {name} can't be used anymore");
}

//------------------------------------------------------------------------------
fn main() {
    let primitive_var = 42;
    let heap_var: String = String::from("Variable on Heap");

    // Fun fact: rust does not like variables named 'foo'
    let foo = pass_through("heap_var", heap_var);
    let bar = pass_through("primitive_var", primitive_var);

    // Some references
    let rfoo = &foo;
    let r2foo = &foo;
    println!("Print refs: {r2foo}, {rfoo}");

    // 1 Takes ownership
    sink_var("foo", foo);
    //    println!("heap_var is gone! {heap_var}");
    //println!("Print refs: {r2foo}, {rfoo}");

    sink_var("bar", bar);
    println!("of course primitive_var can still be used");

    // gibt es den typ str (String slice) oder ist das immer eine referenz?
    // let aString = String::from("Some literal");
    // let aSlice: str = aString[2..];
    // println!("aSlice: {aSlice}");

    println!("---------- Snippets ---------");
    test_splitstring();

    println!("---------- Spooky call chain  ---------");
    // Spooky call chain - with refs to stack -> Rust saves me from this!
    // let v = take_vec_mut_ref(&mut ret_vec(32));

    let mut v_orig = ret_vec(3);
    // Side note &mut on rhs is an operation "give me a mutable ref!"
    let v_orig_ref = &mut v_orig;
    let v = take_vec_mut_ref(v_orig_ref);

    for e in v {
        println!("{e}");
    }
}

//------------------------------------------------------------------------------
fn ret_vec(sz: usize) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    v.resize(sz, 1);
    v
}

fn take_vec_mut_ref(v: &mut Vec<i32>) -> &Vec<i32> {
    v[0] = 2;
    v.resize(4, 3);
    v
}

// Interesting: not only is the vector immutable but also the elements!
// fn take_vec_ref(v: &Vec<i32>) -> &Vec<i32> {
//     v[0] = 2;
//     v.resize(4, 3);
//     v
// }
