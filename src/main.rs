mod game;
mod rng;
mod noise;
mod heightmap;
mod sim;
mod matrix;
mod delaunay;

fn main() {
    unsafe {
        let event_loop = glutin::event_loop::EventLoop::new();
        let mut game = game::Game::new(&event_loop);
        event_loop.run(move |event, _, _| game.handle_event(event));
    }
}

// pub fn main() {
//     let event_loop = glutin::event_loop::EventLoop::new();
//     let mut game = Game::new(&event_loop);
//     event_loop.run(move |event, _, _| game.handle_event(event));
// }