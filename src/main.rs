use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    Paused,
    End,
}

struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        //fill in later
        self.mode = GameMode::End;
    }

    fn main_menu(&mut self, ctx: &Mut BTerm) {
        ctx.cls();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::Paused => self.paused(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Rusty Birb")
        .build()?;

    main_loop(context, State::new())
}
