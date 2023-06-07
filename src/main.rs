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
    loop {
        let tick = Instant::now();
        let (row, col) = size()?;
        let mut vec_x = Vec::new();
        let mut vec_y = Vec::new();
        let mut index = 0;
        let mut line = 0;
        // load txt files as squares, no newlines?
        for char in test_map.chars().skip(1) {
            let (_, b1) = 29970u16.overflowing_sub(x - index - row / 2);
            let (_, b2) = 29970u16.overflowing_sub(y - line - col / 2);
            let str_x = 29970u16.saturating_sub(x - index - row / 2);
            let str_y = 29970u16.saturating_sub(y - line - col / 2);
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
                // queue!(stdout, MoveTo(str_x, str_y), Print(char))?;
            }
        }
        // println!("{:?}", test_map.chars().skip(1).count());
        // println!("{:?} {:?}", vec_x.len(), vec_y.len());
        // println!("{} {} {}", row, col, row * (col - 1));
        queue!(stdout, MoveTo(0, 0))?;
        for c in 0..row * col {
            // if (c) != (center_x, center_y) && (!vec_x.contains(&r) || !vec_y.contains(&c)) {
            // if c % row == row {
            //     queue!(stdout, MoveToNextLine(1))?;
            // }
            if c == row * col / 2 {
                queue!(stdout, Print("@"))?;
            } else if c < row * (col - 1) && c != row * col / 2 {
                queue!(stdout, Print("."))?;
            }
            if c == row * (col - 1) {
                queue!(
                    stdout,
                    SetForegroundColor(Color::Black),
                    SetBackgroundColor(Color::White),
                    Print(format!("coords x {} y {}", x, y)),
                    ResetColor
                )?;
                let max_hp = 100;
                let curr_hp = 33;
                let percent_hp = curr_hp * 100 / max_hp;
                let bar_length = row - 22;
                let hp_length = (percent_hp * bar_length) / 100;
                // println!("{} {} {}", hp_length, bar_length, percent_hp);
                for _ in 22..row {
                    queue!(
                        stdout,
                        SetBackgroundColor(Color::DarkRed),
                        Print(" "),
                        ResetColor
                    )?; // ResetColor for 0 hp
                }
                queue!(stdout, MoveTo(22, col - 1))?;
                for _ in 22..22 + hp_length {
                    queue!(
                        stdout,
                        SetBackgroundColor(Color::Green),
                        Print(" "),
                        ResetColor
                    )?;
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
                            KeyCode::Esc | KeyCode::Char('q') => {
                                execute!(stdout, LeaveAlternateScreen, Show)?;
                                break disable_raw_mode();
                            }
                            _ => (),
                        }
                    }
                    // try to restore last held key for smooth movement
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
