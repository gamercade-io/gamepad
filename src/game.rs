use gamercade_rs::prelude::{self as gc, GraphicsParameters};

const PRESSED_COLOR: GraphicsParameters = GraphicsParameters::new().color_index(31);
const RELEASED_COLOR: GraphicsParameters = GraphicsParameters::new().color_index(9);

#[derive(Default)]
pub struct MyGame {
    width: usize,
    height: usize,

    a: bool,
    b: bool,
    c: bool,
    d: bool,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    start: bool,
    select: bool,

    left_axis: (f32, f32),
    right_axis: (f32, f32),
    left_stick: bool,
    right_stick: bool,

    left_shoulder: bool,
    left_trigger: f32,

    right_shoulder: bool,
    right_trigger: f32,
}

impl crate::Game for MyGame {
    fn init() -> Self {
        let mut out = MyGame::default();

        out.width = gc::width();
        out.height = gc::height();

        out
    }

    fn update(&mut self) {
        self.a = gc::button_a_held(0).unwrap_or_default();
        self.b = gc::button_b_held(0).unwrap_or_default();
        self.c = gc::button_c_held(0).unwrap_or_default();
        self.d = gc::button_d_held(0).unwrap_or_default();
        self.up = gc::button_up_held(0).unwrap_or_default();
        self.down = gc::button_down_held(0).unwrap_or_default();
        self.left = gc::button_left_held(0).unwrap_or_default();
        self.right = gc::button_right_held(0).unwrap_or_default();
        self.start = gc::button_start_held(0).unwrap_or_default();
        self.select = gc::button_select_held(0).unwrap_or_default();
        self.left_stick = gc::button_left_stick_held(0).unwrap_or_default();
        self.right_stick = gc::button_right_stick_held(0).unwrap_or_default();
        self.left_shoulder = gc::button_left_shoulder_held(0).unwrap_or_default();
        self.right_shoulder = gc::button_right_shoulder_held(0).unwrap_or_default();

        self.left_axis.0 = gc::analog_left_x(0).unwrap_or_default();
        self.left_axis.1 = gc::analog_left_y(0).unwrap_or_default();

        self.right_axis.0 = gc::analog_right_x(0).unwrap_or_default();
        self.right_axis.1 = gc::analog_right_y(0).unwrap_or_default();

        self.left_trigger = gc::trigger_left(0).unwrap_or_default();
        self.right_trigger = gc::trigger_right(0).unwrap_or_default();
    }

    fn draw(&self) {
        // Clear the screen each frame
        gc::clear_screen(GraphicsParameters::default());

        self.draw_buttons();
        self.draw_dpad();
        self.draw_start_select();
        self.draw_sticks();

        // TODO: Draw Shoulders

        // TODO: Draw Triggers
    }
}

impl MyGame {
    fn get_button_color(&self, field: bool) -> GraphicsParameters {
        if field {
            PRESSED_COLOR
        } else {
            RELEASED_COLOR
        }
    }

    fn draw_buttons(&self) {
        let button_center = ((self.width / 4) * 3, self.height / 2);
        let button_center = (button_center.0 as i32, button_center.1 as i32);
        let button_distance = (self.width / 15) as i32;
        let button_radius = (button_distance / 2) as u32;

        gc::circle(
            self.get_button_color(self.a),
            button_center.0 + button_distance,
            button_center.1,
            button_radius,
        );
        gc::circle(
            self.get_button_color(self.b),
            button_center.0,
            button_center.1 + button_distance,
            button_radius,
        );
        gc::circle(
            self.get_button_color(self.c),
            button_center.0 - button_distance,
            button_center.1,
            button_radius,
        );
        gc::circle(
            self.get_button_color(self.d),
            button_center.0,
            button_center.1 - button_distance,
            button_radius,
        );
    }

    fn draw_start_select(&self) {
        let ss_center = (self.width / 2, self.height / 2);
        let ss_distance = (self.width / 20) as i32;
        let ss_width = (self.width / 20) as u32;
        let ss_height = (self.height / 20) as u32;
        let ss_center = (
            ss_center.0 as i32 - (ss_width as i32 / 2),
            ss_center.1 as i32 - (ss_height as i32 / 2),
        );

        gc::rect(
            self.get_button_color(self.select),
            ss_center.0 - ss_distance,
            ss_center.1,
            ss_width,
            ss_height,
        );
        gc::rect(
            self.get_button_color(self.start),
            ss_center.0 + ss_distance,
            ss_center.1,
            ss_width,
            ss_height,
        );
    }

    fn draw_dpad(&self) {
        let dpad_center = ((self.width / 4), self.height / 2);
        let dpad_distance = (self.width / 15) as i32;
        let dpad_radius = (dpad_distance) as u32;
        let dpad_center = (
            dpad_center.0 as i32 - (dpad_distance / 2),
            dpad_center.1 as i32 - (dpad_distance / 2),
        );

        gc::rect(
            self.get_button_color(self.right),
            dpad_center.0 + dpad_distance,
            dpad_center.1,
            dpad_radius,
            dpad_radius,
        );
        gc::rect(
            self.get_button_color(self.down),
            dpad_center.0,
            dpad_center.1 + dpad_distance,
            dpad_radius,
            dpad_radius,
        );
        gc::rect(
            self.get_button_color(self.left),
            dpad_center.0 - dpad_distance,
            dpad_center.1,
            dpad_radius,
            dpad_radius,
        );
        gc::rect(
            self.get_button_color(self.up),
            dpad_center.0,
            dpad_center.1 - dpad_distance,
            dpad_radius,
            dpad_radius,
        );
    }

    fn draw_sticks(&self) {
        let columns = (self.width / 10) as i32;
        let rows = ((self.height / 5) * 4) as i32;

        self.draw_stick(columns * 4, rows, self.left_axis.0, self.left_axis.1);
        self.draw_stick(columns * 6, rows, self.left_axis.0, self.left_axis.1);
    }

    fn draw_stick(&self, x: i32, y: i32, x_axis: f32, y_axis: f32) {
        let square = (self.width / 8) as i32;

        gc::rect(
            RELEASED_COLOR,
            x - (square / 2),
            y - (square / 2),
            square as u32,
            square as u32,
        );

        let x_pos = ((x_axis * square as f32).round() as i32) + x;
        let y_pos = ((y_axis * square as f32).round() as i32) + y;

        self.draw_cross(PRESSED_COLOR, x_pos, y_pos, 2);
    }

    fn draw_cross(&self, gp: GraphicsParameters, x: i32, y: i32, half_length: i32) {
        gc::line(gp, x - half_length, y, x + half_length, y);
        gc::line(gp, x, y - half_length, x, y + half_length);
    }
}
