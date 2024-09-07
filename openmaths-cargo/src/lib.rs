// Create 3D Vector

// Add two vectors

// Subtract two vectors

// Multiply two vectors

// Divide two vectors

// Dot Product of two vectors

// Cross Product of two vectors

// Magnitude of a vector

// Normalize a vector 

// Create 3x3 Matrix struct

// Create 4x4 Matrix struct

// Make Identity Matrix

// Make Zero Matrix

// Determinant of a Matrix

// Transpose of a Matrix


mod circular_doubly_list;

pub mod openmaths {
  use wasm_bindgen::prelude::*;
  use serde::{Serialize, Deserialize};

  use crate::circular_doubly_list::{self, CircularDoubly};

  #[wasm_bindgen]
  #[derive(Copy, Clone, Serialize, Deserialize)]
  pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
  }

  #[wasm_bindgen]
  impl Vector3D {
    pub fn create(x: f64, y: f64, z: f64) -> Vector3D {
      Vector3D { x, y, z }
    }

    // TODO: This is not working
    pub fn update(&mut self, x: f64, y: f64, z: f64) {
      self.x = x;
      self.y = y;
      self.z = z;
    }

    pub fn add(&self, other: &Vector3D) -> Vector3D {
      Vector3D {
        x: self.x + other.x,
        y: self.y + other.y,
        z: self.z + other.z,
      }
    }

    pub fn subtract(&self, other: &Vector3D) -> Vector3D {
      Vector3D {
        x: self.x - other.x,
        y: self.y - other.y,
        z: self.z - other.z,
      }
    }

    pub fn add_scalar(&self, scalar: f64) -> Vector3D {
      Vector3D {
        x: self.x + scalar,
        y: self.y + scalar,
        z: self.z + scalar,
      }
    }

    pub fn add_extrude_in_up(&self, height: f64, up_vector: Vector3D) -> Vector3D {
      Vector3D {
        x: self.x + up_vector.x * height,
        y: self.y + up_vector.y * height,
        z: self.z + up_vector.z * height,
      }
    }

    pub fn cross(&self, other: &Vector3D) -> Vector3D {
      Vector3D {
        x: self.y * other.z - self.z * other.y,
        y: self.z * other.x - self.x * other.z,
        z: self.x * other.y - self.y * other.x,
      }
    }
  }

  #[wasm_bindgen]
  #[derive(Copy, Clone, Serialize, Deserialize)]
  pub struct Matrix3D {
      pub m11: f64, pub m12: f64, pub m13: f64,
      pub m21: f64, pub m22: f64, pub m23: f64,
      pub m31: f64, pub m32: f64, pub m33: f64,
  }

  #[wasm_bindgen]
  impl Matrix3D {
      pub fn set(
          m11: f64, m12: f64, m13: f64,
          m21: f64, m22: f64, m23: f64,
          m31: f64, m32: f64, m33: f64,
      ) -> Matrix3D {
          Matrix3D { m11, m12, m13, m21, m22, m23, m31, m32, m33 }
      }

      pub fn add(&self, other: &Matrix3D) -> Matrix3D {
          Matrix3D {
              m11: self.m11 + other.m11, m12: self.m12 + other.m12, m13: self.m13 + other.m13,
              m21: self.m21 + other.m21, m22: self.m22 + other.m22, m23: self.m23 + other.m23,
              m31: self.m31 + other.m31, m32: self.m32 + other.m32, m33: self.m33 + other.m33,
          }
      }

      pub fn subtract(&self, other: &Matrix3D) -> Matrix3D {
          Matrix3D {
              m11: self.m11 - other.m11, m12: self.m12 - other.m12, m13: self.m13 - other.m13,
              m21: self.m21 - other.m21, m22: self.m22 - other.m22, m23: self.m23 - other.m23,
              m31: self.m31 - other.m31, m32: self.m32 - other.m32, m33: self.m33 - other.m33,
          }
      }
  }

  #[wasm_bindgen]
  #[derive(Copy, Clone, Serialize, Deserialize)]
  pub struct ColorRGBA {
      pub r: f64,
      pub g: f64,
      pub b: f64,
      pub a: f64,
  }

  pub struct EdgeStructured {
    pub start: Vector3D,
    pub end: Vector3D,
    pub color: ColorRGBA,
  }

  #[derive(Clone, Serialize, Deserialize)]
  pub struct Geometry {
    pub vertices: Vec<Vector3D>,
    pub edges: Vec<Vec<u8>>,
    pub faces: Vec<Vec<u8>>,
  }

  impl Geometry {
    fn new() -> Geometry {
      Geometry {
        vertices: Vec::new(),
        edges: Vec::new(),
        faces: Vec::new(),
      }
    }

    pub fn get_geometry(&self) -> String {
      // serialize geometry
      let serialized = serde_json::to_string(&self).unwrap();
      serialized
    }
  }

  #[wasm_bindgen]
  #[derive(Clone, Serialize, Deserialize)]
  pub struct Mesh {
    pub position: Vector3D,
    pub position_matrix: Matrix3D,
    pub rotation: Vector3D,
    pub rotation_matrix: Matrix3D,
    pub scale: Vector3D,
    pub scale_matrix: Matrix3D,
    pub color: ColorRGBA,
    buf_faces: Vec<Vector3D>,
    poligon_vertices: Vec<Vector3D>,
    geometry: Geometry,
  }

  #[wasm_bindgen]
  impl Mesh {
    pub fn new() -> Mesh {
      Mesh {
        position: Vector3D::create(0.0, 0.0, 0.0),
        position_matrix: Matrix3D::set(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0),
        rotation: Vector3D::create(0.0, 0.0, 0.0),
        rotation_matrix: Matrix3D::set(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0),
        scale: Vector3D::create(1.0, 1.0, 1.0),
        scale_matrix: Matrix3D::set(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0),
        color: ColorRGBA { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
        buf_faces: Vec::new(),
        poligon_vertices: Vec::new(),
        geometry: Geometry::new(),
      }
    }

    pub fn copy_poligon_vertices(&mut self, vertices: Vec<Vector3D>) {
      self.poligon_vertices = vertices;
    }

    pub fn get_poligon_vertices(&self) -> Vec<Vector3D> {
      self.poligon_vertices.clone()
    }

    pub fn add_buf_face(&mut self, vertex: Vector3D) {
      self.buf_faces.push(vertex);
    }

    pub fn remove_buf_face(&mut self, index: usize) {
      if index < self.buf_faces.len() {
        self.buf_faces.remove(index);
      }
    }

    pub fn set_position(&mut self, position: Vector3D) {
      self.position = position;
    }

    pub fn get_position(&self) -> Vector3D {
      self.position
    }

    // TODO: Fix this iteratively
    pub fn set_extrude_height(&mut self, height: f64) {
      if self.poligon_vertices.len() > 0 {
        let mut buf_vertices = self.poligon_vertices.clone();
        let mut buf_edges: Vec<Vec<u8>> = Vec::new();
        let mut buf_faces: Vec<Vec<u8>> = Vec::new();

        let current_length = self.poligon_vertices.len();

        // Iterate the exisitng vertices and create edges
        for i in 0..self.poligon_vertices.len() {
          let edge = {
            vec![i as u8, ((i + 1) % self.poligon_vertices.len()) as u8]
          };
          buf_edges.push(edge);
        }

        // Creating First Polygon Faces 
        let mut face: Vec<u8> = Vec::new();
        for i in 0..self.poligon_vertices.len() {
          face.push(i as u8);
        }
        buf_faces.push(face);

        // Iterate over vertices and extrude in up direction
        for index in 0..self.poligon_vertices.len() {
          let new_vertex = self.poligon_vertices[index].clone().add_extrude_in_up(height, Vector3D::create(0.0, 1.0, 0.0));
          buf_vertices.push(new_vertex);

          let edge = {
            vec![index as u8, buf_vertices.len() as u8 - 1]
          };
          
          buf_edges.push(edge);
        }

        // Iterate over new vertices starting from the last index of the original vertices
        for i in current_length..buf_vertices.len() {
          if i < buf_vertices.len() - 1 {
            let edge = {
              vec![i as u8, (i + 1) as u8]
            };
            buf_edges.push(edge);
          } else {
            let edge = {
              vec![i as u8, (current_length) as u8]
            };
            buf_edges.push(edge);
          }
        }

        // Side Faces
        for i in 0..current_length {
          let next = (i + 1) % current_length;
          let face: Vec<u8> = vec![
            i as u8,
            next as u8,
            (next + current_length) as u8,
            i as u8 + current_length as u8,
          ];
          buf_faces.push(face);
        }

        // Bottom Face
        let mut face: Vec<u8> = Vec::new();
        for i in 0..current_length {
          face.push(i as u8 + current_length as u8);
        }
        buf_faces.push(face);
        
        let geometry = Geometry {
          vertices: buf_vertices,
          edges: buf_edges,
          faces: buf_faces,
        };
        
        self.geometry = geometry;
      }
    }

    pub fn get_geometry(&self) -> String {
      self.geometry.get_geometry()
    }
  }

  #[wasm_bindgen]
  #[derive(Clone)]
  pub struct Polygon {
    vertices: Vec<Vector3D>,
    pub position: Vector3D,
    pub extrude: bool,
  }

  #[wasm_bindgen]
  impl Polygon {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Polygon {
      Polygon {
        vertices: Vec::new(),
        position: Vector3D::create(0.0, 0.0, 0.0),
        extrude: false,
      }
    }

    pub fn add_vertex(&mut self, vertex: Vector3D) {
      self.vertices.push(vertex);
    }

    pub fn remove_vertex(&mut self, index: usize) {
      if index < self.vertices.len() {
        self.vertices.remove(index);
      }
    }

    pub fn update_vertex(&mut self, index: usize, vertex: Vector3D) {
      if index < self.vertices.len() {
        self.vertices[index] = vertex;
      }
    }

    // Instead of returning Vec<Vector3D>, expose controlled access
    pub fn get_vertex(&self, index: usize) -> Option<Vector3D> {
      self.vertices.get(index).copied()
    }

    pub fn vertex_count(&self) -> usize {
      self.vertices.len()
    }

    pub fn get_all_vertices(&self) -> Vec<Vector3D> {
      self.vertices.clone()
    }

    pub fn clear_vertices(&mut self) {
      self.vertices.clear();
    }

    pub fn set_position(&mut self, position: Vector3D) {
      self.position = position;
    }

    pub fn get_position(&self) -> Vector3D {
      self.position
    }

    pub fn set_extrude(&mut self, extrude: bool) -> Mesh {
      self.extrude = extrude;

      let mut mesh = Mesh::new();
      mesh.position = self.position;
      mesh.poligon_vertices = self.vertices.clone();
      mesh
    }
  }

  #[wasm_bindgen]
  #[derive(Clone)]
  pub struct Triangle {
    pub a: Vector3D,
    pub b: Vector3D,
    pub c: Vector3D,
  }

  #[wasm_bindgen]
  impl Triangle {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Triangle {
      Triangle {
        a: Vector3D::create(0.0, 0.0, 0.0),
        b: Vector3D::create(0.0, 0.0, 0.0),
        c: Vector3D::create(0.0, 0.0, 0.0),
      }
    }

    pub fn set_vertices(&mut self, a: Vector3D, b: Vector3D, c: Vector3D) {
      self.a = a;
      self.b = b;
      self.c = c;
    }

    // pub fn get_area(&self) -> f64 {
    //   let normal = self.get_normal();
    //   let area = 0.5 * normal.magnitude();
    //   area
    // }

    pub fn get_all_vertices(&self) -> Vec<Vector3D> {
      vec![self.a, self.b, self.c]
    }

    pub fn is_point_in_triangle(&self, p : Vector3D) -> bool {
      let ab = self.b.clone().subtract(&self.a);
      let bc = self.c.clone().subtract(&self.b);
      let ca = self.a.clone().subtract(&self.c);
  
      let ap = p.clone().subtract(&self.a);
      let bp = p.clone().subtract(&self.b);
      let cp = p.clone().subtract(&self.c);
  
      let cross_abp = ab.clone().cross(&ap);
      let cross_bcp = bc.clone().cross(&bp);
      let cross_cap = ca.clone().cross(&cp);
  
      if (
          cross_abp.y > 0.0 &&
          cross_bcp.y > 0.0 &&
          cross_cap.y > 0.0
        ) || (
          cross_abp.y < 0.0 &&
          cross_bcp.y < 0.0 &&
          cross_cap.y < 0.0
        ) {
          return true;
        }
      false
    }
  }

  #[wasm_bindgen]
  pub fn triangulate(vertices: Vec<Vector3D>) -> Vec<Vector3D> {
    let mut triangles: Vec<Vector3D> = Vec::new();

    if vertices.len() > 2 {
      for i in 1..vertices.len() - 1 {
        triangles.push(vertices[0]);
        triangles.push(vertices[i]);
        triangles.push(vertices[i + 1]);
      }
    }

    triangles
  }

  #[wasm_bindgen]
  // pub fn triangulate_by_index(vertices: Vec<Vector3D>) -> Vec<u64> {
  pub fn triangulate_by_index() -> String {
    let mut triangle_indices: Vec<Vec<u32>> = Vec::new();

    let mut test_vertices: Vec<Vector3D> = Vec::new();

    // Adding vertices
    test_vertices.push(Vector3D::create(3.0, 0.0, 48.0));
    test_vertices.push(Vector3D::create(52.0, 0.0, 8.0));
    test_vertices.push(Vector3D::create(99.0, 0.0, 50.0));
    test_vertices.push(Vector3D::create(138.0, 0.0, 25.0));
    test_vertices.push(Vector3D::create(175.0, 0.0, 77.0));
    test_vertices.push(Vector3D::create(131.0, 0.0, 72.0));
    test_vertices.push(Vector3D::create(111.0, 0.0, 113.0));
    test_vertices.push(Vector3D::create(72.0, 0.0, 43.0));
    test_vertices.push(Vector3D::create(26.0, 0.0, 55.0));
    test_vertices.push(Vector3D::create(29.0, 0.0, 100.0));

    // Getting the indices of test_vertices
    let mut vertex_indices: Vec<u32> = (0..test_vertices.len() as u32).collect();
    let mut vertex_indices_copy = vertex_indices.clone();

    let mut ear_tip_index = 0;
    let mut triangle_indices_count = 0;
    
    'check_number_triangles: while triangle_indices_count < test_vertices.len() - 2 {
      let eartip = vertex_indices[ear_tip_index];
      let b = if ear_tip_index == 0 { vertex_indices.len() - 1 } else { ear_tip_index - 1 };
      let c = if ear_tip_index == vertex_indices.len() - 1 { 0 } else { ear_tip_index + 1 };

      // if not convex, break
      let ab = test_vertices[b as usize].clone().subtract(&test_vertices[eartip as usize]);
      let ac = test_vertices[c as usize].clone().subtract(&test_vertices[eartip as usize]);

      let cross_product = ab.clone().cross(&ac);
      if cross_product.y > 0.0 {
        
        // check if any other vertex is inside the triangle
        let mut is_ear = true;
        for i in vertex_indices_copy.clone() {
          if i != b as u32 && i != eartip && i != c as u32 {
            let p = test_vertices[i as usize].clone();
            let mut triangle = Triangle::new();
            triangle.set_vertices(test_vertices[b as usize], test_vertices[eartip as usize], test_vertices[c as usize]);
            if triangle.is_point_in_triangle(p) {
              is_ear = false;
              break;
            }
          }
        }
        
        // if ear, add to triangle_indices
        let ear: Vec<u32> = vec![b as u32, eartip as u32, c as u32];
        triangle_indices.push(ear);
        triangle_indices_count += 1;
      }

      // if not ear, break
      ear_tip_index += 1;
      if ear_tip_index == vertex_indices.len() {
        break 'check_number_triangles;
      }
    }
    
    // // calculate cross products - OK
    // // first run without removing any vertices

    // while treated_vertices.len() < test_vertices.len() - 2
    // {
    //   let mut untreated_vertices = vertex_indices.clone();
    //   let mut untreaded_v_length:u32 = untreated_vertices.len().try_into().unwrap();

    //   'outer: for i in 0..untreaded_v_length {
    //     let ear_tip_index = i;
    //     let b_tip_index = if i == 0 { untreaded_v_length - 1 } else { i - 1 };
    //     let c_tip_index = if i == untreaded_v_length - 1 { 0 } else { i + 1 };

    //     let ear_vertex = test_vertices[ear_tip_index as usize];
    //     let b_vertex = test_vertices[b_tip_index as usize];
    //     let c_vertex = test_vertices[c_tip_index as usize];

    //     let ab = b_vertex.clone().subtract(&ear_vertex);
        
    //     let ac = c_vertex.clone().subtract(&ear_vertex);

    //     let cross_product = ab.clone().cross(&ac);
    //     // cross_producs.push(cross_product);
    //     // let c_test = vec![ab, ac, cross_product];
    //     // cross_test.push(c_test);

    //     if cross_product.y > 0.0 {
    //       let mut triangle = Triangle::new();
    //       triangle.set_vertices(ear_vertex, b_vertex, c_vertex);

    //       for i in vertex_indices.clone() {
    //         if (i != ear_tip_index && i != b_tip_index && i != c_tip_index) || !treated_vertices.contains(&i) {
    //           treated_vertices.push(i);

    //           let found_triangle = vec![ear_tip_index, b_tip_index, c_tip_index];
    //           triangle_indices.push(found_triangle);
              
    //           // since number of triangles is equal to number of vertices - 2
    //           if treated_vertices.len() == untreaded_v_length as usize - 2 {
    //             break 'outer;
    //           }
    //         }
    //       }
        
    //       // let found_triangle = vec![ear_tip_index, b_tip_index, c_tip_index];
    //       // triangle_indices.push(found_triangle);
    //     }
    //   }
    // }

    // serde_json::to_string(&cross_test).unwrap()
    // serde_json::to_string(&vertex_indices).unwrap()
    // serde_json::to_string(&treated_vertices).unwrap()

    serde_json::to_string(&triangle_indices).unwrap()
  }
}

