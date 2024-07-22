# Computer Graphics with Rust - Polygon Drawing and Filling

This project demonstrates drawing and filling polygons using the Rust programming language. The polygons are drawn with specific colors and borders, and the program generates bitmap images as output. The project includes four polygons, with the fourth polygon containing a hole.

## Features

- Draw and fill multiple polygons with specified colors and borders.
- Generate bitmap images of the polygons.
- Handle polygon holes for complex shapes.

## Polygons

1. **Polygon 1:** Yellow fill with white border.
   - Vertices: (165, 380), (185, 360), (180, 330), (207, 345), (233, 330), (230, 360), (250, 380), (220, 385), (205, 410), (193, 383)

2. **Polygon 2:** Blue fill with white border.
   - Vertices: (321, 335), (288, 286), (339, 251), (374, 302)

3. **Polygon 3:** Red fill with white border.
   - Vertices: (377, 249), (411, 197), (436, 249)

4. **Polygon 4:** Green fill with white border. Contains a hole (Polygon 5).
   - Vertices: (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52), (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230), (597, 215), (552, 214), (517, 144), (466, 180)
   - **Hole (Polygon 5):** (682, 175), (708, 120), (735, 148), (739, 170)

## Getting Started

### Prerequisites

- Rust: Ensure you have Rust installed. You can download and install Rust from [rust-lang.org](https://www.rust-lang.org/).

### Cloning the Repository

To clone the repository, run the following command:

```sh
git clone https://github.com/Diegoval-Dev/GC-Lab1
cd GC-Lab1
```

### Running the Program

To run the program, use the following command:

```sh
cargo run
```

The program will generate bitmap images of the polygons in the `output` directory.

