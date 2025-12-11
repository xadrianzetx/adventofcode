use std::collections::HashMap;

fn build_graph(data: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph = HashMap::new();

    for line in data.lines() {
        let mut parts = line.split(':');
        let src = parts.next().unwrap();

        let destinations = parts
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();

        graph.insert(src, destinations);
    }

    graph
}

fn traverse(graph: &HashMap<&str, Vec<&str>>, src: &str, dest: &str) -> usize {
    let mut m = HashMap::new();
    step(src, dest, graph, &mut m)
}

fn step<'a>(
    current: &'a str,
    dest: &str,
    graph: &'a HashMap<&str, Vec<&str>>,
    m: &mut HashMap<&'a str, usize>,
) -> usize {
    if current == dest {
        return 1;
    }

    if let Some(known) = m.get(current) {
        return *known;
    }

    let mut paths = 0;
    if let Some(destinations) = graph.get(current) {
        for next in destinations {
            paths += step(next, dest, graph, m);
        }
    }

    m.insert(current, paths);

    paths
}

fn main() {
    let data = include_str!("input");

    let graph = build_graph(data);
    let part_1 = traverse(&graph, "you", "out");

    println!("Part 2: {part_1}");

    let fft_to_dac = traverse(&graph, "svr", "fft")
        * traverse(&graph, "fft", "dac")
        * traverse(&graph, "dac", "out");

    let dac_to_fft = traverse(&graph, "svr", "dac")
        * traverse(&graph, "dac", "fft")
        * traverse(&graph, "fft", "out");

    println!("Part 2: {}", fft_to_dac + dac_to_fft);
}
