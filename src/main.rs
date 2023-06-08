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
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
        SetTitle,
    },
    Result,
};

fn main() -> Result<()> {
    const FPS: u32 = 30;
    const FRAME: Duration = Duration::new(0, 1_000_000_000 / FPS);
    let mut stdout = BufWriter::with_capacity(64_000, stdout());
    let (mut x, mut y, mut xd, mut yd, mut px, mut py) = (30_000, 30_000, 0, 0, false, false);
    execute!(stdout, Hide, EnterAlternateScreen)?;

    let _test_map = r"
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
        enable_raw_mode()?;
        let tick = Instant::now();
        let (row, col) = size()?;
        // println!("{:?}", test_map.chars().skip(1).count());
        // println!("{:?} {:?}", vec_x.len(), vec_y.len());
        // let mut vec_x = Vec::new();
        // let mut vec_y = Vec::new();
        // let mut index = 0;
        // let mut line = 0;
        // for char in test_map.chars().skip(1) {
        //     let (_, b1) = 29970u16.overflowing_sub(x - index - row / 2);
        //     let (_, b2) = 29970u16.overflowing_sub(y - line - col / 2);
        //     let str_x = 29970u16.saturating_sub(x - index - row / 2);
        //     let str_y = 29970u16.saturating_sub(y - line - col / 2);
        //     // temp?
        //     if char != ' ' && char != '\n' {
        //         vec_x.push(str_x);
        //         vec_y.push(str_y);
        //     }
        //     index += 1;
        //     if char == '\n' {
        //         line += 1;
        //         index = 0;
        //     }
        //     if str_x < row
        //         && str_y < col - 1
        //         && (!b1 && str_x == 0 || !b2 && str_y == 0 || str_x > 0 && str_y > 0)
        //     {
        //         queue!(stdout, MoveTo(str_x, str_y), Print(char))?;
        //     }
        // }
        queue!(stdout, MoveTo(0, 0))?;
        // load txt files as squares, no newlines?
        for c in 0..row * col {
            // if (!vec_x.contains(&c) || !vec_y.contains(&c))
            if c == row * (col / 2) + row / 2 {
                queue!(stdout, Print("@"))?;
                // .saturating_sub(x.saturating_mul(col))
                // .saturating_add(y.saturating_mul(row))
                // println!(
                //     "{}",
                //     29970u16
                //         .saturating_sub(((y as u32 * row as u32) + x as u32) as u16)
                //         .saturating_sub(row * (col / 2) + row / 2)
                // );
            } else if c == 29970u16.saturating_sub(((y as u32 * row as u32) + x as u32) as u16)
            // .saturating_sub(row * (col / 2) + row / 2)
            // || c == 0 && (0, false) == 29970u16.overflowing_sub(x - row / 2)
            {
                queue!(stdout, Print("A"))?;
            } else if c < row * (col - 1) {
                queue!(stdout, Print("."))?;
            }
            if c == row * (col - 1) {
                queue!(
                    stdout,
                    SetForegroundColor(Color::Black),
                    SetBackgroundColor(Color::White),
                    Print(format!("coords X {:<5} Y {:<5}", x, y)),
                    ResetColor
                )?;
                let max_hp = 100;
                let curr_hp = 33;
                let percent_hp = curr_hp * 100 / max_hp;
                let bar_length = row - 22;
                let hp_length = (percent_hp * bar_length) / 100;
                // println!("{} {} {}", hp_length, bar_length, percent_hp);
                queue!(stdout, MoveTo(22, col - 1))?;
                for _ in 22..22 + hp_length {
                    queue!(
                        stdout,
                        SetBackgroundColor(Color::Green),
                        Print(" "),
                        ResetColor
                    )?;
                }
                for _ in 22 + hp_length..row {
                    queue!(
                        stdout,
                        SetBackgroundColor(Color::DarkRed),
                        Print(" "),
                        ResetColor
                    )?; // ResetColor for 0 hp
                }
            }
        }
        disable_raw_mode()?;
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
                    // TODO: try to restore last held key for smooth movement
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
                        // because of 2 char steps
                        if x > 1 {
                            x -= xd;
                        }
                    }
                    true => {
                        // because of 2 char steps
                        if x < u16::MAX - 1 {
                            x += xd;
                        }
                    }
                }
                match py {
                    false => {
                        if y > 0 {
                            y -= yd;
                        }
                    }
                    true => {
                        if y < u16::MAX {
                            y += yd;
                        }
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
                Hide, // needed for powershell/cmd
            )?;
        }
        stdout.flush()?;
    }
}
