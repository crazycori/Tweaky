use cfonts;

use cfonts::{ say, Options, Align, BgColors, Colors, Env, Fonts, Rgb };

pub fn title() {
    say(Options {
        text: String::from("Tweaky"),
        font: Fonts::FontBlock,
        colors: vec![Colors::System],
        background: BgColors::Transparent,
        align: Align::Center,
        letter_spacing: 1,
        line_height: 1,
        spaceless: false,
        max_length: 0,
        gradient: Vec::new(),
        independent_gradient: false,
        transition_gradient: false,
        env: Env::Cli,
        ..Options::default()
    });
}