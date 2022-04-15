#! /usr/bin/python
import itertools
import math
import re
from dataclasses import dataclass, field
from time import time as ts
from typing import List, Optional, Dict
from vector import obj as vec


@dataclass
class Scanner:
    number: int
    position: Optional[vec]
    beacons: List[vec]
    rotation: Dict[str, float] = field(default_factory=lambda: {"x": 0, "y": 0, "z": 0}, init=False, compare=False)

    def rotate90(self, axis: str):
        angle = math.pi / 2
        fn_name = f"rotate{axis.upper()}"

        self.beacons = [getattr(b, fn_name)(angle) for b in self.beacons]

        for b in self.beacons:
            b.x = int(round(b.x, 7))
            b.y = int(round(b.y, 7))
            b.z = int(round(b.z, 7))

        self.rotation[axis] += angle
        if self.rotation[axis] > math.pi * 2:
            self.rotation[axis] -= math.pi * 2

    def rotated(self, axii: Dict[str, int]):
        beacons = self.beacons
        for axis, rotation in axii.items():
            angle = math.pi / 2 * rotation
            fn_name = f"rotate{axis.upper()}"

            beacons = [getattr(b, fn_name)(angle) for b in beacons]

        for b in beacons:
            b.x = int(round(b.x, 7))
            b.y = int(round(b.y, 7))
            b.z = int(round(b.z, 7))

        return beacons


def parse(input_):
    scanners = []
    scanner = None

    for line in map(lambda s: s.strip(), input_.split("\n")):
        if "scanner" in line:
            m = re.search(r"\d+", line)
            assert m is not None
            number = int(m.group())

            scanner = Scanner(number=number, position=vec(x=0, y=0, z=0) if number == 0 else None, beacons=[])
            scanners.append(scanner)
        elif line == "":
            scanner = None
        else:
            assert scanner is not None
            x, y, z = map(int, line.split(","))
            point = vec(x=x, y=y, z=z)
            scanner.beacons.append(point)

    return scanners


def match_beacons(beacons, s2):
    for x, y, z in itertools.product(range(4), range(4), range(4)):
        rotation = {"x": x, "y": y, "z": z}
        rotated = s2.rotated(rotation)
        for b1 in beacons:
            for b2 in rotated:
                translation = b1 - b2

                translated = [b + translation for b in rotated]

                overlaps = set((b.x, b.y, b.z) for b in beacons) & set((b.x, b.y, b.z) for b in translated)
                if len(overlaps) >= 12:
                    s2.position = translation
                    s2.rotation = rotation

                    return [vec(x=x, y=y, z=z) for x, y, z in set((b.x, b.y, b.z) for b in translated) - set((b.x, b.y, b.z) for b in beacons)]

    return []


def get_all_beacons(scanners):
    to_scan = scanners[:]
    ref = to_scan.pop()
    beacons = ref.beacons

    while to_scan:
        print(f"List contains {len(to_scan)}")
        for idx, scanner in enumerate(to_scan):
            new_beacons = match_beacons(beacons, scanner)
            if new_beacons:
                print(f"Removing {idx} from list")
                beacons += new_beacons
                del to_scan[idx]
                break
        else:
            raise Exception("FUCK")

    return beacons


if __name__ == '__main__':
    with open("input/day19.txt") as f:
        scanners = parse(f.read())

    t = ts()
    beacons = get_all_beacons(scanners)
    print("Took {ts() - t:.3f}s")

class Test:
    input_ = """--- scanner 0 ---
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
30,-46,-14"""

    def test_parse(self):
        expected = [
            Scanner(
                number=0,
                position=vec(x=0, y=0, z=0),
                beacons=[
                    vec(x=404, y=-588, z=-901),
                    vec(x=528, y=-643, z=409),
                    vec(x=-838, y=591, z=734),
                    vec(x=390, y=-675, z=-793),
                    vec(x=-537, y=-823, z=-458),
                    vec(x=-485, y=-357, z=347),
                    vec(x=-345, y=-311, z=381),
                    vec(x=-661, y=-816, z=-575),
                    vec(x=-876, y=649, z=763),
                    vec(x=-618, y=-824, z=-621),
                    vec(x=553, y=345, z=-567),
                    vec(x=474, y=580, z=667),
                    vec(x=-447, y=-329, z=318),
                    vec(x=-584, y=868, z=-557),
                    vec(x=544, y=-627, z=-890),
                    vec(x=564, y=392, z=-477),
                    vec(x=455, y=729, z=728),
                    vec(x=-892, y=524, z=684),
                    vec(x=-689, y=845, z=-530),
                    vec(x=423, y=-701, z=434),
                    vec(x=7, y=-33, z=-71),
                    vec(x=630, y=319, z=-379),
                    vec(x=443, y=580, z=662),
                    vec(x=-789, y=900, z=-551),
                    vec(x=459, y=-707, z=401),
                ],
            ),
            Scanner(
                number=1,
                position=None,
                beacons=[
                    vec(x=686, y=422, z=578),
                    vec(x=605, y=423, z=415),
                    vec(x=515, y=917, z=-361),
                    vec(x=-336, y=658, z=858),
                    vec(x=95, y=138, z=22),
                    vec(x=-476, y=619, z=847),
                    vec(x=-340, y=-569, z=-846),
                    vec(x=567, y=-361, z=727),
                    vec(x=-460, y=603, z=-452),
                    vec(x=669, y=-402, z=600),
                    vec(x=729, y=430, z=532),
                    vec(x=-500, y=-761, z=534),
                    vec(x=-322, y=571, z=750),
                    vec(x=-466, y=-666, z=-811),
                    vec(x=-429, y=-592, z=574),
                    vec(x=-355, y=545, z=-477),
                    vec(x=703, y=-491, z=-529),
                    vec(x=-328, y=-685, z=520),
                    vec(x=413, y=935, z=-424),
                    vec(x=-391, y=539, z=-444),
                    vec(x=586, y=-435, z=557),
                    vec(x=-364, y=-763, z=-893),
                    vec(x=807, y=-499, z=-711),
                    vec(x=755, y=-354, z=-619),
                    vec(x=553, y=889, z=-390),
                ],
            ),
            Scanner(
                number=2,
                position=None,
                beacons=[
                    vec(x=649, y=640, z=665),
                    vec(x=682, y=-795, z=504),
                    vec(x=-784, y=533, z=-524),
                    vec(x=-644, y=584, z=-595),
                    vec(x=-588, y=-843, z=648),
                    vec(x=-30, y=6, z=44),
                    vec(x=-674, y=560, z=763),
                    vec(x=500, y=723, z=-460),
                    vec(x=609, y=671, z=-379),
                    vec(x=-555, y=-800, z=653),
                    vec(x=-675, y=-892, z=-343),
                    vec(x=697, y=-426, z=-610),
                    vec(x=578, y=704, z=681),
                    vec(x=493, y=664, z=-388),
                    vec(x=-671, y=-858, z=530),
                    vec(x=-667, y=343, z=800),
                    vec(x=571, y=-461, z=-707),
                    vec(x=-138, y=-166, z=112),
                    vec(x=-889, y=563, z=-600),
                    vec(x=646, y=-828, z=498),
                    vec(x=640, y=759, z=510),
                    vec(x=-630, y=509, z=768),
                    vec(x=-681, y=-892, z=-333),
                    vec(x=673, y=-379, z=-804),
                    vec(x=-742, y=-814, z=-386),
                    vec(x=577, y=-820, z=562),
                ],
            ),
            Scanner(
                number=3,
                position=None,
                beacons=[
                    vec(x=-589, y=542, z=597),
                    vec(x=605, y=-692, z=669),
                    vec(x=-500, y=565, z=-823),
                    vec(x=-660, y=373, z=557),
                    vec(x=-458, y=-679, z=-417),
                    vec(x=-488, y=449, z=543),
                    vec(x=-626, y=468, z=-788),
                    vec(x=338, y=-750, z=-386),
                    vec(x=528, y=-832, z=-391),
                    vec(x=562, y=-778, z=733),
                    vec(x=-938, y=-730, z=414),
                    vec(x=543, y=643, z=-506),
                    vec(x=-524, y=371, z=-870),
                    vec(x=407, y=773, z=750),
                    vec(x=-104, y=29, z=83),
                    vec(x=378, y=-903, z=-323),
                    vec(x=-778, y=-728, z=485),
                    vec(x=426, y=699, z=580),
                    vec(x=-438, y=-605, z=-362),
                    vec(x=-469, y=-447, z=-387),
                    vec(x=509, y=732, z=623),
                    vec(x=647, y=635, z=-688),
                    vec(x=-868, y=-804, z=481),
                    vec(x=614, y=-800, z=639),
                    vec(x=595, y=780, z=-596),
                ],
            ),
            Scanner(
                number=4,
                position=None,
                beacons=[
                    vec(x=727, y=592, z=562),
                    vec(x=-293, y=-554, z=779),
                    vec(x=441, y=611, z=-461),
                    vec(x=-714, y=465, z=-776),
                    vec(x=-743, y=427, z=-804),
                    vec(x=-660, y=-479, z=-426),
                    vec(x=832, y=-632, z=460),
                    vec(x=927, y=-485, z=-438),
                    vec(x=408, y=393, z=-506),
                    vec(x=466, y=436, z=-512),
                    vec(x=110, y=16, z=151),
                    vec(x=-258, y=-428, z=682),
                    vec(x=-393, y=719, z=612),
                    vec(x=-211, y=-452, z=876),
                    vec(x=808, y=-476, z=-593),
                    vec(x=-575, y=615, z=604),
                    vec(x=-485, y=667, z=467),
                    vec(x=-680, y=325, z=-822),
                    vec(x=-627, y=-443, z=-432),
                    vec(x=872, y=-547, z=-609),
                    vec(x=833, y=512, z=582),
                    vec(x=807, y=604, z=487),
                    vec(x=839, y=-516, z=451),
                    vec(x=891, y=-625, z=532),
                    vec(x=-652, y=-548, z=-490),
                    vec(x=30, y=-46, z=-14),
                ],
            ),
        ]
        assert parse(self.input_) == expected

    def test_rotate_90(self):
        scanner = Scanner(
            number=0,
            position=vec(x=0, y=0, z=0),
            beacons=[
                vec(x=-1, y=-1, z=1),
                vec(x=-2, y=-2, z=2),
                vec(x=-3, y=-3, z=3),
                vec(x=-2, y=-3, z=1),
                vec(x=5, y=6, z=-4),
                vec(x=8, y=0, z=7),
            ],
        )

        expected = Scanner(
            number=0,
            position=vec(x=0, y=0, z=0),
            beacons=[
                vec(x=1, y=-1, z=1),
                vec(x=2, y=-2, z=2),
                vec(x=3, y=-3, z=3),
                vec(x=2, y=-1, z=3),
                vec(x=-5, y=4,  z=-6),
                vec(x=-8, y=-7, z=0),
            ],
        )

        scanner.rotate90("x")
        scanner.rotate90("y")
        scanner.rotate90("y")

        assert scanner == expected

    def test_rotated(self):
        scanner = Scanner(
            number=0,
            position=vec(x=0, y=0, z=0),
            beacons=[
                vec(x=-1, y=-1, z=1),
                vec(x=-2, y=-2, z=2),
                vec(x=-3, y=-3, z=3),
                vec(x=-2, y=-3, z=1),
                vec(x=5, y=6, z=-4),
                vec(x=8, y=0, z=7),
            ],
        )

        expected = [
                vec(x=1, y=-1, z=1),
                vec(x=2, y=-2, z=2),
                vec(x=3, y=-3, z=3),
                vec(x=2, y=-1, z=3),
                vec(x=-5, y=4,  z=-6),
                vec(x=-8, y=-7, z=0),
            ]

        assert scanner.rotated({"x": 1, "y": 2, "z": 0}) == expected

    def test_get_overlaps(self):
        scanners = parse(self.input_)
        
        s1 = scanners[0]
        s2 = scanners[1]

        overlaps = match_beacons(s1.beacons, s2)
        
        assert s2.position == vec(x=68,y=-1246,z=-43)

        assert overlaps == []

    def test_get_all_beacons(self):
        scanners = parse(self.input_)

        beacons = get_all_beacons(scanners)

        assert len(beacons) == 79
