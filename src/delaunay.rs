use minvect::*;
use std::f32::INFINITY;
use std::hash::Hash;
use std::hash::Hasher;
use std::collections::HashSet;

fn det4(a: [[f32;4];4]) -> f32 {
    let s1=a[0][0]*(a[1][1]*(a[2][2]*a[3][3]-a[3][2]*a[2][3])-a[1][2]*(a[2][1]*a[3][3]-a[2][3]*a[3][1])+a[1][3]*(a[2][1]*a[3][2]-a[2][2]*a[3][1]));
    let s2=a[0][1]*(a[1][0]*(a[2][2]*a[3][3]-a[3][2]*a[2][3])-a[1][2]*(a[2][0]*a[3][3]-a[2][3]*a[3][0])+a[1][3]*(a[2][0]*a[3][2]-a[2][2]*a[3][0]));
    let s3=a[0][2]*(a[1][0]*(a[2][1]*a[3][3]-a[3][1]*a[2][3])-a[1][1]*(a[2][0]*a[3][3]-a[2][3]*a[3][0])+a[1][3]*(a[2][0]*a[3][1]-a[2][1]*a[3][0]));
    let s4=a[0][3]*(a[1][0]*(a[2][1]*a[3][2]-a[3][1]*a[2][2])-a[1][1]*(a[2][0]*a[3][2]-a[2][2]*a[3][0])+a[1][2]*(a[2][0]*a[3][1]-a[2][1]*a[3][0]));
    s1-s2+s3-s4
}

fn point_in_circumcircle(p: Vec2, a: Vec2, b: Vec2, c: Vec2) -> bool {
    det4([
        [a.x, a.y, a.x*a.x + a.y*a.y, 1.0],
        [b.x, b.y, b.x*b.x + b.y*b.y, 1.0],
        [c.x, c.y, c.x*c.x + c.y*c.y, 1.0],
        [p.x, p.y, p.x*p.x + p.y*p.y, 1.0],
    ]) > 0.0
}

fn circumcenter(a: Vec2, b: Vec2, c: Vec2) -> Vec2 {
    let d = 2.0 * (a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y));
    let ux = ((a.x * a.x + a.y * a.y) * (b.y - c.y) + (b.x * b.x + b.y * b.y) * (c.y - a.y) + (c.x * c.x + c.y * c.y) * (a.y - b.y)) / d;
    let uy = ((a.x * a.x + a.y * a.y) * (c.x - b.x) + (b.x * b.x + b.y * b.y) * (a.x - c.x) + (c.x * c.x + c.y * c.y) * (b.x - a.x)) / d;
    Vec2 { x: ux, y: uy }
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    a: usize,
    b: usize,
}
impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        (self.a == other.a && self.b == other.b) || (self.a == other.b && other.a == self.b)
    }
}
impl Eq for Edge {}
impl Hash for Edge {
    fn hash<H>(&self, h: &mut H) where H: Hasher {
        let p = 2654435769;
        ((p * self.a) ^ (p * self.b)).hash(h);
    }
}
impl Edge {
    fn new(a: usize, b: usize) -> Edge {Edge{a,b}}
}

pub struct Delaunay {
    // Idx: sites / delaunay verts
    pub sites: Vec<Vec2>,

    // Idx: delaunay triangles
    pub tri_edges: Vec<[Edge; 3]>,  // This edge idx delaunay verts
    pub tri_verts: Vec<[usize; 3]>,
}

impl Delaunay {
    pub fn new() -> Self {
        let mut vdg = Delaunay { sites: vec![], tri_edges: vec![], tri_verts: vec![] };
                
        vdg.sites.push(Vec2::new(0.0, 0.0));
        vdg.sites.push(Vec2::new(1.0, 0.0));
        vdg.sites.push(Vec2::new(1.0, 1.0));
        vdg.sites.push(Vec2::new(0.0, 1.0));
        vdg.tri_edges.push([Edge::new(0, 1), Edge::new(1, 3), Edge::new(3, 0)]);
        vdg.tri_verts.push([0, 1, 3]);
        
        vdg.tri_edges.push([Edge::new(1, 2), Edge::new(2, 3), Edge::new(1, 3)]);
        vdg.tri_verts.push([1, 2, 3]);

        vdg
    }
    pub fn add_site(&mut self, p: Vec2) {
        let mut bad_triangles = HashSet::new();
        let mut bad_edges = HashSet::new();
        let mut poly = HashSet::new();

        self.sites.push(p);
        for (t_idx, tp) in self.tri_verts.iter().enumerate() {
            if point_in_circumcircle(p, self.sites[tp[0]], self.sites[tp[1]], self.sites[tp[2]]) {
                bad_triangles.insert(t_idx);
            }
        }
        for bt in &bad_triangles {
            for e in &self.tri_edges[*bt] {
                if poly.contains(e) {
                    poly.remove(e);
                    bad_edges.insert(*e);
                } else {
                    poly.insert(*e);
                }
            }
        }

        // delete bad triangles
        let mut btv: Vec<usize> = bad_triangles.iter().map(|x| *x).collect();
        btv.sort();
        for bti in btv.iter().rev() {
            self.tri_edges.swap_remove(*bti);
            self.tri_verts.swap_remove(*bti);
        }

        for poly_edge in &poly {
            self.tri_edges.push([Edge::new(poly_edge.a, poly_edge.b), Edge::new(poly_edge.b, self.sites.len() - 1), Edge::new(self.sites.len() - 1, poly_edge.a)]);
            self.tri_verts.push([poly_edge.a, poly_edge.b, self.sites.len() - 1]);
        }
    }

    pub fn site_triangles_slow(&self, i: usize) -> Vec<usize> {
        (0..(self.tri_verts.len())).filter(|j| self.tri_verts[*j][0] == i || self.tri_verts[*j][1] == i || self.tri_verts[*j][2] == i).collect()
    }

    pub fn site_voronoi_verts(&self, i: usize) -> Vec<Vec2> {
        let site_triangles = self.site_triangles_slow(i);

        let mut site_verts: Vec<Vec2> = site_triangles.iter()
            .map(|i| {
                let t = self.tri_verts[*i];
                let a = self.sites[t[0]];
                let b = self.sites[t[1]];
                let c = self.sites[t[2]];
                circumcenter(a, b, c)
        }).collect();

        let center = self.sites[i];

        site_verts.sort_by_key(|v| {
            let v = *v - center;
            ordered_float::OrderedFloat(v.x.atan2(v.y))
        });
        site_verts
    }

    pub fn centroids(&self) -> Vec<Vec2> {
        (0..self.sites.len()).map(|i| self.site_voronoi_verts(i)).map(|svv| {
            let mut acc = svv[0];
            for i in 1..svv.len() {
                acc += svv[i];
            }
            acc / svv.len() as f32
        }).collect()
    }

    pub fn nearest_site_idx(&self, p: Vec2) -> usize {
        let mut min = INFINITY;
        let mut min_idx = 0;
        for i in 0..self.sites.len() {
            let u = p - self.sites[i];
            let d2 = u.dot(u);
            if d2 < min {
                min = d2;
                min_idx = i;
            }
        }
        min_idx
    }
}

#[test]
fn test_delaunay() {
    let mut d = Delaunay::new();
    d.add_site(vec2(0.1, 0.1));
    d.add_site(vec2(0.6, 0.1));
    d.add_site(vec2(0.6, 0.7));
    dbg!(&d.sites);
    dbg!(&d.tri_verts);
}