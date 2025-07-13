/**
 * Simple Utility To Triangulate A Polygon
 * TODO: Move this to OpenGeometry crate after MVP
 */


pub fn tricut(polygon: Vec<Vec<f64>>) -> Vec<Vec<Vec<f64>>> {
  let mut triangles: Vec<Vec<Vec<f64>>> = Vec::new();

  for i in 1..polygon.len() - 1 {
      let triangle = vec![polygon[0].clone(), polygon[i].clone(), polygon[i + 1].clone()];

      triangles.push(triangle);
  }

  triangles
}