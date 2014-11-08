#![feature(globs)]
#![feature(struct_variant)]
#![feature(phase)]

#[phase(plugin, link)]
extern crate allegro;
extern crate allegro_image;
extern crate allegro_primitives;

use std::c_str::*;
use allegro::*;
use allegro_primitives::*;


allegro_main! {
  let core = Core::init().unwrap();
  let prim = PrimitivesAddon::init(&core).unwrap();
  
  let disp = Display::new(&core, 800, 600).unwrap();
  disp.set_window_title(&"LSD".to_c_str());

  let timer = Timer::new(&core, 1.0 / 60.0).unwrap();

  let q = EventQueue::new(&core).unwrap();
  q.register_event_source(disp.get_event_source());
  q.register_event_source(timer.get_event_source());

  let white = core.map_rgb_f(1.0, 1.0, 1.0);
  let black = core.map_rgb_f(0.0, 0.0, 0.0);

  let mut theta = 0.0f32;
  let mut redraw = true;

  fn moving_line(theta: f32, prim: &PrimitivesAddon, color: Color) {
    prim.draw_line((100.0 + (theta * 100.0)), 200.0, (300.0 + (theta * 100.0)), 200.0, color, 1.0);
  }
  fn draw_box(prim: &PrimitivesAddon, x1: f32, y1: f32, x2: f32, y2: f32, color: Color, theta: f32) {
    let y1 = y1 + (100.0 * theta);
    let y2 = y2 + (100.0 * theta);
    prim.draw_line(x1,y1,x2,y1,color,1.0);
    prim.draw_line(x2,y1,x2,y2,color,1.0);
    prim.draw_line(x2,y2,x1,y2,color,1.0);
    prim.draw_line(x1,y2,x1,y1,color,1.0);
  }

  timer.start();
  'exit: loop {
    if redraw && q.is_empty()
    {
      core.clear_to_color(black);

      draw_box(&prim, 100.0, 100.0, 200.0, 200.0, white, theta);
      moving_line(theta, &prim, white);
      disp.flip();
      redraw = false;
    }

    match q.wait_for_event()
    {
      DisplayClose{source: src, ..} =>
      {
        assert!(disp.get_event_source().get_event_source() == src)
        println!("Display close event...")
        break 'exit;
      },
      KeyDown{keycode: k, ..} if k == key::Escape =>
      {
        println!("Pressed Escape!");
        break 'exit;
      },
      TimerTick{..} =>
      {
        redraw = true;
        theta = theta + 0.01;
      },
      _ => { ; }
    }
  }
}
