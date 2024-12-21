use i_drive::{screen_init, screen_clear, screen_end};

const NODE_ADDRESS: u8 = 1;

fn main() {
    screen_init(NODE_ADDRESS);

    screen_clear();

    screen_end();
}