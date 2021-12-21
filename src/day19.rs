use crate::prelude::*;
use ahash::AHashSet;
use std::{fmt::Display, ops::Mul};

day!(19, parse => pt1, pt2);

type Int = i32;
type Vec3 = framework::vec::Vec3<Int>;
type ScanData = Vec<Vec3>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Matrix([i8; 9]);

impl Matrix {
    pub const fn mul_mat(self, Matrix(rhs): Matrix) -> Matrix {
        let Matrix(lhs) = self;
        Matrix([
            // Col 1
            lhs[0] * rhs[0] + lhs[3] * rhs[1] + lhs[6] * rhs[2],
            lhs[1] * rhs[0] + lhs[4] * rhs[1] + lhs[7] * rhs[2],
            lhs[2] * rhs[0] + lhs[5] * rhs[1] + lhs[8] * rhs[2],
            // Col 2
            lhs[0] * rhs[3] + lhs[3] * rhs[4] + lhs[6] * rhs[5],
            lhs[1] * rhs[3] + lhs[4] * rhs[4] + lhs[7] * rhs[5],
            lhs[2] * rhs[3] + lhs[5] * rhs[4] + lhs[8] * rhs[5],
            // Col 3
            lhs[0] * rhs[6] + lhs[3] * rhs[7] + lhs[6] * rhs[8],
            lhs[1] * rhs[6] + lhs[4] * rhs[7] + lhs[7] * rhs[8],
            lhs[2] * rhs[6] + lhs[5] * rhs[7] + lhs[8] * rhs[8],
        ])
    }

    pub const fn mul_vec(self, rhs: Vec3) -> Vec3 {
        let Matrix(lhs) = self;
        Vec3 {
            x: lhs[0] as Int * rhs.x + lhs[3] as Int * rhs.y + lhs[6] as Int * rhs.z,
            y: lhs[1] as Int * rhs.x + lhs[4] as Int * rhs.y + lhs[7] as Int * rhs.z,
            z: lhs[2] as Int * rhs.x + lhs[5] as Int * rhs.y + lhs[8] as Int * rhs.z,
        }
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..3 {
            write!(f, "[ ")?;
            for j in 0..3 {
                if j != 0 {
                    write!(f, ", ")?;
                }
                f.write_str(match self.0[i * 3 + j] {
                    -1 => "-1",
                    0 => " 0",
                    1 => " 1",
                    _ => unreachable!(),
                })?;
            }
            write!(f, " ]")?;
            if i != 3 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Matrix {}
impl Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Self::Output {
        self.mul_mat(rhs)
    }
}

impl Mul<Vec3> for Matrix {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.mul_vec(rhs)
    }
}

const fn compute_all_transforms() -> [Matrix; 24] {
    const IDENTITY: Matrix = Matrix([1, 0, 0, 0, 1, 0, 0, 0, 1]);
    const ROT_90_X: Matrix = Matrix([1, 0, 0, 0, 0, 1, 0, -1, 0]);
    const ROT_90_Y: Matrix = Matrix([0, 0, -1, 0, 1, 0, 1, 0, 0]);
    const ROT_90_Z: Matrix = Matrix([0, 1, 0, -1, 0, 0, 0, 0, 1]);
    const ROTATIONS: [Matrix; 4] = [
        IDENTITY,
        ROT_90_X,
        ROT_90_X.mul_mat(ROT_90_X),
        ROT_90_X.mul_mat(ROT_90_X).mul_mat(ROT_90_X),
    ];
    const ORIENTATIONS: [Matrix; 6] = [
        IDENTITY,
        ROT_90_Y,
        ROT_90_Y.mul_mat(ROT_90_Y),
        ROT_90_Y.mul_mat(ROT_90_Y).mul_mat(ROT_90_Y),
        ROT_90_Z,
        ROT_90_Z.mul_mat(ROT_90_Z).mul_mat(ROT_90_Z),
    ];

    let mut matrices = [Matrix([0; 9]); 24];
    let mut i = 0;
    while i < 6 {
        let mut j = 0;
        while j < 4 {
            matrices[i * 4 + j] = ORIENTATIONS[i].mul_mat(ROTATIONS[j]);
            j += 1;
        }
        i += 1;
    }
    matrices
}

const ALL_TRANSFORMS: [Matrix; 24] = compute_all_transforms();

// So, I expected this to be faster than actually doing the matrix multiplications
// but after measuring, this changed the runtime from approx 800ms to 930ms.
// const TRANSFORMATIONS: [fn(Vec3) -> Vec3; 24] = #[rustfmt::skip] [
//     |v| Vec3 { x:  v.x, y:  v.y, z:  v.z },
//     |v| Vec3 { x:  v.x, y:  v.z, z: -v.y },
//     |v| Vec3 { x:  v.x, y: -v.y, z: -v.z },
//     |v| Vec3 { x:  v.x, y: -v.z, z:  v.y },
//     |v| Vec3 { x:  v.y, y:  v.x, z: -v.z },
//     |v| Vec3 { x:  v.y, y:  v.z, z:  v.x },
//     |v| Vec3 { x:  v.y, y: -v.x, z:  v.z },
//     |v| Vec3 { x:  v.y, y: -v.z, z: -v.x },
//     |v| Vec3 { x:  v.z, y:  v.x, z:  v.y },
//     |v| Vec3 { x:  v.z, y:  v.y, z: -v.x },
//     |v| Vec3 { x:  v.z, y: -v.x, z: -v.y },
//     |v| Vec3 { x:  v.z, y: -v.y, z:  v.x },
//     |v| Vec3 { x: -v.x, y:  v.y, z: -v.z },
//     |v| Vec3 { x: -v.x, y:  v.z, z:  v.y },
//     |v| Vec3 { x: -v.x, y: -v.y, z:  v.z },
//     |v| Vec3 { x: -v.x, y: -v.z, z: -v.y },
//     |v| Vec3 { x: -v.y, y:  v.x, z:  v.z },
//     |v| Vec3 { x: -v.y, y:  v.z, z: -v.x },
//     |v| Vec3 { x: -v.y, y: -v.x, z: -v.z },
//     |v| Vec3 { x: -v.y, y: -v.z, z:  v.x },
//     |v| Vec3 { x: -v.z, y:  v.x, z: -v.y },
//     |v| Vec3 { x: -v.z, y:  v.y, z:  v.x },
//     |v| Vec3 { x: -v.z, y: -v.x, z:  v.y },
//     |v| Vec3 { x: -v.z, y: -v.y, z: -v.x },
// ];

fn determine_beacon_position(
    points: &AHashSet<Vec3>,
    scan_data: &ScanData,
    buffer: &mut Vec<Vec3>,
) -> Option<Vec3> {
    for transformation in &ALL_TRANSFORMS {
        buffer.clear();
        for point in scan_data {
            buffer.push(*transformation * *point);
        }
        buffer.push(Vec3::default());

        for &known_point in points.iter() {
            for i in 0..buffer.len() {
                // Anchor a particular point to "match" the known point
                let offset = known_point - buffer[i];
                for item in buffer.iter_mut() {
                    *item += offset;
                }

                // Count the overlaps
                let mut count = 0;
                for item in buffer.iter() {
                    if points.contains(item) {
                        count += 1;
                        if count >= 12 {
                            return buffer.pop();
                        }
                    }
                }
            }
        }
    }
    None
}

fn pt1(input: &[ScanData]) -> usize {
    assert!(input.len() <= 64);
    let mut all_points = AHashSet::<Vec3>::from_iter(input[0].iter().cloned());
    let goal_mask = (1u64 << (input.len() - 1)) - 1;
    let mut visited_mask = 0;
    let mut buffer = Vec::new();
    while goal_mask != visited_mask {
        for (index, scan_data) in input
            .iter()
            .enumerate()
            .skip(visited_mask.trailing_ones() as usize + 1)
        {
            let current_mask = 1 << (index - 1);
            if (visited_mask & current_mask) == current_mask {
                continue;
            }
            if determine_beacon_position(&all_points, scan_data, &mut buffer).is_some() {
                all_points.extend(buffer.drain(..));
                visited_mask |= current_mask;
            }
        }
    }

    all_points.len()
}

fn pt2(input: &[ScanData]) -> Int {
    let mut all_points = AHashSet::<Vec3>::from_iter(input[0].iter().cloned());
    let mut positions = vec![None; input.len()];
    positions[0] = Some(Vec3::default());
    let mut scanners_left = input.len() - 1;
    let mut buffer = Vec::new();
    while scanners_left != 0 {
        for (index, scan_data) in input.iter().enumerate() {
            if positions[index].is_some() {
                continue;
            }
            if let Some(position) = determine_beacon_position(&all_points, scan_data, &mut buffer) {
                all_points.extend(buffer.drain(..));
                positions[index] = Some(position);
                scanners_left -= 1;
            }
        }
    }

    let positions = positions.as_slice();
    (0..positions.len())
        .flat_map(|i| {
            (0..positions.len())
                .filter(move |&j| i != j)
                .map(move |j| positions[i].unwrap().manhathan_dist(positions[j].unwrap()))
        })
        .max()
        .unwrap()
}

fn parse(input: &[u8]) -> ParseResult<Vec<ScanData>> {
    use parsers::*;
    let header = token(b"--- scanner ")
        .then(number::<usize>())
        .trailed(token(b" ---\n"));
    let coord = number::<i32>()
        .trailed(token(b','))
        .and(number::<i32>())
        .trailed(token(b','))
        .and(number::<i32>())
        .map(|((x, y), z)| Vec3 { x, y, z });
    let coords = coord.sep_by(token(b'\n'));

    let scan_data = header.then(coords).sep_by(token(b"\n\n"));
    scan_data.parse(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

    simple_tests!(parse, pt1, pt1_tests, EXAMPLE => 79);
    simple_tests!(parse, pt2, pt2_tests, EXAMPLE => 3621);
}
