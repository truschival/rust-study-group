pub const TRANSMOG_MAGIC: i32 = 7;

pub mod context {

    mod gorydetails;

    pub enum Color {
        Normal,
        Yellow,
        Red,
    }

    pub struct TransmogCtx {
        pub verbose: bool,
        color: Option<String>,
    }

    mod private {
        use crate::context::Color;
        use crate::context::TransmogCtx;

        fn secret_func() {
            println!("secret func");
        }

        impl TransmogCtx {
            pub fn create_ctx() -> Self {
                secret_func();
                Self {
                    verbose: false,
                    color: None,
                }
            }

            pub fn colorize_ctx(&mut self, color: Color) {
                self.color = match color {
                    Color::Normal => None,
                    Color::Yellow => Some(String::from("\x1b[93m")),
                    Color::Red => Some(String::from("\x1b[91m")),
                };
            }

            pub fn get_color(&self) -> &Option<String> {
                super::gorydetails::this_is_gory(self);

                &self.color
            }
        }
    }
}

use context::TransmogCtx;

impl TransmogCtx {
    pub fn otherfun() {}
}

/// Transmogrify a value including magic
pub fn transmogrify(ctx: &TransmogCtx, to_transmog: i32) -> i32 {
    print_banner(ctx);
    i32::pow(to_transmog, 2) + TRANSMOG_MAGIC * to_transmog
}

fn print_banner(ctx: &TransmogCtx) {
    if ctx.verbose {
        let mut message =
            String::from("You are using the power of transmogrification, be careful!");

        if let Some(escape) = ctx.get_color() {
            message.insert_str(0, escape);
        }

        println!("{}", message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_it_transmogrify() {
        let ctx = TransmogCtx::create_ctx();
        let result = transmogrify(&ctx, 3);
        assert_eq!(result, 30);
    }
}
