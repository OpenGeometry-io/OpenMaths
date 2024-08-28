const vertices2D = [
  {x: 1, y: 1},
  {x: 5, y: 1},
  {x: 4, y: 4},
  // {x: 2, y: 5}
];

const height = 10;

const vertices3D = [];

vertices2D.forEach(vertex => {
  vertices3D.push({x: vertex.x, y: vertex.y, z: 0});
  vertices3D.push({x: vertex.x, y: vertex.y, z: height});
});

const edges = [];

for (let i = 0; i < vertices2D.length; i++) {
  const next = (i + 1) % vertices2D.length;
  edges.push([i, next]); 
  edges.push([i + vertices2D.length, next + vertices2D.length]);
  edges.push([i, i + vertices2D.length]); 
}

const faces = [];

const bottomFace = [];
for (let i = 0; i < vertices2D.length; i++) {
  bottomFace.push(i);
}
faces.push(bottomFace);

const topFace = [];
for (let i = 0; i < vertices2D.length; i++) {
  topFace.push(i + vertices2D.length);
}
faces.push(topFace.reverse()); 

for (let i = 0; i < vertices2D.length; i++) {
  const next = (i + 1) % vertices2D.length;
  faces.push([i, next, next + vertices2D.length, i + vertices2D.length]);
}

const mesh = {
  vertices: vertices3D,
  edges: edges,
  faces: faces
};

console.log(mesh);