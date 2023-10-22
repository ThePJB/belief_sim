use std::f32::INFINITY;
use std::f32::NEG_INFINITY;

use glow_mesh::xyzrgba::*;
use glow_mesh::xyzrgba_build2d::*;
use crate::delaunay::Delaunay;
use crate::rng::*;
use crate::heightmap::*;
use minvect::*;

pub const REQUIRED_ADVANTAGE_TO_INVADE: usize = 0;
pub const AGGRESSION: usize = 1;
pub const TOLERATED_DIFFERENCE: usize = 2;
pub const INITIATIVE: usize = 3;
pub const FERTILITY: usize = 4;
pub const WELCOMING: usize = 5;
pub const PROGRESSIVE: usize = 6;
pub const HUE: usize = 7;
pub const NUM_MEMES: usize = 8;

pub const NAMES: [&str; NUM_MEMES] = [
    "INVADE_ADVANTAGE    ",
    "AGGRESSION          ",
    "TOLERATED_DIFFERENCE",
    "INITIATIVE          ",
    "FERTILITY           ",
    "WELCOMING           ",
    "PROGRESSIVE         ",
    "HUE                 ",
];

pub struct Culture {
    memes: [f32; NUM_MEMES],
}

impl Culture {
    pub fn new(rng: &mut Rng) -> Self {
        let mut memes = [0.0; NUM_MEMES];
        for i in 0..NUM_MEMES {
            memes[i] = rng.next_float();
        }
        Culture { memes }
    }
    pub fn dot(&self, other: &Culture) -> f32 {
        let mut acc = 0.0;
        for i in 0..NUM_MEMES {
            acc += self.memes[i]*other.memes[i];
        }
        acc
    }
    pub fn lerp(&self, other: &Culture, t: f32) -> Culture {
        let mut c = Culture { memes: [0.0; NUM_MEMES]};
        for i in 0..NUM_MEMES {
            c.memes[i] = self.memes[i] * (1.0 - t) + other.memes[i] * t;
        }
        c
    }
    pub fn drift(&mut self, rng: &mut Rng) {
        for i in 0..NUM_MEMES {
            let d = (rng.next_float() - 0.5) * 0.05;
            self.memes[i] = (self.memes[i] + d * self.memes[PROGRESSIVE]).max(0.0).min(1.0);
        }
    }
    // crossover: lerps or randoms?
    // also random noise it
}

pub struct Sim {
    pub diagram: Delaunay,
    pub centroids: Vec<Vec2>,
    pub population: Vec<f32>,
    pub culture: Vec<Culture>,
    pub seed: u32,
    pub h_seed: u32,
    pub selected_cell: Option<usize>,
    pub year: isize,
    pub rng: Rng,
    pub is_water: Vec<bool>,
}



impl Sim {
    pub fn new(seed: u32, a: f32) -> Self {
        let mut rng = Rng::new_seeded(seed);
        let h_seed = rng.next_u32();

        let mut diagram = Delaunay::new();
        for i in 0..200 {
            diagram.add_site(vec2(rng.next_float(), rng.next_float()));
        }
        let centroids = diagram.centroids();
        let mut diagram = Delaunay::new();
        for p in centroids.iter().skip(4) {
            diagram.add_site(*p);
        }
        let mut population = vec![2.0; diagram.sites.len()];
        let centroids = diagram.centroids();

        let mut is_water = vec![false; diagram.sites.len()];
        for i in 0..diagram.sites.len() {
            if heightmap(&centroids[i], h_seed) < 0.6 {
                population[i] = 0.0;
                is_water[i] = true;
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
            year: 0,
            rng,
            is_water,
        }
    }

    pub fn select_cell(&mut self, idx: usize) {
        self.selected_cell = Some(idx);
        println!("Selected cell {} site {} pop {} centroid {} height {}", idx, self.diagram.sites[idx], self.population[idx], self.centroids[idx], heightmap(&self.centroids[idx], self.h_seed))
    }

    // 1 year
    pub fn step(&mut self) {
        self.year += 1;
        // sort by initiative

        for i in 0..self.diagram.sites.len() {
            if self.is_water[i] {continue;}
            let pop_exp = 1.0 + self.culture[i].memes[FERTILITY] / 50.0;
            // self.population[i] *= pop_exp;
            self.population[i] = ((self.population[i] + 2.0) * pop_exp) - 2.0;
            if self.population[i] > 10.0 && self.rng.next_float() < self.culture[i].memes[AGGRESSION] {
                let mut neighbours = self.diagram.site_neighbours_slow(i);
                neighbours.retain(|j| !self.is_water[*j] && 
                    self.population[i] > 4.0*self.population[*j]*self.culture[i].memes[REQUIRED_ADVANTAGE_TO_INVADE] &&
                    self.culture[*j].dot(&self.culture[i]) > self.culture[i].memes[TOLERATED_DIFFERENCE]);
                // self.population[i] > 2.0*self.population[*j] && 
                if neighbours.len() == 0 { continue; }
                let j = (self.rng.next_u32() % neighbours.len() as u32) as usize;
                self.invade(i, neighbours[j]);
            }
            self.culture[i].drift(&mut self.rng);
        }
        println!("report year {}:", self.year);
        self.report();
    }

    // movement of people
    pub fn invade(&mut self, from: usize, to: usize) {
        let move_amount = self.population[from] * 0.5;
        let defend_amount = self.population[to];

        let (atk_casualties, def_casualties, attacker_win) = battle(move_amount, defend_amount, &mut self.rng);
        if attacker_win {
            self.population[from] -= move_amount;
            // maybe remaining defenders army flees to other / emigrates and then they handle emigrate
            let from_pop = move_amount - atk_casualties;
            self.population[to] += from_pop;
            let to_pop = self.population[to];
            let lerp_t = from_pop / (from_pop + to_pop);
            let def_culture = self.culture[to].lerp(&self.culture[from], lerp_t);
            // defending culture = assimilation with attackers
            self.population[to] += atk_casualties;
            self.culture[to] = def_culture;
        } else {
            self.population[from] -= atk_casualties;
            self.population[to] -= def_casualties;
        }
    }

    pub fn civ_geometry(&self) -> Vec<XYZRGBA> {
        let mut buf = vec![];
        for i in 0..self.diagram.sites.len() {
            let civ = &self.culture[i];
            let col = vec4(civ.memes[HUE] * 360.0, 0.8, 1.0, 1.0).hsv_to_rgb();
            let pop = self.population[i];
            if pop == 0.0 { continue; }
            put_poly(&mut buf, self.centroids[i], pop.ln().ln() / 100.0, 10, 0.0, col, 0.2);
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

    pub fn report(&self) {
        let mut mean = [0.0; NUM_MEMES];
        let mut min = [INFINITY; NUM_MEMES];
        let mut max = [NEG_INFINITY; NUM_MEMES];

        for i in 0..self.culture.len() {
            for j in 0..NUM_MEMES {
                mean[j] += self.culture[i].memes[j];
                min[j] = self.culture[i].memes[j].min(min[j]);
                max[j] = self.culture[i].memes[j].max(max[j]);
            }
        }
        for j in 0..NUM_MEMES {
            mean[j] /= self.culture.len() as f32;
            println!("{}: min: {} mean: {} max: {}", NAMES[j], min[j], mean[j], max[j]);
        }
    }
}
fn battle(num_attackers: f32, num_defenders: f32, rng: &mut Rng) -> (f32, f32, bool) {
    let effectiveness_attackers = num_attackers * rng.next_float();
    let effectiveness_defenders = num_defenders * rng.next_float();

    let ea_ratio = effectiveness_attackers / (effectiveness_attackers + effectiveness_defenders);
    let ed_ratio = effectiveness_defenders / (effectiveness_attackers + effectiveness_defenders);

    let casualties_attackers = num_attackers * ed_ratio * rng.next_float();
    let casualties_defenders = num_defenders * ea_ratio * rng.next_float();

    (
        casualties_attackers,
        casualties_defenders,
        effectiveness_attackers > effectiveness_defenders,
    )
}

#[test]
fn test_battle() {
    let mut rng = Rng::new_seeded(1234);
    dbg!(battle(100.0, 100.0, &mut rng));
    dbg!(battle(100.0, 100.0, &mut rng));

    dbg!(battle(200.0, 100.0, &mut rng));
    dbg!(battle(200.0, 100.0, &mut rng));

    dbg!(battle(300.0, 100.0, &mut rng));
    dbg!(battle(300.0, 100.0, &mut rng));

    dbg!(battle(1.0, 2.0, &mut rng));
    dbg!(battle(1.0, 2.0, &mut rng));
    dbg!(battle(1.0, 2.0, &mut rng));
    dbg!(battle(1.0, 2.0, &mut rng));
    dbg!(battle(1.0, 2.0, &mut rng));
    dbg!(battle(1.0, 2.0, &mut rng));
}