use glow_mesh::xyzrgba::*;
use glow_mesh::xyzrgba_build2d::put_poly;
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
    pub seed: u32,
}


impl Sim {
    pub fn new(seed: u32, a: f32) -> Self {
        let mut rng = Rng::new_seeded(seed);
        let noisemap_seed = rng.next_u32();

        let mut sites: Vec<Vec2> = vec![];
        for i in 0..400 {
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

        let elevation: Vec<f32> = sites.iter().map(|s| heightmap(s, noisemap_seed)).collect();
        let temperature = sites.iter().map(|s| temperature_map(s, noisemap_seed)).collect();
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
            seed,
        }
    }

    // 1 year
    pub fn step(&mut self) {

    }

    // movement of people
    pub fn movement(&mut self, from: usize, to: usize) {

    }

    pub fn civ_geometry(&self) -> Vec<XYZRGBA> {
        let mut buf = vec![];
        for i in 0..self.sites.len() {
            let civ = &self.culture[i];
            let col = vec4(civ.h * 360.0, 0.8, 1.0, 1.0).hsv_to_rgb();
            let pop = self.population[i];
            if pop == 0.0 { continue; }
            put_poly(&mut buf, self.sites[i], pop.ln() / 200.0, 10, 0.0, col, 0.4);
        }
        buf
    }

    pub fn terrain_geometry(&self) -> Vec<XYZRGBA> {
        let mut buf = vec![];
        for i in 0..self.sites.len() {
            let bg_colour = col(&self.sites[i], self.seed);

            // then get the triangles from the voronoi diagram and draw them
            let cell = self.diagram.cell(i);
            let p = cell.site_position();
            let p = vec2(p.x as f32, p.y as f32);
            let c = self.sites[i];
            // let c = p;
            let v: Vec<Vec2> = cell.iter_vertices().map(|p| vec2(p.x as f32, p.y as f32)).collect();
            for i in 1..v.len() {
                let a = v[i - 1];
                let b = v[i];
                put_triangle(&mut buf, a, b, c, bg_colour, 0.5);
            }
            let a = v[v.len() - 1];
            let b = v[0];
            put_triangle(&mut buf, a, b, c, bg_colour, 0.5);

            put_poly(&mut buf, c, 0.01, 10, 0.0, vec4(0.0, 0.0, 0.0, 1.0), 0.4);

        }
        buf
    }
}

// its almost like the sim should own that shit
// but well this shouldnt be about rendering it could be argued
// or theres like sim render 
// but yea theres probably a running sim context i guess that has the opengl handles etc
// which by the way needs a way to clean up