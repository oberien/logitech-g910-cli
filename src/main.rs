extern crate argparse;
extern crate g910;
extern crate g910_handler;

use std::io::Write;
use argparse::{ArgumentParser, StoreTrue};
use g910::{KeyboardImpl, Keyboard};
use g910_handler::{FlashHandler, UinputHandler, HeatmapHandler, Snake};

fn main() {
    let mut use_flash_handler = false;
    let mut use_uinput_handler = false;
    let mut use_heatmap_handler = false;
    let mut use_snake = false;
    let mut handle_signals = false;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Logitech G910");
        ap.refer(&mut use_flash_handler)
            .add_option(&["-f", "--flash"], StoreTrue,
            "Enable FlashHandler");
        ap.refer(&mut use_uinput_handler)
            .add_option(&["-u", "--uinput"], StoreTrue,
            "Enable UinputHandler");
        ap.refer(&mut use_heatmap_handler)
            .add_option(&["--heatmap"], StoreTrue,
            "Enable HeatmapHandler");
        ap.refer(&mut use_snake)
            .add_option(&["-s", "--snake"], StoreTrue,
            "Play Snake");
        ap.refer(&mut handle_signals)
            .add_option(&["--signals"], StoreTrue,
            "React to system signals (SIGINT, SIGTERM)");
        ap.parse_args_or_exit();
    }

    if !use_flash_handler && !use_uinput_handler && !use_heatmap_handler && !use_snake {
        print!("No handler selected. Is this correct? [Y/n]: ");
        ::std::io::stdout().flush().unwrap();
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        line = line.to_lowercase();
        if line != "\n" && line != "y\n" && line != "yes\n" {
            ::std::process::exit(0);
        }
    }
    let mut keyboard = KeyboardImpl::new().unwrap();
    if use_flash_handler {
        keyboard.add_handler(FlashHandler::new().into());
    }
    if use_uinput_handler {
        keyboard.add_handler(UinputHandler::new().into());
    }
    if use_heatmap_handler {
        keyboard.add_handler(HeatmapHandler::new().into());
    }
    if use_snake {
        keyboard.add_handler(Snake::new().into());
    }
    if handle_signals {
        unsafe { keyboard.enable_signal_handling().unwrap() };
    }
    keyboard.start_handle_loop().unwrap();
}
