use libtransmogrify::context::Color;
use libtransmogrify::context::TransmogCtx;
use libtransmogrify::transmogrify;

pub fn main() {
    let mut ctx = TransmogCtx::create_ctx();
    ctx.colorize_ctx(Color::Yellow);
    ctx.verbose = true;
    TransmogCtx::otherfun();
    // ctx.color = Some(String::from("foo"));
    println!("Transmogrifying 3 -> {}", transmogrify(&ctx, 3));
}
