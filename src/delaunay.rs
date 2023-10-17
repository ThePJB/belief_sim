// use minvect::*;
// use crate::rng::*;
// use std::hash::Hash;
// use std::hash::Hasher;
// use std::collections::HashSet;


// fn det4(a: [[f32;4];4]) -> f32 {
//     let s1=a[0][0]*(a[1][1]*(a[2][2]*a[3][3]-a[3][2]*a[2][3])-a[1][2]*(a[2][1]*a[3][3]-a[2][3]*a[3][1])+a[1][3]*(a[2][1]*a[3][2]-a[2][2]*a[3][1]));
//     let s2=a[0][1]*(a[1][0]*(a[2][2]*a[3][3]-a[3][2]*a[2][3])-a[1][2]*(a[2][0]*a[3][3]-a[2][3]*a[3][0])+a[1][3]*(a[2][0]*a[3][2]-a[2][2]*a[3][0]));
//     let s3=a[0][2]*(a[1][0]*(a[2][1]*a[3][3]-a[3][1]*a[2][3])-a[1][1]*(a[2][0]*a[3][3]-a[2][3]*a[3][0])+a[1][3]*(a[2][0]*a[3][1]-a[2][1]*a[3][0]));
//     let s4=a[0][3]*(a[1][0]*(a[2][1]*a[3][2]-a[3][1]*a[2][2])-a[1][1]*(a[2][0]*a[3][2]-a[2][2]*a[3][0])+a[1][2]*(a[2][0]*a[3][1]-a[2][1]*a[3][0]));
//     s1-s2+s3-s4
// }

// fn point_in_circumcircle(p: Vec2, a: Vec2, b: Vec2, c: Vec2) -> bool {
//     det4([
//         [a.x, a.y, a.x*a.x + a.y*a.y, 1.0],
//         [b.x, b.y, b.x*b.x + b.y*b.y, 1.0],
//         [c.x, c.y, c.x*c.x + c.y*c.y, 1.0],
//         [p.x, p.y, p.x*p.x + p.y*p.y, 1.0],
//     ]) > 0.0
// }

// fn circumcenter(a: Vec2, b: Vec2, c: Vec2) -> Vec2 {
//     let d = 2.0 * (a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y));
//     let ux = ((a.x * a.x + a.y * a.y) * (b.y - c.y) + (b.x * b.x + b.y * b.y) * (c.y - a.y) + (c.x * c.x + c.y * c.y) * (a.y - b.y)) / d;
//     let uy = ((a.x * a.x + a.y * a.y) * (c.x - b.x) + (b.x * b.x + b.y * b.y) * (a.x - c.x) + (c.x * c.x + c.y * c.y) * (b.x - a.x)) / d;
//     Vec2 { x: ux, y: uy }
// }

// #[derive(Debug, Clone, Copy)]
// struct VoronoiEdge {
//     start: Vec2,  // Starting point of the Voronoi edge
//     end: Vec2,    // Ending point of the Voronoi edge
// }
// #[derive(Debug, Clone, Copy)]
// struct Edge {
//     a: usize,
//     b: usize,
// }
// impl PartialEq for Edge {
//     fn eq(&self, other: &Edge) -> bool {
//         (self.a == other.a && self.b == other.b) || (self.a == other.b && other.a == self.b)
//     }
// }
// impl Eq for Edge {}
// impl Hash for Edge {
//     fn hash<H>(&self, h: &mut H) where H: Hasher {
//         let p = 2654435769;
//         ((p * self.a) ^ (p * self.b)).hash(h);
//     }
// }
// impl Edge {
//     fn new(a: usize, b: usize) -> Edge {Edge{a,b}}
// }
// /*
// Every site has a voronoi polygon
// */

// pub struct Delaunay {
//     sites: Vec<Vec2>,
//     site_voronoi_edges: Vec<Vec<Edge>>,
//     site_tris: Vec<Vec<usize>>,
    
//     tri_edges: Vec<[Edge; 3]>,
//     tri_verts: Vec<[usize; 3]>,

//     voronoi_verts: Vec<Vec2>,


    
//     // would like a triangulation
//     // centerpoint can be site
//     // vertexes are circumcenter of each triangle
//     // or the points of the voronoi polygon

//     // maybe this stuff can just be local ie we add 1 then resolve this shit
//     bad_triangles: HashSet<usize>,
//     bad_edges: HashSet<Edge>,
//     poly: HashSet<Edge>,
// }

// impl Delaunay {
//     fn compute_site_edges_and_verts(&mut self, i: usize) {
//         // 1. gather all triangles of this site
//         let triangles = self.triangle_inds_of_site(i);
//         // 2. gather all circumcenters of triangles
//         let circumcenters: Vec<Vec2> = triangles.iter().map(|i| {
//             let t = self.tri_verts[*i];
//             let a = self.sites[t[0]];
//             let b = self.sites[t[1]];
//             let c = self.sites[t[2]];
//             circumcenter(a, b, c)
//         }).collect();
//         // connecting any two circumcentres whose triangles share an edge.
//         // how do u know if fucking neighbouring triangles.
//         // u have to sort the triangles by ccw ...
//         // yea get winding order of triangles bruv
//         // maybe my mesh making was just short 1 triangle for each polygon tho
//         // 3. circumcenters get pushed
//         // 4. voronoi edges get pushed
//         well these can be sorted indexes maybe...
//     }
//     pub fn add_site(&mut self, p: Vec2) {
//         self.sites.push(p);
//         self.bad_triangles = HashSet::new();
//         for (t_idx, tp) in self.tri_verts_idx.iter().enumerate() {
//             if point_in_circumcircle(p, self.sites[tp[0]], self.sites[tp[1]], self.sites[tp[2]]) {
//                 self.bad_triangles.insert(t_idx);
//             }
//         }
//         self.bad_edges = HashSet::new();
//         self.poly = HashSet::new();

//         for bt in &self.bad_triangles {
//             for e in &self.tri_edges[*bt] {
//                 if self.poly.contains(e) {
//                     self.poly.remove(e);
//                     self.bad_edges.insert(*e);
//                 } else {
//                     self.poly.insert(*e);
//                 }
//             }
//         }

//         // delete bad triangles
//         let mut btv: Vec<usize> = self.bad_triangles.iter().map(|x| *x).collect();
//         btv.sort();
//         for bti in btv.iter().rev() {
//             self.tri_edges.swap_remove(*bti);
//             self.tri_verts_idx.swap_remove(*bti);
//         }

//         for poly_edge in &self.poly {
//             self.tri_edges.push([Edge::new(poly_edge.a, poly_edge.b), Edge::new(poly_edge.b, self.sites.len() - 1), Edge::new(self.sites.len() - 1, poly_edge.a)]);
//             self.tri_verts_idx.push([poly_edge.a, poly_edge.b, self.sites.len() - 1]);
//         }
//     }
//     pub fn triangle_inds_of_site(&self, ind: usize) -> Vec<usize> {
//         // im making this O(edges) straight up
//         self.tri_edges.iter().enumerate().filter(|(i, t)| t.iter().any(|e| e.a == ind || e.b == ind)).map(|x| x.0).collect()
//     }

//     pub fn edges_of_site() {
//         // guess you could 
//     }
//     // each triangle the opposite 

//     pub fn compute_voronoi_edges(delaunay: &Delaunay) -> Vec<VoronoiEdge> {
//         let mut voronoi_edges = Vec::new();
    
//         for (tri_idx, tri) in delaunay.tri_verts_idx.iter().enumerate() {
//             let circumcenter = compute_circumcenter(&delaunay.sites, tri);
    
//             for edge in &delaunay.tri_edges[tri_idx] {
//                 if let Some(adj_tri_idx) = find_adjacent_triangle(delaunay, tri_idx, edge) {
//                     let adj_circumcenter = compute_circumcenter(&delaunay.sites, &delaunay.tri_verts_idx[adj_tri_idx]);
//                     voronoi_edges.push(VoronoiEdge {
//                         start: circumcenter,
//                         end: adj_circumcenter,
//                     });
//                 }
//             }
//         }
    
//         voronoi_edges
//     }

//     pub fn compute_voronoi(&mut self) {
//         let mut voronoi_verts = vec![];
//         let mut voronoi_edges = vec![];
//         let mut site_voronoi_verts = vec![];

//         for triangle_idx in 0..self.tri_verts_idx.len() {
//             let t = self.tri_verts_idx[triangle_idx];
//             let a = self.tri_verts_idx[t[0]];
//             let b = self.tri_verts_idx[t[1]];
//             let c = self.tri_verts_idx[t[2]];
//             voronoi_verts = circumcenter(a, b, c);
//         }
//         let site_triangles = self. // i think site triangles needed this
//         for site_idx in 0..self.sites.len() {

//             // site voronoi verts
//             // then sort them clockwise into edges
//         }


//         // self.voronoi_verts: Vec<Vec2>,
//         // voronoi_edges: Vec<Edge>,
//         // site_voronoi_verts: Vec<Vec2>,
//     }
    
// }


// // how do i compute circumcenters fookin ell