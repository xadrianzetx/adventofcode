from functools import lru_cache


class Blueprint:
    def __init__(self, id, ore, clay, obsidian, geode) -> None:
        self.id = id
        self.ore_robot = ore
        self.clay_robot = clay
        self.obsidian_robot = obsidian
        self.geode_robot = geode


def parse(blueprint: str) -> Blueprint:
    id = int(blueprint.split(":")[0].split()[-1])
    res = blueprint.split(".")
    ore = int(res[0].split()[-2])
    clay = int(res[1].split()[-2])
    obs = (int(res[2].split()[-5]), int(res[2].split()[-2]))
    geo = (int(res[3].split()[-5]), int(res[3].split()[-2]))
    return Blueprint(id, ore, clay, obs, geo)


def factory(blueprint: Blueprint):
    # State tuple is (ore, clay, obsidian, geode).
    @lru_cache(maxsize=None)
    def simulate(time, resources, robots):
        if time == 0:
            return resources[-1]

        ore, clay, obs, geode = tuple(res + rob for res, rob in zip(resources, robots))
        orr, cr, obr, gr = robots

        # In order to keep state space manageable, we need to make some assumptions.
        # * Assume robot importance (from most important): geode -> obsidian -> clay | ore.
        # * If we can afford geode or obsidian robots, go for it and don't test other robots or noop paths.
        # * Avoid building clay robots if there's less than 5 minites until the end.
        # * Avoid building ore robots if there's less than 15 minites until the end.
        # There are probably better optimizations to find and this code could avoid being total garbage,
        # but at the moment I dont really care ¯\_(ツ)_/¯.
        if (
            resources[0] >= blueprint.geode_robot[0]
            and resources[2] >= blueprint.geode_robot[1]
        ):
            return simulate(
                time - 1,
                (
                    ore - blueprint.geode_robot[0],
                    clay,
                    obs - blueprint.geode_robot[1],
                    geode,
                ),
                (orr, cr, obr, gr + 1),
            )

        elif (
            resources[0] >= blueprint.obsidian_robot[0]
            and resources[1] >= blueprint.obsidian_robot[1]
        ):
            return simulate(
                time - 1,
                (
                    ore - blueprint.obsidian_robot[0],
                    clay - blueprint.obsidian_robot[1],
                    obs,
                    geode,
                ),
                (orr, cr, obr + 1, gr),
            )

        else:
            a, b = 0, 0
            if time > 5:
                if resources[0] >= blueprint.clay_robot:
                    b = simulate(
                        time - 1,
                        (ore - blueprint.clay_robot, clay, obs, geode),
                        (orr, cr + 1, obr, gr),
                    )
            if time > 15:
                if resources[0] >= blueprint.ore_robot:
                    a = simulate(
                        time - 1,
                        (ore - blueprint.ore_robot, clay, obs, geode),
                        (orr + 1, cr, obr, gr),
                    )
            noop = simulate(time - 1, (ore, clay, obs, geode), robots)
            return max(noop, a, b)

    return simulate


if __name__ == "__main__":
    part1 = 0
    part2 = 1
    with open("input") as file:
        blueprints = [parse(line) for line in file]

    # Takes about 30 seconds with pypy.
    for id, blueprint in enumerate(blueprints):
        print(f"Processing blueprint {id + 1}...", end="", flush=True)
        sim = factory(blueprint)
        partialp1 = sim(24, (0, 0, 0, 0), (1, 0, 0, 0))
        part1 += blueprint.id * partialp1
        if blueprint.id <= 3:
            partialp2 = sim(32, (0, 0, 0, 0), (1, 0, 0, 0))
            part2 *= partialp2

        print("done.")
    print(f"Part1: {part1}")
    print(f"Part2: {part2}")
