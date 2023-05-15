use std::{
    io::{stdout, BufWriter, Write},
    thread::sleep,
    time::{Duration, Instant},
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, poll, Event, KeyCode, KeyEventKind},
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen, SetTitle},
    Result,
};

fn main() -> Result<()> {
    const FPS: u32 = 30;
    const FRAME: Duration = Duration::new(0, 1_000_000_000 / FPS);
    let mut stdout = BufWriter::with_capacity(64_000, stdout());
    let (mut x, mut y, mut xd, mut yd, mut px, mut py) = (30_000, 30_000, 0, 0, false, false);
    execute!(stdout, Hide, EnterAlternateScreen)?;
    // enable_raw_mode()?;

    let test_map = r"
                          #############################
               ###########   |------------------|      ################
          #####              |                  |                      ###############
        ##                   |                  |                                     ########
      ##   |--|              |--/------------/--|                                |            ####
    ##     |> /    *                                                             |                ##
   #       |--|                                                                                     ##
   #                                                                             |       |-------|    #
  #   |--------|  |--------|                                *                            |       |     #
 #*   |      , |  |        |                                                     |       |---+---|     #
#     |        |  |        |                                                     |                     #
#     |---/----|  |------/-|                                                     |                     #
#                                                                                                     #
#  |--/-----|                        *                                                               #
#  |        |                                                                                       #
#  |        |                                                                                  ----#
#  |--------|                                                                                     #
#                                                           *                                    #
#                                                                                                #
#                   *                                                                           #
#                                                                           *                  #
#                                                                                             #
#-------------------------|                                                                  #
#            *  *  *  *  *|                                                                ##
#     |--|         *  *  *|                                                               #
#  |--|  |------|         |                                                             ##
#  |*           /                                                                      #
#  |--|  |------|                                                                     #
#    >|--|         *  *  *|                                                       ####
#            *  *  *  *  *|                                                    ###
###########################                                                   #
                           ##                                              ###
                             ##                          *                #
                               ###                                      ##
                                  ####                                 #
                                      #####                          ##
                                           ##########################%";
    // try to restore last held key for smooth movement
    // redo screen drawing to not draw extra chars on the right (why does it happen?)
    // panic hook or ctrl-c to restore cursor and leave alt screen
    loop {
        let tick = Instant::now();
        let (row, col) = size()?;
        let (center_x, center_y) = (row / 2, col / 2);
        let mut vec_x = Vec::new();
        let mut vec_y = Vec::new();
        let mut index = 0;
        let mut line = 0;
        for char in test_map.chars().skip(1) {
            let (_, b1) = 29970u16.overflowing_sub(x - index - center_x);
            let (_, b2) = 29970u16.overflowing_sub(y - line - center_y);
            let str_x = 29970u16.saturating_sub(x - index - center_x);
            let str_y = 29970u16.saturating_sub(y - line - center_y);
            // temp?
            if char != ' ' && char != '\n' {
                vec_x.push(str_x);
                vec_y.push(str_y);
            }
            index += 1;
            if char == '\n' {
                line += 1;
                index = 0;
            }
            if str_x < row
                && str_y < col - 1
                && (!b1 && str_x == 0 || !b2 && str_y == 0 || str_x > 0 && str_y > 0)
            {
                queue!(stdout, MoveTo(str_x, str_y), Print(char))?;
            }
        }
        // println!("{:?}", test_map.chars().skip(1).count());
        println!("{:?} {:?}", vec_x.len(), vec_y.len());
        for c in 0..col - 1 {
            for r in 0..row {
                if (r, c) != (center_x, center_y) && (!vec_x.contains(&r) || !vec_y.contains(&c)) {
                    // need to store all chars to check if coord is already empty space?
                    queue!(stdout, MoveTo(r, c), Print(" "))?;
                    // if r == 0 || r == row {
                    //     queue!(stdout, MoveTo(r, c), Print(' '))?;
                    // }
                    // if r == 1 || r == row - 1 {
                    //     queue!(stdout, MoveTo(r, c), Print("│"))?; // ─
                    // }

                    // if c == col - 1 {
                    // queue!(
                    // stdout,
                    // MoveTo(r, c),
                    // SetForegroundColor(Color::Red),
                    // SetBackgroundColor(Color::Green),
                    // SetColors(Colors::new(Color::DarkRed, Color::DarkGreen)),
                    // Print('█'),
                    // ResetColor,
                    // )?;
                    // }
                    if r == 0 && c == col - 1 {
                        queue!(
                            stdout,
                            MoveTo(r, c),
                            SetForegroundColor(Color::Black),
                            SetBackgroundColor(Color::White),
                            Print(format!("coords x {} y {}", x, y)),
                            ResetColor
                        )?;
                    }
                    if r == 22 && c == col - 1 {
                        let max_hp = 100;
                        let curr_hp = 33;
                        let percent_hp = curr_hp * 100 / max_hp;
                        let bar_length = row - 22;
                        let hp_length = (percent_hp * bar_length) / 100;
                        // println!("{} {} {}", hp_length, bar_length, percent_hp);
                        for r in 22..row {
                            queue!(
                                stdout,
                                MoveTo(r, c),
                                SetBackgroundColor(Color::DarkRed),
                                Print(" "),
                                ResetColor
                            )?; // ResetColor for 0 hp
                        }
                        for r in 22..22 + hp_length {
                            queue!(
                                stdout,
                                MoveTo(r, c),
                                SetBackgroundColor(Color::Green),
                                Print(" "),
                                ResetColor
                            )?;
                        }
                    }
                    // queue!(stdout, MoveTo(r, c), Print("."))?;
                }
            }
        }
        // let loc_x = 30024u16.saturating_sub(x);
        // let loc_y = 30024u16.saturating_sub(y);
        // let string = String::from("#!1234567890SAPC");
        // for (index, char) in string.chars().enumerate() {
        //     let str_x = 30030u16.saturating_sub(x - index as u16);
        //     let str_y = 30030u16.saturating_sub(y);
        //     if str_x > 0 && str_y > 0 && str_x < row && str_y < col {
        //         queue!(stdout, MoveTo(str_x, str_y), Print(char))?;
        //     }
        // }
        // if loc_x > 0 && loc_y > 0 && loc_x < row && loc_y < col {
        //     queue!(stdout, MoveTo(loc_x, loc_y), Print("L"))?;
        // }
        // println!("{:?}", test_map.chars().enumerate());
        // let mut line = 0;
        // let mut idx = 0;
        // for char in test_map.chars().skip(1) {
        //     idx += 1;
        //     if char == '\n' {
        //         line += 1;
        //         idx = 0;
        //     }
        // if loc_x + idx > 0 && loc_y + line > 0 {
        // queue!(stdout, MoveTo(loc_x + idx, loc_y + line), Print(char))?;
        // }
        // }
        // for (index, line) in test_map.split('\n').enumerate() {
        //     queue!(stdout, MoveTo(loc_x, loc_y + index as u16), Print(line))?;
        // }
        // for (index, line) in map.split('\n').enumerate() {
        //     queue!(
        //         stdout,
        //         MoveTo(30050 - x, 30030 - y + index as u16),
        //         Print(line)
        //     )?;
        // }
        queue!(stdout, MoveTo(center_x, center_y), Print("@"))?;
        if tick.elapsed() < FRAME {
            if poll(FRAME - tick.elapsed())? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Left => {
                                xd = 2;
                                yd = 0;
                                px = false;
                            }
                            KeyCode::Right => {
                                xd = 2;
                                yd = 0;
                                px = true;
                            }
                            KeyCode::Up => {
                                yd = 1;
                                xd = 0;
                                py = false;
                            }
                            KeyCode::Down => {
                                yd = 1;
                                xd = 0;
                                py = true;
                            }
                            KeyCode::Esc => {
                                execute!(stdout, LeaveAlternateScreen, Show)?;
                                break disable_raw_mode();
                            }
                            _ => (),
                        }
                    }
                    if key.kind == KeyEventKind::Release {
                        match key.code {
                            KeyCode::Left => {
                                if xd > 0 && !px {
                                    xd = 0
                                }
                            }
                            KeyCode::Right => {
                                if xd > 0 && px {
                                    xd = 0
                                }
                            }
                            KeyCode::Up => {
                                if yd > 0 && !py {
                                    yd = 0;
                                }
                            }
                            KeyCode::Down => {
                                if yd > 0 && py {
                                    yd = 0;
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }
            if xd > 0 || yd > 0 {
                match px {
                    false => {
                        // if x > 1 {
                        // because of 2 char steps
                        x -= xd;
                        // }
                    }
                    true => {
                        // if x < row - 2 {
                        x += xd;
                        // }
                    }
                }
                match py {
                    false => {
                        // if y > 0 {
                        y -= yd;
                        // }
                    }
                    true => {
                        // if y < col - 1 {
                        y += yd;
                        // }
                    }
                }
            }
            if tick.elapsed() < FRAME {
                sleep(FRAME - tick.elapsed());
            }
            queue!(
                stdout,
                SetTitle(format!(
                    "Atventure {:02} FPS",
                    // 1000_u128.div_ceil(tick.elapsed().as_millis())
                    1000 / tick.elapsed().as_millis()
                )),
                Hide, // needed for powershell/cmd?
            )?;
        }
        stdout.flush()?;
    }
}
