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

pub mod openmaths {
  use wasm_bindgen::prelude::*;
  use serde::{Serialize, Deserialize};

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

  #[wasm_bindgen]
  #[derive(Clone, Serialize, Deserialize)]
  pub struct Edge {
    indices: Vec<u64>,
  }

  pub struct EdgeStructured {
    pub start: Vector3D,
    pub end: Vector3D,
    pub color: ColorRGBA,
  }

  #[derive(Clone, Serialize, Deserialize)]
  pub struct Geometry {
    pub vertices: Vec<Vector3D>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Vec<Vector3D>>,
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
        let mut buf_edges: Vec<Edge> = Vec::new();
        let mut buf_faces: Vec<Vec<Vector3D>> = Vec::new();

        let current_length = self.poligon_vertices.len();

        // Iterate the exisitng vertices and create edges
        for i in 0..self.poligon_vertices.len() {
          let start = self.poligon_vertices[i].clone();
          let end = self.poligon_vertices[(i + 1) % self.poligon_vertices.len()].clone();
          let edge = Edge {
            start: start,
            end: end,
          };
          buf_edges.push(edge);
        }

        // Creating First Polygon Faces 
        let mut face: Vec<Vector3D> = Vec::new();
        face = self.poligon_vertices.clone();
        buf_faces.push(face);

        // Iterate over vertices and extrude in up direction
        for vertex in self.poligon_vertices.iter() {
          let new_vertex = vertex.clone().add_extrude_in_up(height, Vector3D::create(0.0, 1.0, 0.0));
          buf_vertices.push(new_vertex);

          let edge = Edge {
            start: vertex.clone(),
            end: new_vertex,
          };
          buf_edges.push(edge);

          // Creating Second Polygon Faces
          // for i in current_length..buf_vertices.len() {
          //   // check if adjacent vertices are available and edge is present for them
          //   self.try_create_face(&mut buf_faces, &buf_vertices, i, i - current_length);
          // }
        }

        // Iterate over new vertices starting from the last index of the original vertices
        for i in current_length..buf_vertices.len() {
          let start = buf_vertices[i].clone();
          let end = buf_vertices[(i + 1) % buf_vertices.len()].clone();
          let edge = Edge {
            start: start,
            end: end,
          };
          buf_edges.push(edge);
        }
    
        let geometry = Geometry {
          vertices: buf_vertices,
          edges: buf_edges,
          faces: buf_faces,
        };
        
        self.geometry = geometry;
      }
    }

    fn try_create_face(&self, buf_faces: &mut Vec<Vec<Vector3D>>, buf_vertices: &Vec<Vector3D>, i: usize, j: usize) {
      if i % 2 == 0 {
        let face = vec![
          buf_vertices[i].clone(),
          buf_vertices[j].clone(),
          buf_vertices[(j + 1) % buf_vertices.len()].clone(),
        ];
        buf_faces.push(face);
      } else {
        let face = vec![
          buf_vertices[i].clone(),
          buf_vertices[(j + 1) % buf_vertices.len()].clone(),
          buf_vertices[i - 1].clone(),
        ];
        buf_faces.push(face);
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
}
