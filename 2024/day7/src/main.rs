fn evaluates_to_testval(testval: usize, current: usize, nextptr: usize, numbers: &[usize]) -> bool {
    if nextptr < numbers.len() {
        let next = numbers[nextptr];
        return evaluates_to_testval(testval, current + next, nextptr + 1, numbers)
            || evaluates_to_testval(testval, current * next, nextptr + 1, numbers);
    }
    current == testval
}

fn evaluates_to_testval_with_concatenation(
    testval: usize,
    current: usize,
    nextptr: usize,
    numbers: &[usize],
) -> bool {
    if nextptr < numbers.len() {
        let next = numbers[nextptr];
        let concatenated = [current.to_string(), next.to_string()]
            .join("")
            .parse::<usize>()
            .unwrap();

        return evaluates_to_testval_with_concatenation(
            testval,
            current + next,
            nextptr + 1,
            numbers,
        ) || evaluates_to_testval_with_concatenation(
            testval,
            current * next,
            nextptr + 1,
            numbers,
        ) || evaluates_to_testval_with_concatenation(
            testval,
            concatenated,
            nextptr + 1,
            numbers,
        );
    }
    current == testval
}

fn main() {
    let data = include_str!("../input").lines().map(|line| {
        let data = line.split(':').collect::<Vec<&str>>();
        let numbers = data[1]
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let testval = data[0].parse::<usize>().unwrap();
        (testval, numbers)
    });

    let part_1 = data
        .clone()
        .filter(|(testval, numbers)| evaluates_to_testval(*testval, numbers[0], 1, numbers))
        .map(|(testval, _)| testval)
        .sum::<usize>();
    println!("{part_1}");

    let part_2 = data
        .filter(|(testval, numbers)| {
            evaluates_to_testval_with_concatenation(*testval, numbers[0], 1, numbers)
        })
        .map(|(testval, _)| testval)
        .sum::<usize>();
    println!("{part_2}");
}
