mod foo;

pub fn this_is_gory(ctx: &super::TransmogCtx) {
    println!("Public interface to gory details");
    if ctx.verbose {
        even_gorier();
    }

    //  crate::transmogrify(ctx, 2);
}

fn even_gorier() {
    println!("..........This is now super-gory, walk away, nothing to see here!");
    foo::foo();
}
