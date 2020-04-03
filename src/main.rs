use sdl2::{
    pixels::Color,
    event::Event,
    render::Canvas,
    video::Window
};
use spin_sleep::LoopHelper;

static WHITE: Color = Color::RGB(255,255,255);
static BLACK: Color = Color::RGB(0,0,0);

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn rotate_right(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down
    }
}

fn rotate_left(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Left,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
        Direction::Right => Direction::Up
    }
}

struct Ant {
    position: (usize, usize),
    direction: Direction
}

fn move_ant(ant: Ant) -> Ant {
    match ant.direction {
        Direction::Up =>    Ant {position: (ant.position.0, ant.position.1 - 1), .. ant},
        Direction::Down =>  Ant {position: (ant.position.0, ant.position.1 + 1), .. ant},
        Direction::Left =>  Ant {position: (ant.position.0 - 1, ant.position.1), .. ant},
        Direction::Right => Ant {position: (ant.position.0 + 1, ant.position.1), .. ant}
    }
}

fn do_ant(mut ant: Ant, grid: &mut [[bool; 720]; 1280], canvas: &mut Canvas<Window>) -> Result<Ant, String> {
    let antx = ant.position.0;
    let anty = ant.position.1;

    if antx >= 1280 || anty >= 720 {
        Err(String::from("Ant is out of bounds."))
    } else {
        if grid[antx][anty] { // is black
            ant.direction = rotate_left(ant.direction);
            canvas.set_draw_color(WHITE);
            canvas.draw_point((antx as i32, anty as i32))?;
        } else { // if it's white
            ant.direction = rotate_right(ant.direction);
            canvas.set_draw_color(BLACK);
            canvas.draw_point((antx as i32, anty as i32))?;
        }

        grid[antx][anty] = !grid[antx][anty];
        Ok(move_ant(ant))
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("ANT", 1280, 720)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let mut loop_helper = LoopHelper::builder().build_with_target_rate(60);
    let mut event_pump = sdl_context.event_pump()?;

    let mut ant = Ant {position: (640, 360), direction: Direction::Left};
    let mut grid: [[bool; 720]; 1280] = [[false; 720]; 1280]; // this is like, 921KB worth of bool

    'running: loop {
        loop_helper.loop_start();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'running,
                _ => ()
            }
        }

        ant = do_ant(ant, &mut grid, &mut canvas)?;
        canvas.present();

        loop_helper.loop_sleep();
    }

    Ok(())
}
