#[cfg(test)]
mod tests {
    use colorize_rs::AnsiColor;

    #[test]
    fn all() {
        //colorize::global_fg(Red);
        //colorize::global_bg(Greenb);
        println!("{}", "\tTest foreground color for strbuf".b_green());
        crate::tests::foreground_color_strbuf();
        println!("{}", "\tTest background color for strbuf".greenb());
        crate::tests::background_color_strbuf();
        println!("{}", "\tTest foreground color for &'static str".b_green());
        crate::tests::foreground_color_ref_str();
        println!("{}", "\tTest background color for &'static str".greenb());
        crate::tests::background_color_ref_str();
        println!("{}", "\tTest custom styles for owned str".b_green());
        crate::tests::custom_styles_strbuf();
        println!("{}", "\tTest custom styles for ref str".b_green());
        crate::tests::custom_styles_ref_str();
        crate::tests::final_test();
    }
    #[test]
    fn foreground_color_strbuf() {
        println!("{}", "Black".to_string().black());
        println!("{}", "Bright black".to_string().b_black());
        println!("{}", "Red".to_string().red());
        println!("{}", "Bright Red".to_string().b_red());
        println!("{}", "Green".to_string().green());
        println!("{}", "Bright Green".to_string().b_green());
        println!("{}", "Yellow".to_string().yellow());
        println!("{}", "Bright Yellow".to_string().b_yellow());
        println!("{}", "Blue".to_string().blue());
        println!("{}", "Bright Blue".to_string().b_blue());
        println!("{}", "Magenta".to_string().magenta());
        println!("{}", "Bright Magenta".to_string().b_magenta());
        println!("{}", "Cyan".to_string().cyan());
        println!("{}", "Bright Cyan".to_string().b_cyan());
        println!("{}", "Grey".to_string().grey());
        println!("{}", "Bright Grey".to_string().b_grey());
        println!("{}", "Hello world".to_string().default());
    }
    #[test]
    fn background_color_strbuf() {
        println!("{}", "Black".to_string().blackb());
        println!("{}", "Bright black".to_string().b_blackb());
        println!("{}", "Red".to_string().redb());
        println!("{}", "Bright Red".to_string().b_redb());
        println!("{}", "Green".to_string().greenb());
        println!("{}", "Bright Green".to_string().b_greenb());
        println!("{}", "Yellow".to_string().yellowb());
        println!("{}", "Bright Yellow".to_string().b_yellowb());
        println!("{}", "Blue".to_string().blueb());
        println!("{}", "Bright Blue".to_string().b_blueb());
        println!("{}", "Magenta".to_string().magentab());
        println!("{}", "Bright Magenta".to_string().b_magentab());
        println!("{}", "Cyan".to_string().cyanb());
        println!("{}", "Bright Cyan".to_string().b_cyanb());
        println!("{}", "Grey".to_string().greyb());
        println!("{}", "Bright Grey".to_string().b_greyb());
        println!("{}", "Hello world".to_string().defaultb());
    }
    #[test]
    fn foreground_color_ref_str() {
        println!("{}", "Black".black());
        println!("{}", "Bright black".b_black());
        println!("{}", "Red".red());
        println!("{}", "Bright Red".b_red());
        println!("{}", "Green".green());
        println!("{}", "Bright Green".b_green());
        println!("{}", "Yellow".yellow());
        println!("{}", "Bright Yellow".b_yellow());
        println!("{}", "Blue".blue());
        println!("{}", "Bright Blue".b_blue());
        println!("{}", "Magenta".magenta());
        println!("{}", "Bright Magenta".b_magenta());
        println!("{}", "Cyan".cyan());
        println!("{}", "Bright Cyan".b_cyan());
        println!("{}", "Grey".grey());
        println!("{}", "Bright Grey".b_grey());
        println!("{}", "Hello world".default());
    }
    #[test]
    fn background_color_ref_str() {
        println!("{}", "Black".blackb());
        println!("{}", "Bright black".b_blackb());
        println!("{}", "Red".redb());
        println!("{}", "Bright Red".b_redb());
        println!("{}", "Green".greenb());
        println!("{}", "Bright Green".b_greenb());
        println!("{}", "Yellow".yellowb());
        println!("{}", "Bright Yellow".b_yellowb());
        println!("{}", "Blue".blueb());
        println!("{}", "Bright Blue".b_blueb());
        println!("{}", "Magenta".magentab());
        println!("{}", "Bright Magenta".b_magentab());
        println!("{}", "Cyan".cyanb());
        println!("{}", "Bright Cyan".b_cyanb());
        println!("{}", "Grey".greyb());
        println!("{}", "Bright Grey".b_greyb());
        println!("{}", "Hello world".defaultb());
    }
    #[test]
    fn custom_styles_strbuf() {
        println!("{}", "Hello world".to_string().underlined());
        println!("{}", "hello world".to_string().bold());
        println!("{}", "Hello world".to_string().blink());
        println!("{}", "Hello world".to_string().reverse());
        println!("{}", "Hello world".to_string().concealed());
    }
    #[test]
    fn custom_styles_ref_str() {
        println!("{}", "Hello world".underlined());
        println!("{}", "Hello world".bold());
        println!("{}", "Hello world".blink());
        println!("{}", "Hello world".reverse());
        println!("{}", "Hello world".concealed());
    }
    #[test]
    fn final_test() {
        println!(
            "{}",
            "Super final test combo !"
                .magenta()
                .blink()
                .b_yellowb()
                .underlined()
        );
    }
}
