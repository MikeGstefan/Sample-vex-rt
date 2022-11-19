#![no_std]
#![no_main]

use core::time::Duration;

use vex_rt::{prelude::*, select};



struct MyRobot {
    
}

impl Robot for MyRobot {
    fn new(peripherals: Peripherals) -> Self {
        Self {
            //Declare the robot
        }
    }

    fn initialize(&'static self, _ctx: Context) {
        // Do any extra initialization here.
    }

    fn autonomous(&'static self, _ctx: Context) {
        // Write your autonomous routine here.

    }

    fn opcontrol(&'static self, ctx: Context) {
        //Write your opcontrol routine here.
    }

    fn disabled(&'static self, _ctx: Context) {
        // This runs when the robot is in disabled mode.

    }
}

entry!(MyRobot);
