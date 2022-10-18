use ansi_style::{Color, Style};

fn main() {
    // You can either color the text directly with the Color enumeration
    println!(
        "{}Cyan colored \"Hello World!\"{}",
        Color::Cyan.open(),
        Color::Cyan.close()
    );

    // or you can use the builder function from within the Style stuct
    // to create a style that can be used for more than one instance of
    // a string and you wouldn't need to have an open and close function
    // prepended and appended to every text you type like the above example

    let style = Style::builder().red().strikethrough().build();

    println!(
        "{}",
        style.stylize("Hello World in red with strikethrough")
    )
}
