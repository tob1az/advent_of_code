use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Report {
    pub beacons: Vec<Beacon>,
}

pub type ThreeD = [i32; 3];

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Beacon {
    pub coordinates: ThreeD,
}

#[derive(Debug, Clone)]
pub struct BeaconPair<'a> {
    pub offsets: ThreeD,
    pub beacons: (&'a Beacon, &'a Beacon),
}

pub type AxisIndices = [usize; 3];
pub type AxisInversions = [bool; 3];
pub type Offset = ThreeD;

/// 90*n degree rotation around the 3 axes
#[derive(Debug, Clone)]
pub struct Rotation {
    axis_indices: AxisIndices,
    axis_inversions: AxisInversions,
}

#[derive(Debug, Clone)]
pub struct Transformation {
    pub rotation: Rotation,
    pub offset: Offset,
}

#[derive(Clone)]
pub struct Mapping {
    pub from_index: usize,
    pub to_index: usize,
    pub transformations: Vec<Transformation>,
}

impl From<&[i32]> for Beacon {
    fn from(coordinates: &[i32]) -> Self {
        debug_assert!(coordinates.len() == 3);
        Self {
            coordinates: [coordinates[0], coordinates[1], coordinates[2]],
        }
    }
}

impl Report {
    pub fn new(coordinates: &[i32]) -> Self {
        debug_assert!(coordinates.len() % 3 == 0);
        Self {
            beacons: coordinates.chunks(3).map(Beacon::from).collect(),
        }
    }

    pub fn beacon_pairs(&self) -> Vec<BeaconPair<'_>> {
        self.beacons
            .iter()
            .combinations(2)
            .map(|pair| BeaconPair::new(pair[0], pair[1]))
            .collect()
    }

    pub fn transformed(&self, transformations: &[Transformation]) -> Self {
        Self {
            beacons: self
                .beacons
                .iter()
                .map(|b| {
                    let mut transformed_beacon = b.clone();
                    for t in transformations.iter() {
                        transformed_beacon = transformed_beacon
                            .reoriented(&t.rotation)
                            .transposed(&t.offset);
                    }
                    transformed_beacon
                })
                .collect_vec(),
        }
    }
}

fn same_absolute_offsets(a: &Offset, b: &Offset) -> bool {
    a[0].abs() == b[0].abs() && a[1].abs() == b[1].abs() && a[2].abs() == b[2].abs()
}

fn extract_inversions(a: &Offset, b: &Offset) -> AxisInversions {
    [a[0] != b[0], a[1] != b[1], a[2] != b[2]]
}

pub fn calculate_offsets(a: &Beacon, b: &Beacon) -> ThreeD {
    [
        b.coordinates[0] - a.coordinates[0],
        b.coordinates[1] - a.coordinates[1],
        b.coordinates[2] - a.coordinates[2],
    ]
}

impl<'a> BeaconPair<'a> {
    fn new(a: &'a Beacon, b: &'a Beacon) -> Self {
        Self {
            offsets: calculate_offsets(a, b),
            beacons: (a, b),
        }
    }

    pub fn find_possible_transformations(
        &self,
        target_pair: &BeaconPair<'_>,
    ) -> Vec<Transformation> {
        self.offsets
            .iter()
            .cloned()
            .enumerate()
            .permutations(self.offsets.len())
            .filter_map(|c| {
                // verify that an offset permutation matches to the target -
                // that might be a sign that it is a transformed pair of same beacon coordinates
                let (indices, offsets): (Vec<_>, Vec<_>) = c.into_iter().unzip();
                let offsets: ThreeD = offsets.try_into().unwrap();
                let axis_indices = indices.try_into().unwrap();
                if !same_absolute_offsets(&offsets, &target_pair.offsets) {
                    return None;
                }
                let rotation = Rotation {
                    axis_indices,
                    axis_inversions: extract_inversions(&offsets, &target_pair.offsets),
                };
                // make sure the rotation is valid: after rotation, the beacon pair offsets from
                // the target should be the same
                let rotated_beacon0 = self.beacons.0.reoriented(&rotation);
                let rotated_beacon1 = self.beacons.1.reoriented(&rotation);
                let offset0 = calculate_offsets(target_pair.beacons.0, &rotated_beacon0);
                let offset1 = calculate_offsets(target_pair.beacons.1, &rotated_beacon1);
                if offset0 == offset1 {
                    Some(Transformation {
                        rotation,
                        offset: offset0,
                    })
                } else {
                    None
                }
            })
            .collect_vec()
    }
}

impl Rotation {
    pub fn apply(&self, coordinates: &ThreeD) -> ThreeD {
        [
            self.rotate_coordinate(0, coordinates),
            self.rotate_coordinate(1, coordinates),
            self.rotate_coordinate(2, coordinates),
        ]
    }

    fn rotate_coordinate(&self, index: usize, coordinates: &ThreeD) -> i32 {
        if self.axis_inversions[index] {
            -coordinates[self.axis_indices[index]]
        } else {
            coordinates[self.axis_indices[index]]
        }
    }
}

impl Beacon {
    pub fn reoriented(&self, rotation: &Rotation) -> Self {
        Self {
            coordinates: rotation.apply(&self.coordinates),
        }
    }

    pub fn transposed(self, offsets: &ThreeD) -> Self {
        Self {
            coordinates: [
                self.coordinates[0] - offsets[0],
                self.coordinates[1] - offsets[1],
                self.coordinates[2] - offsets[2],
            ],
        }
    }
}

fn invert_indices(indices: &AxisIndices) -> AxisIndices {
    match indices {
        [1, 2, 0] => [2, 0, 1],
        [2, 0, 1] => [1, 2, 0],
        _ => *indices,
    }
}

impl Transformation {
    pub fn inverted(&self) -> Self {
        let inverted_indices = invert_indices(&self.rotation.axis_indices);
        let rotation = Rotation {
            axis_inversions: self.reorder_inversions(&inverted_indices),
            axis_indices: inverted_indices,
        };
        Self {
            offset: rotation.apply(&[-self.offset[0], -self.offset[1], -self.offset[2]]),
            rotation,
        }
    }

    fn reorder_inversions(&self, indices: &AxisIndices) -> AxisInversions {
        let current = &self.rotation.axis_inversions;
        [
            current[indices[0]],
            current[indices[1]],
            current[indices[2]],
        ]
    }
}

impl Mapping {
    pub fn inverted(&self) -> Self {
        Self {
            from_index: self.to_index,
            to_index: self.from_index,
            transformations: self
                .transformations
                .iter()
                .cloned()
                .rev()
                .map(|t| t.inverted())
                .collect(),
        }
    }
}
