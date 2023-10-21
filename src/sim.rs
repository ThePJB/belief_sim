use glow_mesh::xyzrgba::*;
use glow_mesh::xyzrgba_build2d::*;
use crate::delaunay::Delaunay;
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
    pub diagram: Delaunay,
    pub centroids: Vec<Vec2>,
    pub population: Vec<f32>,
    pub culture: Vec<Culture>,
    pub seed: u32,
    pub h_seed: u32,
    pub selected_cell: Option<usize>,
}


impl Sim {
    pub fn new(seed: u32, a: f32) -> Self {
        let mut rng = Rng::new_seeded(seed);
        let h_seed = rng.next_u32();

        let mut diagram = Delaunay::new();
        for i in 0..200 {
            diagram.add_site(vec2(rng.next_float(), rng.next_float()));
        }
        let mut population = vec![2.0; diagram.sites.len()];
        // but oi nah do a lloyd step ay
        let centroids = diagram.centroids();

        for i in 0..diagram.sites.len() {
            if heightmap(&centroids[i], h_seed) < 0.6 {
                population[i] = 0.0;
            }
        }
        let mut culture = vec![];
        for i in 0..diagram.sites.len() {
            culture.push(Culture::new(&mut rng));
        }


        Sim {
            diagram,
            population,
            culture,
            seed,
            h_seed,
            centroids,
            selected_cell: None,
        }
    }

    pub fn select_cell(&mut self, idx: usize) {
        self.selected_cell = Some(idx);
        println!("Selected cell {} site {} pop {} centroid {} height {} verts {:?}", idx, self.diagram.sites[idx], self.population[idx], self.centroids[idx], heightmap(&self.centroids[idx], self.h_seed), self.diagram.site_voronoi_verts(idx))
    }

    // 1 year
    pub fn step(&mut self) {

    }

    // movement of people
    pub fn movement(&mut self, from: usize, to: usize) {

    }

    pub fn civ_geometry(&self) -> Vec<XYZRGBA> {
        let mut buf = vec![];
        for i in 0..self.diagram.sites.len() {
            let civ = &self.culture[i];
            let col = vec4(civ.h * 360.0, 0.8, 1.0, 1.0).hsv_to_rgb();
            let pop = self.population[i];
            if pop == 0.0 { continue; }
            put_poly(&mut buf, self.centroids[i], pop.ln() / 200.0, 10, 0.0, col, 0.2);
        }

        if let Some(idx) = self.selected_cell {
            let mut verts = self.diagram.site_voronoi_verts(idx);
            verts.push(verts[0]);
            for i in 1..verts.len() {
                let a = verts[i-1];
                let b = verts[i];
                put_line(&mut buf, a, b, 0.001, vec4(1.0, 1.0, 1.0, 1.0), 0.4);
            }
        }

        // // draw triangles
        // for i in 0..self.diagram.tri_verts.len() {
        //     let t = self.diagram.tri_verts[i];
        //     put_line(&mut buf, self.diagram.sites[t[0]], self.diagram.sites[t[1]], 0.002, vec4(1.0, 0.0, 0.0, 1.0), 0.3);
        //     put_line(&mut buf, self.diagram.sites[t[1]], self.diagram.sites[t[2]], 0.002, vec4(1.0, 0.0, 0.0, 1.0), 0.3);
        //     put_line(&mut buf, self.diagram.sites[t[2]], self.diagram.sites[t[0]], 0.002, vec4(1.0, 0.0, 0.0, 1.0), 0.3);
        // }

        buf
    }

    pub fn terrain_geometry(&self) -> Vec<XYZRGBA> {
        let mut buf = vec![];
        for i in 0..self.diagram.sites.len() {
            let bg_colour = col(&self.centroids[i], self.h_seed);
            let c = self.diagram.sites[i];
            let v = self.diagram.site_voronoi_verts(i);
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