
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

/// This is a demo example file that reads from the Teensy
///
/// Please follow this example for future examples and sanity tests

use embedded_alloc::Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

use teensy4_panic as _;

#[rtic::app(device = teensy4_bsp, peripherals = true, dispatchers = [GPT2])]
mod app {
    use bsp::board;
    use teensy4_bsp as bsp;
    use bsp::hal::gpio::Input;
    use teensy4_pins::t41::*;

    use rtic_monotonics::systick::*;

    #[local]
    struct Local {
        button: Input<P14>,
    }

    #[shared]
    struct Shared {}

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        let board::Resources {
            usb,
            pins,
            mut gpio1,
            ..
        } = board::t41(ctx.device);

        bsp::LoggingFrontend::default_log().register_usb(usb);

        let systick_token = rtic_monotonics::create_systick_token!();
        Systick::start(ctx.core.SYST, 600_000_000, systick_token);

        let button = gpio1.input(pins.p14);

        blink_led::spawn().ok();

        (Shared {}, Local {button})
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    #[task(priority = 1, local = [button])]
    async fn blink_led(_ctx: blink_led::Context) {
        loop {
            if _ctx.local.button.is_set() {
                log::info!("Pressed!");
                Systick::delay(10u32.millis()).await;
            }

            
        }
    }
}
