use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::Display;
use std::time::Instant;

use aoc_2021::get_input;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    node: (usize, usize),
    cost: u8,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    position: (usize, usize),
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &[String]) -> Vec<Vec<u8>> {
    input
        .iter()
        .map(|r| {
            r.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect()
}

fn expand(tiles: &[Vec<u8>], factor: usize) -> Vec<Vec<u8>> {
    (0..factor * tiles.len())
        .map(|r| {
            (0..factor)
                .flat_map(|c| {
                    tiles[r % tiles.len()]
                        .iter()
                        .map(|&n| (n + (r / tiles.len()) as u8 + c as u8 - 1) % 9 + 1)
                        .collect::<Vec<u8>>()
                })
                .collect()
        })
        .collect()
}

fn make_graph(tiles: &[Vec<u8>]) -> Vec<Vec<Edge>> {
    let height = tiles.len();
    let width = tiles[0].len();

    let mut graph = Vec::with_capacity(height * width);

    for x in 0..height {
        for y in 0..width {
            let mut edges = Vec::new();
            if x > 0 {
                edges.push(Edge {
                    node: (x - 1, y),
                    cost: tiles[x - 1][y],
                });
            }
            if x < height - 1 {
                edges.push(Edge {
                    node: (x + 1, y),
                    cost: tiles[x + 1][y],
                });
            }
            if y > 0 {
                edges.push(Edge {
                    node: (x, y - 1),
                    cost: tiles[x][y - 1],
                });
            }
            if y < height - 1 {
                edges.push(Edge {
                    node: (x, y + 1),
                    cost: tiles[x][y + 1],
                });
            }

            graph.push(edges);
        }
    }

    graph
}

// Thank you https://doc.rust-lang.org/std/collections/binary_heap/index.html
fn get_cheapest_path(tiles: &[Vec<u8>]) -> Option<usize> {
    let height = tiles.len();
    let width = tiles[0].len();

    let graph = make_graph(tiles);

    let start = (0, 0);
    let goal = (height - 1, width - 1);

    let mut dist: Vec<_> = (0..graph.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    dist[start.0 * width + start.1] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        if cost > dist[position.0 * width + position.1] {
            continue;
        }

        for edge in &graph[position.0 * width + position.1] {
            let next = State {
                cost: cost + edge.cost as usize,
                position: edge.node,
            };

            if next.cost < dist[next.position.0 * width + next.position.1] {
                heap.push(next);
                dist[next.position.0 * width + next.position.1] = next.cost;
            }
        }
    }

    None
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let tiles = parse_input(input);

    let p1 = get_cheapest_path(&tiles).unwrap();
    let p2 = get_cheapest_path(&expand(&tiles, 5)).unwrap();

    (p1, p2)
}

fn main() {
    let input = get_input("day15.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

#[cfg(test)]
mod tests {
    use crate::{expand, get_cheapest_path, parse_input};

    static TEST_INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test_parse_input() {
        let input = TEST_INPUT
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let expected = vec![
            vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            vec![1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            vec![2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            vec![3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            vec![7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            vec![1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            vec![1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            vec![3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            vec![1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            vec![2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ];
        assert_eq!(parse_input(&input), expected);
    }

    #[test]
    fn test_part_1() {
        let input = TEST_INPUT
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let tiles = parse_input(&input);

        assert_eq!(get_cheapest_path(&tiles), Some(40));
    }

    #[test]
    fn test_part_2() {
        let input = TEST_INPUT
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let tiles = parse_input(&input);

        assert_eq!(get_cheapest_path(&expand(&tiles, 5)), Some(315));
    }

    #[test]
    fn test_expand() {
        let input = TEST_INPUT
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let input = parse_input(&input);

        let expected = parse_input(&vec![
            "11637517422274862853338597396444961841755517295286".to_string(),
            "13813736722492484783351359589446246169155735727126".to_string(),
            "21365113283247622439435873354154698446526571955763".to_string(),
            "36949315694715142671582625378269373648937148475914".to_string(),
            "74634171118574528222968563933317967414442817852555".to_string(),
            "13191281372421239248353234135946434524615754563572".to_string(),
            "13599124212461123532357223464346833457545794456865".to_string(),
            "31254216394236532741534764385264587549637569865174".to_string(),
            "12931385212314249632342535174345364628545647573965".to_string(),
            "23119445813422155692453326671356443778246755488935".to_string(),
            "22748628533385973964449618417555172952866628316397".to_string(),
            "24924847833513595894462461691557357271266846838237".to_string(),
            "32476224394358733541546984465265719557637682166874".to_string(),
            "47151426715826253782693736489371484759148259586125".to_string(),
            "85745282229685639333179674144428178525553928963666".to_string(),
            "24212392483532341359464345246157545635726865674683".to_string(),
            "24611235323572234643468334575457944568656815567976".to_string(),
            "42365327415347643852645875496375698651748671976285".to_string(),
            "23142496323425351743453646285456475739656758684176".to_string(),
            "34221556924533266713564437782467554889357866599146".to_string(),
            "33859739644496184175551729528666283163977739427418".to_string(),
            "35135958944624616915573572712668468382377957949348".to_string(),
            "43587335415469844652657195576376821668748793277985".to_string(),
            "58262537826937364893714847591482595861259361697236".to_string(),
            "96856393331796741444281785255539289636664139174777".to_string(),
            "35323413594643452461575456357268656746837976785794".to_string(),
            "35722346434683345754579445686568155679767926678187".to_string(),
            "53476438526458754963756986517486719762859782187396".to_string(),
            "34253517434536462854564757396567586841767869795287".to_string(),
            "45332667135644377824675548893578665991468977611257".to_string(),
            "44961841755517295286662831639777394274188841538529".to_string(),
            "46246169155735727126684683823779579493488168151459".to_string(),
            "54698446526571955763768216687487932779859814388196".to_string(),
            "69373648937148475914825958612593616972361472718347".to_string(),
            "17967414442817852555392896366641391747775241285888".to_string(),
            "46434524615754563572686567468379767857948187896815".to_string(),
            "46833457545794456865681556797679266781878137789298".to_string(),
            "64587549637569865174867197628597821873961893298417".to_string(),
            "45364628545647573965675868417678697952878971816398".to_string(),
            "56443778246755488935786659914689776112579188722368".to_string(),
            "55172952866628316397773942741888415385299952649631".to_string(),
            "57357271266846838237795794934881681514599279262561".to_string(),
            "65719557637682166874879327798598143881961925499217".to_string(),
            "71484759148259586125936169723614727183472583829458".to_string(),
            "28178525553928963666413917477752412858886352396999".to_string(),
            "57545635726865674683797678579481878968159298917926".to_string(),
            "57944568656815567976792667818781377892989248891319".to_string(),
            "75698651748671976285978218739618932984172914319528".to_string(),
            "56475739656758684176786979528789718163989182927419".to_string(),
            "67554889357866599146897761125791887223681299833479".to_string(),
        ]);

        assert_eq!(expand(&input, 5), expected);
    }
}
