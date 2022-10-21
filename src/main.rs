#![no_std]
#![no_main]

use panic_halt as _;
use attiny_hal as hal;
use hal::prelude::*;

enum Symbols {
    Dot,
    Dash,
    ShortGap,
    LongGap,
}

static SYMBOLS : [Symbols; 24] =
    [
	Symbols::Dot,  Symbols::Dot,  Symbols::Dash, Symbols::Dash, Symbols::Dash, Symbols::ShortGap,
	Symbols::Dot,  Symbols::Dash, Symbols::Dash, Symbols::Dash, Symbols::Dash, Symbols::ShortGap,
	Symbols::Dot,  Symbols::Dot,  Symbols::Dot,  Symbols::Dash, Symbols::Dash, Symbols::ShortGap,
	Symbols::Dash, Symbols::Dash, Symbols::Dot,  Symbols::Dot,  Symbols::Dot,  Symbols::LongGap, 
    ];
static DOT: u16 = 100; // ms

#[hal::entry]
fn main() -> ! {
    let dp = hal::pac::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);
    
    let mut delay = hal::delay::Delay::<hal::clock::MHz1>::new();
    let mut led = pins.pb4.into_output();
    led.set_low();
    
    loop {
	for s in &SYMBOLS {
	    let (h, l) = match s {
		Symbols::Dot => (1, 1),
		Symbols::Dash => (3, 1),
		Symbols::ShortGap => (0, 2),
		Symbols::LongGap => (0, 6)
	    };
	    if h > 0 {
		led.toggle();
		delay.delay_ms(h * DOT);
		led.toggle();
	    }
	    delay.delay_ms(l * DOT);
	}
    }
}

	
