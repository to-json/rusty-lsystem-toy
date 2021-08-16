extern crate allegro;
extern crate allegro_image;
extern crate allegro_primitives;

use allegro::*;
use allegro_primitives::*;

mod turtle;
mod l_system;
use turtle::Turtle;

allegro_main! {
    let core = Core::init().unwrap();
    let prim = PrimitivesAddon::init(&core).unwrap();
    let disp = Display::new(&core, 2000, 800).unwrap();
    disp.set_window_title("L System Display");

    let timer = Timer::new(&core, 1.0 / 30.0).unwrap();

    let q = EventQueue::new(&core).unwrap();
    q.register_event_source(disp.get_event_source());
    q.register_event_source(timer.get_event_source());

    // let white = core.map_rgb_f(1.0, 1.0, 1.0);
    let black = Color::from_rgb_f(0.0, 0.0, 0.0);
    let system = l_system::create_example_l_system();
    let letters = system.run(16);
    let drawdup = system.draw.clone();
    let draw = drawdup.unwrap();
    let turtle = Turtle { x: 400.0, y: 400.0, angle: 0.0, pen: true };
    let distance: f32 = 10.0;
    let mut theta = 0.0f32;
    let mut redraw = true;
    let next_color: f32 = 360.0 / 4097.0;
    let mut hue: f32 = next_color;
    timer.start();
    'exit: loop {
        if redraw && q.is_empty()
        {
            let handle_letter = |turtle: Turtle, letter: char| -> Turtle {
                match letter {
                    '+' => { turtle.right(system.angle) },
                    '-' => { turtle.left(system.angle) },
                    _ => {
                        if draw.iter().any( |trigger| { &letter == trigger }) {
                            let next = turtle.forward(distance);
                            prim.draw_line( turtle.x, turtle.y,
                                            next.x, next.y,
                                            core.hsl_color( hue, 0.8, 0.5 ), 
                                            1.0);
                            hue += next_color;
                            if hue > 360.0 { hue -= 360.0 };
                            next
                        } else { turtle }
                    }
                }
            };
            core.clear_to_color(black);
            letters.chars().fold(turtle, handle_letter);
            core.flip_display();
            redraw = false;
        }

        match q.wait_for_event()
        {
            DisplayClose{source: src, ..} =>
            {
                assert!(disp.get_event_source().get_event_source() == src);
                    println!("Display close event...");
                    break 'exit;
            },
            KeyDown{keycode: k, ..} if k == KeyCode::Escape =>
            {
                println!("Pressed Escape!");
                break 'exit;
            },
            TimerTick{..} =>
            {
                redraw = true;
                theta = theta + 0.01;
            },
            _ => { }
        }
    }
}


fn  hsv_to_rgb(h: f32, saturation: f32, value: f32) -> [f32; 3] {
    let mut hue = h % 360.0;
    if hue < 0.0 { hue += 360.0 };
    let d: i32 = (hue as i32) / 60 as i32;
    let e: f32 = hue / 60.0 - (d as f32);
    let a: f32 = value * (1.0 - saturation);
    let b: f32 = value * (1.0 - e * saturation);
    let c: f32 = value * (1.0 - (1.0 - e) * saturation);
    match d {
        0 => [value, c, a],
        1 => [b, value, a],
        2 => [a, value, c],
        3 => [a, b, value],
        4 => [c, a, value],
        5 => [value, a, b],
        _ => panic!("Imposible color selected")
    }
}

trait HueColor {
    fn hsv_color(&self, h: f32, s: f32, v: f32) -> Color;
    fn hsl_color(&self, h: f32, s: f32, v: f32) -> Color;
}

impl HueColor for Core {
    fn hsv_color(&self, h: f32, s: f32, v: f32) -> Color { 
        let base = hsv_to_rgb(h,s,v);
        let r = base.get(0).unwrap();
        let g = base.get(1).unwrap();
        let b = base.get(2).unwrap();
        Color::from_rgb_f(*r,*g,*b)
    }

    fn hsl_color(&self, h: f32, s: f32, v: f32) -> Color { 
        let base = hsl_to_rgb(h,s,v);
        let r = base.get(0).unwrap();
        let g = base.get(1).unwrap();
        let b = base.get(2).unwrap();
        Color::from_rgb_f(*r,*g,*b)
    }
}


fn hsl_to_rgb_helper(mut x: f32, a: f32, b: f32) -> f32 {
    if x < 0.0
        { x += 1.0 };
    if x > 1.0
        { x -= 1.0 };

    if x * 6.0 < 1.0
        { return b + (a - b) * 6.0 * x };
    if x * 6.0 < 3.0
        { return a };
    if x * 6.0 < 4.0
        { return b + (a - b) * (4.0 - 6.0 * x) };
    return b;
}


/* Function: al_color_hsl_to_rgb
*/
fn hsl_to_rgb(mut hue: f32, saturation: f32, lightness: f32) -> [f32 ; 3]
{
    hue = hue % 360.0;
    if hue < 0.0
        { hue += 360.0 };
    let h = hue / 360.0;
    let a: f32;
    if lightness < 0.5 { 
        a = lightness + lightness * saturation;
    } else { 
        a = lightness + saturation - lightness * saturation; };
    let b = lightness * 2.0 - a;
    [ hsl_to_rgb_helper(h + 1.0 / 3.0, a, b), hsl_to_rgb_helper(h, a, b), hsl_to_rgb_helper(h - 1.0 / 3.0, a, b) ]
}
