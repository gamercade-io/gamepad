use gamercade_rs::{
    prelude::{self as gc},
    GraphicsParameters,
};

#[derive(Default)]
pub struct MyGame {
    width: usize,
    height: usize,

    pressed_color: GraphicsParameters,
    released_color: GraphicsParameters,

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

        out.pressed_color = GraphicsParameters::default().color_index(31);
        out.released_color = GraphicsParameters::default().color_index(9);

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
        gc::clear_screen(GraphicsParameters::default());

        // Draw the ABCD Buttons
        let button_center = ((self.width / 4) * 3, self.height / 2);
        let button_center = (button_center.0 as i32, button_center.1 as i32);
        let button_distance = (self.width / 12) as i32;
        let button_radius = (button_distance / 3) as u32;

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

        // TODO: Draw Start & Select

        // TODO: Draw Dpad

        // TODO: Draw Left & Right Stick

        // TODO: Draw Shoulders

        // TODO: Draw Triggers
    }
}

impl MyGame {
    fn get_button_color(&self, field: bool) -> GraphicsParameters {
        if field {
            self.pressed_color
        } else {
            self.released_color
        }
    }
}
