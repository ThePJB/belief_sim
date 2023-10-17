use glow_mesh::xyzrgba::*;
use glow_mesh::xyzrgba_build2d::put_triangle;
use voronoice::{Voronoi, VoronoiBuilder, Point, BoundingBox};
use crate::rng::*;
use crate::heightmap::*;
use minvect::*;

pub struct Culture {
    aggression: f32,
    orthodoxy: f32,
    consolidate_tendency: f32,
    nomad_tendency: f32,
    
    // determines colour
    h: f32,
    s: f32,
    v: f32,
}

impl Culture {
    pub fn new(rng: &mut Rng) -> Self {
        Culture {
            aggression: rng.next_float(),
            orthodoxy: rng.next_float(),
            consolidate_tendency: rng.next_float(),
            nomad_tendency: rng.next_float(),
            h: rng.next_float(),
            s: rng.next_float(),
            v: rng.next_float(),
        }
    }
}

pub struct Sim {
    pub diagram: Voronoi,
    pub elevation: Vec<f32>,
    pub temperature: Vec<f32>,
    pub population: Vec<f32>,
    pub culture: Vec<Culture>,
    pub sites: Vec<Vec2>,

}


impl Sim {
    pub fn new(seed: u32, a: f32) -> Self {
        let mut rng = Rng::new_seeded(seed);
        let noisemap_seed = rng.next_u32();

        let mut sites = vec![];
        for i in 0..800 {
            let x = rng.next_float() * a;
            let y = rng.next_float();
            sites.push(vec2(x, y));
        }
        
        let bb = BoundingBox::new(Point { x: a as f64 / 2.0, y: 0.5}, a as f64, 1.0);
        let diagram = VoronoiBuilder::default()
                        .set_sites(sites.iter().map(|v| Point { x: v.x as f64, y: v.y as f64 }).collect())
                        .set_bounding_box(bb)
                        .set_lloyd_relaxation_iterations(1)
                        .build()
                        .expect("failed to make voronoi diagram for some reason");

        let elevation: Vec<f32> = sites.iter().map(|s| heightmap(*s, noisemap_seed)).collect();
        let temperature = sites.iter().map(|s| temperature_map(*s, noisemap_seed)).collect();
        let mut population = vec![2.0; sites.len()];
        for i in 0..sites.len() {
            if elevation[i] < 0.5 {
                population[i] = 0.0; // water
            }
        }
        let mut culture = vec![];
        for i in 0..sites.len() {
            culture.push(Culture::new(&mut rng));
        }

        Sim {
            diagram,
            sites,
            elevation,
            temperature,
            population,
            culture,
        }
    }

    // 1 year
    pub fn step(&mut self) {

    }

    // movement of people
    pub fn movement(&mut self, from: usize, to: usize) {

    }

    pub fn render(&self) -> Vec<XYZRGBA> {
        let col_deep_water = vec4(0.0, 0.3, 0.6, 1.0);
        let col_shallow_water = vec4(0.0, 0.6, 0.8, 1.0);
        let col_plains = vec4(0.4, 0.8, 0.4, 1.0);
        let col_desert = vec4(0.8, 0.8, 0.0, 1.0);
        let col_mountain = vec4(0.5, 0.5, 0.5, 1.0);
        let col_snow = vec4(1.0, 1.0, 1.0, 1.0);

        let mut buf = vec![];
        for i in 0..self.sites.len() {
            let h = self.elevation[i];
            let temp = self.temperature[i];
            let bg_colour = if h < 0.5 {
                let t = h * 2.0;
                col_deep_water.lerp(col_shallow_water, t)
            } else {
                let th = (h - 0.5) * 2.0;
                let c_temp = if temp < 0.5 {
                    let tt = temp * 2.0;
                    col_snow.lerp(col_plains, tt)
                } else {
                    let tt = (temp - 0.5) * 2.0;
                    col_plains.lerp(col_desert, tt)
                };

                let c_h = col_plains.lerp(col_mountain, th);
                c_h.lerp(c_temp, 0.5)
            };

            // colour due to temp height etc
            // then a dot, size due to culture colour, shape, etc
            // circle square star: more population

            // then get the triangles from the voronoi diagram and draw them
            let cell = self.diagram.cell(i);
            let c = self.sites[i];
            let v: Vec<Vec2> = cell.iter_vertices().map(|p| vec2(p.x as f32, p.y as f32)).collect();
            for i in 1..v.len() {
                let a = v[i - 1];
                let b = v[i];
                put_triangle(&mut buf, a, b, c, bg_colour, 0.5);
            }
            let a = v[v.len() - 1];
            let b = v[0];
            put_triangle(&mut buf, a, b, c, bg_colour, 0.5);
        }
        buf
    }
}

// its almost like the sim should own that shit
// but well this shouldnt be about rendering it could be argued
// or theres like sim render 
// but yea theres probably a running sim context i guess that has the opengl handles etc
// which by the way needs a way to clean up