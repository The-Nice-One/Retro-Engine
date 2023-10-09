

use retro_engine::components::trait_def::Component;
use retro_engine::{scene::Scene};

use retro_engine::components::{text::Text, fancy_text::FancyText, grid::Grid, vertical_line::VerticalLine};


use retro_engine::core::Terminal;


fn main() {

    let mut terminal = Terminal::init();

    let mut my_scene = Scene::new();
    

    let title = FancyText::new("Retro");

    let bar = VerticalLine::new(" | ", 6);

    let paragraph = Text::new("this is some text that\nshould be formatted correctly :)");

    let mut display = retro_engine::scene::allign_horizontally(title.display(), bar.display());

    display = retro_engine::scene::allign_horizontally(display, paragraph.display());
    println!("{}", display);

    loop {
        terminal.poll_events(500);
    }
    

    // print_events().unwrap();
}