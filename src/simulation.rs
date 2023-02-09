use crate::util::mutate;
use crate::world::{Particle, World};

pub(crate) fn tick(world: &mut World, delta_micro: f32) {
    let sub_steps = 10;
    let delta = delta_micro.max(1.0) / 100_000.0 / sub_steps as f32;
    let delta_sq = delta * delta;

    for _ in 0..sub_steps {
        // apply gravity
        let gx = 0.0;
        let gy = 9.81 * 3.0;
        for x in 0..world.cells.len() {
            for y in 0..world.cells[x].len() {
                for p in &mut world.cells[x][y] {
                    p.ax += gx;
                    p.ay += gy;
                }
            }
        }

        // collisions
        let response_coef = 0.75;
        for x in 1..(world.cells.len()-1){
            for y in 1..(world.cells[x].len()-1) {
                let near = [
                    &world.cells[x-1][y-1], &world.cells[x][y-1], &world.cells[x+1][y-1],
                    &world.cells[x-1][y], &world.cells[x][y], &world.cells[x+1][y],
                    &world.cells[x-1][y+1], &world.cells[x][y+1], &world.cells[x+1][y+1]
                ].into_iter().flatten().collect::<Vec<&Particle>>();
                if near.len() > 1 {
                    // (i, k) collision pairs
                    for i in 0..near.len() {
                        let p1 = unsafe {mutate(*near.get_unchecked(i))};
                        for k in (i+1)..near.len() {
                            let p2 = unsafe {mutate(*near.get_unchecked(k))};
                            let dx = p1.x - p2.x;
                            let dy = p1.y - p2.y;
                            let dsq = dx * dx + dy * dy;
                            // min_dist = 1, r = 0.5
                            if dsq < 1.0 {
                                let d = dsq.sqrt();
                                let nx = dx / d;
                                let ny = dy / d;
                                let dm = 0.5 * response_coef * (d - 1.0);
                                p1.x -= nx * 0.5 * dm;
                                p1.y -= ny * 0.5 * dm;
                                p2.x += nx * 0.5 * dm;
                                p2.y += ny * 0.5 * dm;
                            }
                        }
                    }
                }
            }
        }

        // borders
        for x in 0..world.cells.len() {
            for y in 0..world.cells[x].len() {
                for p in &mut world.cells[x][y] {
                    if p.x < 0.0 { p.x = 0.0 }
                    if p.x > world.width as f32 - 1.0 { p.x = world.width as f32 - 1.0 }
                    if p.y < 0.0 { p.y = 0.0 }
                    if p.y > world.height as f32 - 1.0 { p.y = world.height as f32 - 1.0 }
                }
            }
        }

        // update pos + cell
        let mut removes = vec![];
        for x in 0..world.cells.len() {
            for y in 0..world.cells[x].len() {
                world.cells[x][y].retain_mut(|p| {
                    let dx = p.x - p.px;
                    let dy = p.y - p.py;
                    p.px = p.x;
                    p.py = p.y;
                    p.x += dx + p.ax * delta_sq;
                    p.y += dy + p.ay * delta_sq;
                    p.ax = 0.0;
                    p.ay = 0.0;
                    if p.x as usize != x || p.y as usize != y {
                        removes.push(p.clone());
                        false
                    } else { true }
                });
            }
        }
        for r in removes {
            world.add_particle(r)
        }
    }
}