use std::collections::HashMap;

// Code for representing a Rubiks cube of arbitrary size

fn write_2d_vec<T>(v: &mut Vec<T>, size: &u8, x: &u8, y: &u8, val: T) {
    v[usize::from(x + y * size)] = val;
}

fn read_2d_vec<T>(v: &Vec<T>, size: &u8, x: &u8, y: &u8) -> Option<&T> {
    v.get(usize::from(x + y * size))
}

// Function to rotate a 2d array clockwise
// Transposes the matrix and reverses the rows
// | A B |
// | C D |
// Transposed
// | A C |
// | B D |
// Reversed
// | C A |
// | D B |
fn rotate_vec_cw(base: &Vec<RubiksColor>, size: &u8) -> Vec<RubiksColor> {
    // Step 1: Initialize a new vec that will hold the array
    let mut r_vec = vec![RubiksColor::UP; usize::from(size * size)];
    // Step 2: Itterate over x y positions of input, write them to output
    for x in 0..*size {
        for y in 0..*size {
            let base_value = read_2d_vec(&base, *size, &x, &y)
                .expect("This shouldn't happen unless you are lying to the API, naughty.");
            let mut tx = y;
            let ty = x;
            tx = (size - 1) - tx;
            write_2d_vec(&mut r_vec, *size, &tx, &ty, *base_value);
        }
    }
    return r_vec;
}

fn rotate_vec_ccw(base: &Vec<RubiksColor>, size: &u8) -> Vec<RubiksColor> {
    // Step 1: Initialize a new vec that will hold the array
    let mut r_vec = vec![RubiksColor::UP; usize::from(size * size)];
    // Step 2: Itterate over x y positions of input, write them to output
    for x in 0..*size {
        for y in 0..*size {
            let base_value = read_2d_vec(&base, *size, x, y)
                .expect("This shouldn't happen unless you are lying to the API, naughty.");
            let mut tx = (size - 1) - x;
            let mut ty = y;
            let temp = tx;
            tx = ty;
            ty = temp;
            write_2d_vec(&mut r_vec, *size, tx, ty, *base_value);
        }
    }
    return r_vec;
}

// Face Colors are represented by their direction in the "solved" state
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum RubiksColor {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    FORWARD,
    BACK,
}

impl RubiksColor {
    fn opposite(&self) -> RubiksColor {
        match self {
            RubiksColor::UP => RubiksColor::DOWN,
            RubiksColor::DOWN => RubiksColor::UP,
            RubiksColor::LEFT => RubiksColor::RIGHT,
            RubiksColor::RIGHT => RubiksColor::LEFT,
            RubiksColor::FORWARD => RubiksColor::BACK,
            RubiksColor::BACK => RubiksColor::FORWARD,
        }
    }
}

enum TurnDirection {
    CLOCKWISE,
    COUNTERCLOCKWISE,
}

impl TurnDirection {
    fn opposite(&self) -> TurnDirection {
        match self {
            TurnDirection::CLOCKWISE => TurnDirection::COUNTERCLOCKWISE,
            TurnDirection::COUNTERCLOCKWISE => TurnDirection::CLOCKWISE,
        }
    }
}

#[derive(Clone)]
struct RubiksCube {
    face_size: u8,                                 // The edge length of the cube
    faces: HashMap<RubiksColor, Vec<RubiksColor>>, // TODO: Some data structure to represent the faces
}

impl RubiksCube {
    // Initializes a new cube of given size with all faces set properly
    fn new(face_size: u8) -> Self {
        let face_squares: usize = (face_size * face_size).into();

        let u_face = vec![RubiksColor::UP; face_squares];
        let d_face = vec![RubiksColor::DOWN; face_squares];
        let l_face = vec![RubiksColor::LEFT; face_squares];
        let r_face = vec![RubiksColor::RIGHT; face_squares];
        let f_face = vec![RubiksColor::FORWARD; face_squares];
        let b_face = vec![RubiksColor::BACK; face_squares];
        let faces = HashMap::from([
            (RubiksColor::UP, u_face),
            (RubiksColor::DOWN, d_face),
            (RubiksColor::LEFT, l_face),
            (RubiksColor::RIGHT, r_face),
            (RubiksColor::FORWARD, f_face),
            (RubiksColor::BACK, b_face),
        ]);
        Self { face_size, faces }
    }

    // Preform a move on a rubiks cube
    // Takes in the targeted face, how many layers down from that face is the layer being turned,
    // and the turning drection
    // NOTE: Layers are 0 indexed, passing in 0 will turn the whole input face
    fn do_move(
        &mut self,
        face_target: RubiksColor,
        layer: u8,
        direction: TurnDirection,
    ) -> RubiksCube {
        let new = self.clone();
        match face_target {
            RubiksColor::UP => new.rotate_top(&direction, &layer),
            RubiksColor::DOWN => {
                new.rotate_top(&direction.opposite(), &(self.face_size.clone() - layer - 1))
            }

            RubiksColor::RIGHT => new.rotate_right(&direction, &layer),
            RubiksColor::LEFT => {
                new.rotate_right(&direction.opposite(), &(self.face_size.clone() - layer - 1))
            }
            RubiksColor::FORWARD => new.rotate_front(&direction, &layer),
            RubiksColor::BACK => {
                new.rotate_front(&direction.opposite(), &(self.face_size.clone() - layer - 1))
            }
        }
    }

    // Rotates the cube along the axis facing the front direction
    fn rotate_front(mut self, direction: &TurnDirection, layer: &u8) -> RubiksCube {
        // Step 1: Determine if the front face is being rotated, if so rotate it
        if *layer == 0 {
            let face = self
                .faces
                .get_mut(&RubiksColor::FORWARD)
                .unwrap() // We will never not have this be filled out so we can just unwrap here
                .to_owned();
            let new_face = match direction {
                TurnDirection::CLOCKWISE => rotate_vec_cw(&face, &self.face_size),
                TurnDirection::COUNTERCLOCKWISE => rotate_vec_ccw(&face, &self.face_size),
            };
            self.faces.insert(RubiksColor::FORWARD, new_face);
        }
        // Step 2: Determine if the back face is being rotated, if so rotate it
        if *layer == self.face_size - 1 {
            let face = self.faces.get_mut(&RubiksColor::BACK).unwrap().to_owned();
            let new_face = match direction {
                TurnDirection::CLOCKWISE => rotate_vec_ccw(&face, &self.face_size),
                TurnDirection::COUNTERCLOCKWISE => rotate_vec_cw(&face, &self.face_size),
            };
            self.faces.insert(RubiksColor::BACK, new_face);
        }
        // Step 3: Determine the strips allong the top, bottom, left, and right that are going to
        // be swapped
        // Step 3.1: TOP [0, Size - 1 - Layer] -> [Size - 1, Size - 1 - Layer]
        let mut top_strip: Vec<RubiksColor> = vec![];
        let mut top_face = self.faces.get_mut(&RubiksColor::UP).unwrap().to_owned();
        for x in 0..self.face_size {
            top_strip.push(
                read_2d_vec(
                    &top_face,
                    &self.face_size,
                    &x,
                    &(self.face_size - 1 - layer),
                )
                .unwrap()
                .clone(),
            );
        }
        // Step 3.2: RIGHT [Layer, 0] -> [Layer, Size -1]
        let mut right_strip: Vec<RubiksColor> = vec![];
        let mut right_face = self.faces.get_mut(&RubiksColor::RIGHT).unwrap().to_owned();
        for y in 0..self.face_size {
            right_strip.push(
                read_2d_vec(&right_face, &self.face_size, &layer, &y)
                    .unwrap()
                    .clone(),
            );
        }
        // Step 3.3: BOTTOM [Size -1, Layer] -> [0, Layer]
        let mut bottom_strip: Vec<RubiksColor> = vec![];
        let mut bottom_face = self.faces.get_mut(&RubiksColor::DOWN).unwrap().to_owned();
        for x in (0..self.face_size).rev() {
            bottom_strip.push(
                read_2d_vec(
                    &bottom_face,
                    &self.face_size,
                    &x,
                    &(self.face_size - 1 - layer),
                )
                .unwrap()
                .clone(),
            );
        }
        // Step 3.4: LEFT [Size - Layer, Size] -> [Size - Layer, 0]
        let mut left_strip: Vec<RubiksColor> = vec![];
        let mut left_face = self.faces.get_mut(&RubiksColor::LEFT).unwrap().to_owned();
        for y in (0..self.face_size).rev() {
            left_strip.push(
                read_2d_vec(
                    &left_face,
                    &self.face_size,
                    &(self.face_size - 1 - layer),
                    &y,
                )
                .unwrap()
                .clone(),
            );
        }
        // Step 4: Swap the faces forwards if clockwise, backwards if counterclockwise
        let mut new_top: Vec<RubiksColor>;
        let mut new_right: Vec<RubiksColor>;
        let mut new_bottom: Vec<RubiksColor>;
        let mut new_left: Vec<RubiksColor>;

        match direction {
            TurnDirection::CLOCKWISE => {
                new_top = left_strip;
                new_right = top_strip;
                new_bottom = right_strip;
                new_left = bottom_strip;
            }
            TurnDirection::COUNTERCLOCKWISE => {
                new_top = right_strip;
                new_right = bottom_strip;
                new_bottom = left_strip;
                new_left = top_strip;
            }
        };
        // Step 5: Write new strips to new faces
        new_top.reverse();
        for x in 0..self.face_size {
            write_2d_vec(
                &mut top_face,
                &self.face_size,
                &x,
                &(self.face_size - 1 - layer),
                new_top.pop().unwrap(),
            )
        }
        new_right.reverse();
        for y in 0..self.face_size {
            write_2d_vec(
                &mut right_face,
                &self.face_size,
                &layer,
                &y,
                new_right.pop().unwrap(),
            )
        }
        new_bottom.reverse();
        for x in (0..self.face_size).rev() {
            write_2d_vec(
                &mut bottom_face,
                &self.face_size,
                &x,
                &layer,
                new_bottom.pop().unwrap(),
            )
        }
        new_left.reverse();
        for y in (0..self.face_size).rev() {
            write_2d_vec(
                &mut left_face,
                &self.face_size,
                &(self.face_size - 1 - layer),
                &y,
                new_left.pop().unwrap(),
            )
        }

        // Step 6: Place new faces back onto cube
        self.faces.insert(RubiksColor::UP, top_face);
        self.faces.insert(RubiksColor::RIGHT, right_face);
        self.faces.insert(RubiksColor::DOWN, bottom_face);
        self.faces.insert(RubiksColor::LEFT, left_face);

        self
    }

    // Rotates the cube along the axis facing the right direction
    fn rotate_top(mut self, direction: &TurnDirection, layer: &u8) -> RubiksCube {
        // Step 1: Determine if the front face is being rotated, if so rotate it
        if *layer == 0 {
            let face = self
                .faces
                .get_mut(&RubiksColor::UP)
                .unwrap() // We will never not have this be filled out so we can just unwrap here
                .to_owned();
            let new_face = match direction {
                TurnDirection::CLOCKWISE => rotate_vec_cw(&face, &self.face_size),
                TurnDirection::COUNTERCLOCKWISE => rotate_vec_ccw(&face, &self.face_size),
            };
            self.faces.insert(RubiksColor::UP, new_face);
        }
        // Step 2: Determine if the back face is being rotated, if so rotate it
        if *layer == self.face_size - 1 {
            let face = self.faces.get_mut(&RubiksColor::DOWN).unwrap().to_owned();
            let new_face = match direction {
                TurnDirection::CLOCKWISE => rotate_vec_ccw(&face, &self.face_size),
                TurnDirection::COUNTERCLOCKWISE => rotate_vec_cw(&face, &self.face_size),
            };
            self.faces.insert(RubiksColor::DOWN, new_face);
        }
        // Step 3: Determine the strips allong the top, bottom, left, and right that are going to
        // be swapped
        // Step 3.1: ALL faces itterate the same way here
        let mut right_strip: Vec<RubiksColor> = vec![];
        let mut right_face = self.faces.get_mut(&RubiksColor::RIGHT).unwrap().to_owned();
        let mut left_strip: Vec<RubiksColor> = vec![];
        let mut left_face = self.faces.get_mut(&RubiksColor::LEFT).unwrap().to_owned();
        let mut front_strip: Vec<RubiksColor> = vec![];
        let mut front_face = self
            .faces
            .get_mut(&RubiksColor::FORWARD)
            .unwrap()
            .to_owned();
        let mut back_strip: Vec<RubiksColor> = vec![];
        let mut back_face = self.faces.get_mut(&RubiksColor::BACK).unwrap().to_owned();

        for x in (0..self.face_size).rev() {
            right_strip.push(
                read_2d_vec(&right_face, &self.face_size, &x, &layer)
                    .unwrap()
                    .clone(),
            );
            front_strip.push(
                read_2d_vec(&front_face, &self.face_size, &x, &layer)
                    .unwrap()
                    .clone(),
            );
            left_strip.push(
                read_2d_vec(&front_face, &self.face_size, &x, &layer)
                    .unwrap()
                    .clone(),
            );
            back_strip.push(
                read_2d_vec(&back_face, &self.face_size, &x, &layer)
                    .unwrap()
                    .clone(),
            );
        }

        // Step 4: Swap the faces forwards if clockwise, backwards if counterclockwise
        let mut new_left: Vec<RubiksColor>;
        let mut new_back: Vec<RubiksColor>;
        let mut new_right: Vec<RubiksColor>;
        let mut new_front: Vec<RubiksColor>;

        match direction {
            TurnDirection::CLOCKWISE => {
                new_left = front_strip;
                new_back = left_strip;
                new_right = back_strip;
                new_front = right_strip;
            }
            TurnDirection::COUNTERCLOCKWISE => {
                new_left = back_strip;
                new_back = right_strip;
                new_right = front_strip;
                new_front = left_strip;
            }
        };
        // Step 5: Write new strips to new faces
        new_left.reverse();
        new_back.reverse();
        new_right.reverse();
        new_front.reverse();
        for x in (0..self.face_size).rev() {
            write_2d_vec(
                &mut front_face,
                &self.face_size,
                &x,
                &(self.face_size - 1 - layer),
                new_front.pop().unwrap(),
            );
            write_2d_vec(
                &mut left_face,
                &self.face_size,
                &x,
                &(self.face_size - 1 - layer),
                new_left.pop().unwrap(),
            );
            write_2d_vec(
                &mut back_face,
                &self.face_size,
                &x,
                &(self.face_size - 1 - layer),
                new_back.pop().unwrap(),
            );
            write_2d_vec(
                &mut right_face,
                &self.face_size,
                &x,
                &(self.face_size - 1 - layer),
                new_right.pop().unwrap(),
            );
        }

        self.faces.insert(RubiksColor::RIGHT, right_face);
        self.faces.insert(RubiksColor::FORWARD, front_face);
        self.faces.insert(RubiksColor::LEFT, left_face);
        self.faces.insert(RubiksColor::BACK, back_face);

        self
    }
    // Rotates the cube along the axis facing the right direction
    fn rotate_right(mut self, direction: &TurnDirection, layer: &u8) -> RubiksCube {
        // Step 1: Determine if the front face is being rotated, if so rotate it
        if *layer == 0 {
            let face = self
                .faces
                .get_mut(&RubiksColor::RIGHT)
                .unwrap() // We will never not have this be filled out so we can just unwrap here
                .to_owned();
            let new_face = match direction {
                TurnDirection::CLOCKWISE => rotate_vec_cw(&face, &self.face_size),
                TurnDirection::COUNTERCLOCKWISE => rotate_vec_ccw(&face, &self.face_size),
            };
            self.faces.insert(RubiksColor::RIGHT, new_face);
        }
        // Step 2: Determine if the back face is being rotated, if so rotate it
        if *layer == self.face_size - 1 {
            let face = self.faces.get_mut(&RubiksColor::LEFT).unwrap().to_owned();
            let new_face = match direction {
                TurnDirection::CLOCKWISE => rotate_vec_ccw(&face, &self.face_size),
                TurnDirection::COUNTERCLOCKWISE => rotate_vec_cw(&face, &self.face_size),
            };
            self.faces.insert(RubiksColor::LEFT, new_face);
        }
        // Step 3: Determine the strips allong the top, bottom, left, and right that are going to
        // be swapped
        // Step 3.1: Top, Bottom, and Front faces all itterate the same way here
        let mut top_strip: Vec<RubiksColor> = vec![];
        let mut top_face = self.faces.get_mut(&RubiksColor::UP).unwrap().to_owned();
        let mut bottom_strip: Vec<RubiksColor> = vec![];
        let mut bottom_face = self.faces.get_mut(&RubiksColor::DOWN).unwrap().to_owned();
        let mut front_strip: Vec<RubiksColor> = vec![];
        let mut front_face = self
            .faces
            .get_mut(&RubiksColor::FORWARD)
            .unwrap()
            .to_owned();

        for y in (0..self.face_size).rev() {
            top_strip.push(
                read_2d_vec(
                    &top_face,
                    &self.face_size,
                    &(self.face_size - 1 - layer),
                    &y,
                )
                .unwrap()
                .clone(),
            );
            bottom_strip.push(
                read_2d_vec(
                    &bottom_face,
                    &self.face_size,
                    &(self.face_size - 1 - layer),
                    &y,
                )
                .unwrap()
                .clone(),
            );
            bottom_strip.push(
                read_2d_vec(
                    &top_face,
                    &self.face_size,
                    &(self.face_size - 1 - layer),
                    &y,
                )
                .unwrap()
                .clone(),
            );
        }

        let mut back_strip: Vec<RubiksColor> = vec![];
        let mut back_face = self.faces.get_mut(&RubiksColor::BACK).unwrap().to_owned();

        for y in 0..self.face_size {
            back_strip.push(
                read_2d_vec(
                    &back_face,
                    &self.face_size,
                    &(self.face_size - 1 - layer),
                    &y,
                )
                .unwrap()
                .clone(),
            );
        }

        // Step 4: Swap the faces forwards if clockwise, backwards if counterclockwise
        let mut new_top: Vec<RubiksColor>;
        let mut new_back: Vec<RubiksColor>;
        let mut new_bottom: Vec<RubiksColor>;
        let mut new_front: Vec<RubiksColor>;

        match direction {
            TurnDirection::CLOCKWISE => {
                new_top = front_strip;
                new_back = top_strip;
                new_bottom = back_strip;
                new_front = bottom_strip;
            }
            TurnDirection::COUNTERCLOCKWISE => {
                new_top = back_strip;
                new_back = bottom_strip;
                new_bottom = front_strip;
                new_front = bottom_strip;
            }
        };
        // Step 5: Write new strips to new faces
        new_top.reverse();
        new_back.reverse();
        new_bottom.reverse();
        new_front.reverse();
        for y in (0..self.face_size).rev() {
            write_2d_vec(
                &mut top_face,
                &self.face_size,
                &(self.face_size - 1 - layer),
                &y,
                new_top.pop().unwrap(),
            );
            write_2d_vec(
                &mut bottom_face,
                &self.face_size,
                &(self.face_size - 1 - layer),
                &y,
                new_bottom.pop().unwrap(),
            );
            write_2d_vec(
                &mut front_face,
                &self.face_size,
                &(self.face_size - 1 - layer),
                &y,
                new_front.pop().unwrap(),
            );
        }

        for y in 0..self.face_size {
            write_2d_vec(
                &mut back_face,
                &self.face_size,
                &(self.face_size - 1 - layer),
                &y,
                new_back.pop().unwrap(),
            );
        }
        // Step 6: Place new faces back onto cube
        self.faces.insert(RubiksColor::UP, top_face);
        self.faces.insert(RubiksColor::FORWARD, front_face);
        self.faces.insert(RubiksColor::DOWN, bottom_face);
        self.faces.insert(RubiksColor::BACK, back_face);

        self
    }

    fn get_square(self, face: RubiksColor, x: u8, y: u8) -> Option<RubiksColor> {
        let t_face: &Vec<RubiksColor> = self.faces.get(&face)?;
        let val: Option<&RubiksColor> = t_face.get(usize::from(x + y * self.face_size));
        match val {
            Some(v) => Some(v.to_owned()),
            None => None,
        }
    }
}

fn main() {
    // Create a 3x3x3 cube, and turn the center layer facing the player upwards (ie, the second
    // layer when viewed from the right)
    let cube = RubiksCube::new(3).do_move(RubiksColor::RIGHT, 1, TurnDirection::CLOCKWISE);
}
