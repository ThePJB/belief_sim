use minvect::*;
use std::f32::INFINITY;
use std::hash::Hash;
use std::hash::Hasher;
use std::collections::HashSet;

fn orient(a: Vec2, b: Vec2, p: Vec2) -> bool {
    let ab = b - a;
    let ap = p - a;
    ab.x * ap.y - ab.y * ap.x < 0.0
}
// nb its for a certain winding order, flip sign in orient for the other one. or see if all same
fn point_in_polygon(verts: &[Vec2], p: Vec2) -> bool {
    for i in 1..verts.len() {
        let a = verts[i-1];
        let b = verts[i];
        if !orient(a, b, p) {
            return false
        }
    }
    let a = verts[verts.len() - 1];
    let b = verts[0];
    if !orient(a, b, p) {
        return false
    }
    return true;
}
fn segment_intersection(a1: Vec2, b1: Vec2, a2: Vec2, b2: Vec2) -> Option<Vec2> {

    let s1 = b1 - a1;
    let s2 = b2 - a2;

    let s1_x = s1.x;
    let s1_y = s1.y;
    let s2_x = s2.x;
    let s2_y = s2.y;

    let det = s1_x * s2_y - s1_y * s2_x;

    if det.abs() < 1e-9f32 { // Adjust EPSILON as needed
        return None; // The segments are parallel or colinear, no intersection
    }

    let s = (-s1_y * (a1.x - a2.x) + s1_x * (a1.y - a2.y)) / (-s2_x * s1_y + s1_x * s2_y);
    let t = (s2_x * (a1.y - a2.y) - s2_y * (a1.x - a2.x)) / (-s2_x * s1_y + s1_x * s2_y);

    if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
        // Collision detected
        let intersection = Vec2::new(a1.x + t * s1_x, a1.y + t * s1_y);
        Some(intersection)
    } else {
        None // Segments do not intersect within their bounds
    }
}

fn centroid(v: &[Vec2]) -> Vec2 {
    let mut acc = v[0];
            for i in 1..v.len() {
                if v[i].x < 0.0 || v[i].x > 1.0 || v[i].y < 0.0 || v[i].y > 1.0 {
                    continue;
                }
                acc += v[i];
            }
            acc / v.len() as f32
}

pub fn winding_order_sort(v: &mut Vec<Vec2>) {
    let centroid = centroid(v);
    v.sort_by_key(|v| {
        let v = *v - centroid;
        ordered_float::OrderedFloat(v.x.atan2(v.y))
    });
}

fn clip_poly(verts: &[Vec2], min: Vec2, max: Vec2) -> Vec<Vec2> {
    // all poly verts within aabb + all aabb verts within poly + all intersection points of poly and aabb, winding order sort
    let mut clipped_verts = vec![];

    for v in verts {
        if v.x > min.x && v.x < max.x && v.y > min.y && v.y < max.y {
            clipped_verts.push(*v);
        }
    }

    let a = min;
    let b = vec2(max.x, min.y);
    let c = max;
    let d = vec2(min.x, max.y);

    for aabb_vert in [a, b, c, d] {
        if point_in_polygon(verts, aabb_vert) {
            clipped_verts.push(aabb_vert);
        }
    }

    let aabb_edges = [(a,b), (b,c), (c,d), (d,a)];
    for (a, b) in aabb_edges {
        for i in 1..verts.len() {
            let ap = verts[i-1];
            let bp = verts[i];
            if let Some(p) = segment_intersection(a, b, ap, bp) {
                clipped_verts.push(p);
            }
        }
        let ap = verts[verts.len() - 1];
        let bp = verts[0];
        if let Some(p) = segment_intersection(a, b, ap, bp) {
            clipped_verts.push(p);
        }
    }
    winding_order_sort(&mut clipped_verts);
    clipped_verts
}

#[test]
fn test_pip() {
    let p = vec![vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(0.5, -1.0)];
    assert!(point_in_polygon(&p, vec2(0.1, -0.1)));
    assert!(!point_in_polygon(&p, vec2(1.1, -0.1)));
}

#[test]
fn test_pip2() {
    // let p = vec![vec2(0.5, 0.5), vec2(1.5, 0.5), vec2(1.5, 1.5), vec2(0.5, 1.5)];
    let p = vec![vec2(0.5, 1.5), vec2(1.5, 1.5), vec2(1.5, 0.5), vec2(0.5, 0.5)];
    assert!(point_in_polygon(&p, vec2(1.0, 1.0)));
}

#[test]
fn test_clip_poly() {
    // let v = vec![vec2(0.5, 0.5), vec2(1.5, 0.5), vec2(1.5, 1.5), vec2(0.5, 1.5)];
    let v = vec![vec2(0.5, 1.5), vec2(1.5, 1.5), vec2(1.5, 0.5), vec2(0.5, 0.5)];
    let clipped = clip_poly(&v, vec2(0.0, 0.0), vec2(1.0, 1.0));
    assert_eq!(clipped, vec![vec2(0.5, 0.5), vec2(0.5, 1.0), vec2(1.0, 1.0), vec2(1.0, 0.5)]);
}

#[test]
fn test_segment_segment() {
    assert_eq!(segment_intersection(vec2(1.0, 0.0), vec2(1.0, 1.0), vec2(1.5, 0.5), vec2(0.5, 0.5)), Some(vec2(1.0, 0.5)));

}

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

    pub fn site_neighbours_slow(&self, i: usize) -> Vec<usize> {
        let mut neighbours: HashSet<usize> = self.site_triangles_slow(i).iter().map(|t| self.tri_verts[*t]).flatten().collect();
        neighbours.remove(&i);
        neighbours.into_iter().collect()
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
            })
            // .map(|v| {
            //     vec2(v.x.max(0.0).min(1.0), v.y.max(0.0).min(1.0))
            // })
            // i see how this is retarded
            // need to actually calculate intersections of the line segments ay bruh
            .collect();

        let center = self.sites[i];

        site_verts.sort_by_key(|v| {
            let v = *v - center;
            ordered_float::OrderedFloat(v.x.atan2(v.y))
        });

        clip_poly(&site_verts, vec2(0.0, 0.0), vec2(1.0, 1.0))
    }

    pub fn centroids(&self) -> Vec<Vec2> {
        (0..self.sites.len()).map(|i| self.site_voronoi_verts(i)).map(|svv| {
            centroid(&svv)
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