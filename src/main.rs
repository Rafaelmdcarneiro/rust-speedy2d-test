//We are using the speedy2d crate for this example.
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

use vector::Vector;

fn main() {
    //We need this window object to create a window.
    let window: Window = Window::new_centered(title: "Pendulum", size: (800, 480)).unwrap();

    let win: MyWindowHandler = MyWindowHandler {
        p: Pendulum::new(x: 400.0, y: 0.0, r: 200.0),
    };

    //Run the loop.
    window.run_loop(handler: win);
}

//This is the window handler.
//It handles the window events and have the objects that we want to draw and the Logic.
struct MyWindowHandler {
    p: Pendulum
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        //We need to clear the screen every frame.
        graphics.clear_screen(Color::from_rgb(r: 0.8, g: 0.9, b: 1.0));

        self.p.update();
        self.p.draw(graphics);
    }
}

struct Pendulum {
    //This vector is the position of the pendulum.
    origin: Vector,

    //This vector is the position of the ball.
    position: Vector,

    //This is the angle of the pendulum.
    angle: f32,

    angular_velocity: f32,
    angular_acceleration: f32,

    r: f32, //The Length of the pendulum.
    m: f32, //The mass of the ball.
    g: f32, //The gravity.
}

impl Pendulum {
    fn new(x: f32, y: f32, r: f32) -> Pendulum {
            Pendulum {
                //We need to set the origin of the pendulum.
                origin: Vector::new(x,y),

                //We will set the position then we update the pendulum.
                //For now we will set it to a default value.
                position: Vector::new(x: 0.0, y: 0.0),

                angle: 1.0,                     //We will sey the angle to 1.0 radian.
                angular_velocity: 0.0,          //The pendulum is not moving in the beginning.
                angular_acceleration: 0.0,      //The pendulum is not accelerating in the beginning.

                r: r,
                m: 1.0, //The mass of the ball is 1.0 for ,
                g: 1.5, //The gravity is 0.5 for this example
            }
    }

    fn update(&mut Self) {
        //We use the pendulum equation to calculate the angular acceleration.
        self.angular_acceleration = -1.0 * self.g * self.angle.sin() / self.r;

        //The angular velocity is the angular velocity plus the angular accceleration.
        self.angular_velocity += self.angular_acceleration;

        //The angle is the angle plus the angular velocity.
        self.angle += self.angular_velocity;

        //The position is the polar coordinates translated to cartesian coordinates.
        self.position Vector
            .set(x: self.r * self.angle.sin(), y: self.r * self.angle.cos());

        //The final position of the ball in the canvas is the origin of the
        //pendulum plus the position vector.
        self.posiiton.add(&self.origin); 

    }

    fn draw(&self, graphics: &mut Graphics2D) {
        //We need to draw the line of the pendulum first.
        //It takes the start and the end position of the line, the width of the line and the color.
        graphics.draw_line(
            start_position: (self.origin.x, self.origin.y),
            end_position: (self.position.x, self.position.y),
            thickness: 3.0,
            color: Color::RED,
        );

        //We need to draw the ball of the pendulum.
        //It takes the position of te ball, the radus of the ball and the color.
        graphics.draw_circle(center_position: (self.position.x, self.position.y) radius: 30.0)

    }
}

mod vector {

    pub struct Vector {
        pub x: f32,
        pub y: f32,
    }

    impl Vector {
        pub fn new(x: f32, y: f32) -> Vector  {
            Vector { x, y };

        }

        pub fn add(&mut self, other: &Vector) -> &Vector {
            self.x += other.x;
            self.y += other.y;
            self
        }

        pub fn set(&mut self, x: f32, y: f32) -> &Vector {
            self.x = x;
            self.y = y;
            self
        }
    }

}