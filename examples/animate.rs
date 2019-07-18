use piston_window::*;
use sdl2_window::Sdl2Window;

use ss5_piston::ssa::SsaJson;

fn main() {
    let mut window: PistonWindow<Sdl2Window> =
        WindowSettings::new("SpriteStudio5 on Piston", [800.0, 600.0])
            .build()
            .unwrap();

    let mut a = SsaJson::<G2d>::open("datas/animetest.json").unwrap();

    const MOTION: usize = 0;

    let mut c = window.create_texture_context();
    a.allocate_texture_for(&mut c, MOTION);

    window.set_size(a.get_size_of(MOTION));

    // イベントループ
    while let Some(evt) = window.next() {
        if let Some(args) = evt.update_args() {
            let delta_time = args.dt;
            a.update(delta_time);
        }

        if let Some(v) = evt.render_args() {
            let [w, h] = v.draw_size;
            let (w, h) = (f64::from(w), f64::from(h));

            window.draw_2d(&evt, |c, g, _d| {
                clear([1.0, 1.0, 1.0, 1.0], g);
                a.draw(c.transform.trans(w * 0.5, h * 0.5), g, MOTION);
            });
        }

        if let Some(_) = evt.press_args() {
            a.play_once();
        }
    }
}
