extern crate cgmath;
extern crate meshlite;
extern crate petgraph;

use meshlite::mesh;
use meshlite::subdivide;
use meshlite::iterator;
use meshlite::util;
use meshlite::wrap;
use meshlite::bmesh;
use meshlite::triangulate;
use meshlite::wavefront;
use meshlite::debug;

use cgmath::Matrix4;
use cgmath::prelude::*;
use cgmath::Vector3;
use cgmath::Point3;
use cgmath::Deg;
use cgmath::Rad;

use wrap::GiftWrapper;
use subdivide::CatmullClarkSubdivider;
use triangulate::Triangulate;
use subdivide::Subdivide;

use mesh::Mesh;
use bmesh::Bmesh;

use mesh::Export;
use mesh::Import;

fn cube() -> Mesh {
    let mut m = Mesh::new();
    let face_id = m.add_plane(1.0, 1.0);
    let normal = m.face_norm(face_id);
    m.extrude_face(face_id, normal, 1.0).translate(0.0, 0.0, -0.5);
    m
}

fn plane() -> Mesh {
    let mut m = Mesh::new();
    let face_id = m.add_plane(1.0, 1.0);
    m
}

fn main() {
    //let mut m = Mesh::new();
    //let face_id = m.add_plane(2.0, 1.0);
    //let normal = m.face_norm(face_id);
    //m.extrude_face(face_id, normal, 1.0);
    //m.save_obj("test.obj").expect("save file failed");

    /*
    let mut m1 = cube();
    let v1 = Vector3 {x: 0.0, y: -1.0, z: 0.0};
    let mut mat1 = Matrix4::from_translation(v1);
    let matr = Matrix4::from_angle_x(Rad::from(Deg(-90.0)));
    mat1 = mat1 * matr;
    m1.transform(&mat1);

    let mut m2 = plane();
    let v2 = Vector3 {x: 0.0, y: 2.0, z: 0.0};
    let mut mat2 = Matrix4::from_translation(v2);
    let matr = Matrix4::from_angle_x(Rad::from(Deg(90.0)));
    mat2 = mat2 * matr;
    m2.transform(&mat2);

    let mut m3 = m1 + m2;

    let mut gw = GiftWrapper::new();
    gw.stitch_two_faces(&mut m3, 1, 7);

    m3.save_obj("test.obj").expect("save file failed");
    */

    //m.load_obj("/Users/jeremy/Repositories/dust3d/gourd.obj").expect("load file failed");
    //let mut sm = CatmullClarkSubdivider::new(&mut m);
    //sm.generated_mesh_mut().save_obj("test.obj").expect("save file failed");
    //println!("Mesh debug info: {:?}", m);

    //let mut bmesh = Bmesh::new();
    /*
    let node1 = bmesh.add_node(Point3 {x: -1.0, y: 1.5, z: 1.0}, 0.25);
    let node2 = bmesh.add_node(Point3 {x: 0.0, y: 0.0, z: 0.0}, 0.3);
    let node3 = bmesh.add_node(Point3 {x: 1.0, y: -1.5, z: -1.0}, 0.5);
    let node4 = bmesh.add_node(Point3 {x: 1.0, y: 1.5, z: -1.0}, 0.2);
    bmesh.add_edge(node1, node2);
    bmesh.add_edge(node2, node3);
    bmesh.add_edge(node2, node4);
    let mut mesh = bmesh.generate_mesh(node2);
    */

    /*
    x:1.633484 y:1.411765 z:0.000000 r:0.144590
    x:1.778280 y:0.411765 z:0.000000 r:0.191739
    */
    /*
    let mut bmesh = Bmesh::new();
    let node1 = bmesh.add_node(Point3 {x:1.633484, y:1.411765, z:0.000000}, 0.144590);
    let node2 = bmesh.add_node(Point3 {x:1.778280, y:0.411765, z:0.000000}, 0.191739);
    bmesh.add_edge(node1, node2);
    let mut mesh = bmesh.generate_mesh(node1);
    mesh.export("test.obj").expect("save file failed");
    */

    /*
    let node0 = bmesh.add_node(Point3 {x: -2.07575, y: 1.53902, z: 0.04122}, 0.25);                                    
    let node1 = bmesh.add_node(Point3 {x: 2.40837, y: 2.34882, z: 0.48585}, 0.3);
    let node2 = bmesh.add_node(Point3 {x: -0.91403, y: 0.77069, z: 0.62299}, 0.5);         
    let node3 = bmesh.add_node(Point3 {x: 2.25224, y: 0.74973, z: 0.85115}, 0.5);
    let node4 = bmesh.add_node(Point3 {x: 0.0, y: 0.0, z: 0.0}, 0.82);
    let node5 = bmesh.add_node(Point3 {x: 0.00920, y: -0.66115, z: -2.04601}, 0.5);
    let node6 = bmesh.add_node(Point3 {x: 0.01726, y: -0.88224, z: -2.87471}, 0.2);
    let node7 = bmesh.add_node(Point3 {x: 0.0, y: -2.0, z: 0.00}, 0.2);
    let node8 = bmesh.add_node(Point3 {x: -0.3, y: -2.8, z: 0.13}, 0.5);
    let node9 = bmesh.add_node(Point3 {x: -0.3, y: -3.8, z: 1.13}, 0.6);
    bmesh.add_edge(node0, node2);
    bmesh.add_edge(node2, node4);
    bmesh.add_edge(node4, node3);
    bmesh.add_edge(node3, node1);
    bmesh.add_edge(node4, node5);
    bmesh.add_edge(node5, node6);
    bmesh.add_edge(node4, node7);
    bmesh.add_edge(node7, node8);
    bmesh.add_edge(node8, node9);
    let mut mesh = bmesh.generate_mesh(node4);
    //mesh.import("test.obj").expect("save file failed");

    //let mut cc = CatmullClarkSubdivider::new(&mut mesh);
    //cc.generate().save_obj("test.obj").expect("save file failed");

    //cc.generate().triangulate().save_obj("test.obj").expect("save file failed");

    mesh.subdivide().triangulate().export("test.obj").expect("save file failed");
    */

    /*
    let mesh Mesh::new();
    let point = Point3 {x: 0.25, y: 0.25, z: 0.25};
    let norm = Vector3 {x: 0.0, y: 0.2, z: 0.78};
    let (front_mesh, back_mesh) = mesh.split_mesh_by_plane(point, norm);
    front_mesh.export("test.obj").expect("save file failed");
    */

    /*
    let mut mesh = Mesh::new();
    mesh.import("/Users/jeremy/ball.obj");
    let point = Point3 {x: 0.25, y: 0.25, z: 0.25};
    let norm = Vector3 {x: 0.0, y: 0.2, z: 0.78};
    let (mut front_mesh, mut back_mesh) = mesh.split_mesh_by_plane(point, norm);
    let mut merged_mesh = Mesh::new();
    merged_mesh += front_mesh;
    back_mesh.translate(0.0, 0.0, -0.5);
    merged_mesh += back_mesh;
    mesh.translate(0.0, 0.0, -3.0);
    merged_mesh += mesh;
    merged_mesh.export("test.obj").expect("save file failed");
    */

    /*
    let mut m1 = Mesh::new();
    let mut m2 = Mesh::new();
    m1.import("/Users/jeremy/cube.obj");
    m1.scale(0.60);
    m2.import("/Users/jeremy/ball.obj");
    //m2.scale(0.75);
    m2.translate(0.3, 0.3, 0.3);
    //m2.export("test.obj").expect("save file failed");
    m1.union_mesh(&m2).export("test.obj").expect("save file failed");
    */

    let mut bmesh = Bmesh::new();
    /*
    NOMESH:
    let node0 = bmesh.add_node(Point3 {x:0.162995, y:-0.413932, z:-0.322483}, 0.0860678);
    let node1 = bmesh.add_node(Point3 {x:0.076923, y:-0.214899, z:0.132066}, 0.150619);
    let node2 = bmesh.add_node(Point3 {x:0.302851, y:0.325713, z:0.234263}, 0.174287);
    let node3 = bmesh.add_node(Point3 {x:-0.391071, y:0.024476, z:-0.322483}, 0.0860678);
    bmesh.add_edge(node1, node0);
    bmesh.add_edge(node1, node2);
    bmesh.add_edge(node1, node3);
    */
    let node1 = bmesh.add_node(Point3 {x:-0.384956, y:0.0752215, z:-0.166669}, 0.0943954);
    let node0 = bmesh.add_node(Point3 {x:0.0988248, y:-0.0545711, z:0.166669}, 0.0943954);
    let node2 = bmesh.add_node(Point3 {x:0.384956, y:0.405605, z:-0.163724}, 0.0943954);
    let node3 = bmesh.add_node(Point3 {x:0.222719, y:-0.405605, z:-0.166669}, 0.0943954);
    println!("node0:{:?} node1:{:?} node2:{:?} node3:{:?}", node0, node1, node2, node3);
    bmesh.add_edge(node1, node0);
    bmesh.add_edge(node2, node0);
    bmesh.add_edge(node3, node0);
    let mut mesh = bmesh.generate_mesh();
    mesh.export("test.obj").expect("save file failed");
}

