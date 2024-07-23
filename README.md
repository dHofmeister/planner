# Planner

[![Master](https://github.com/dHofmeister/planner/actions/workflows/master.yml/badge.svg)](https://github.com/dHofmeister/planner/actions/workflows/master.yml)

## Documentation 

This project aims to solve a planning problem in which the goal is to navigate a grid and collect as many points as possible. Side goals are doing this in a computationally efficient manner and allowing a multi-agent system to traverse the grid. The project contains examples, tests and docs.

## Design

The path planning problem in its most basic form is a solved problem. Solutions exist in the form of Breadth First Search, Dijkstra, A* and D*. However, there are multiple sources of added complexity that make such solutions potentially less ideal or complicated. 1) Non-static grid. 2) Multi-agent system. 3) No clearly defined end-goal. The combination of 1 and 3 allow backtracking and "running in circles" to be perfectly valid solutions. Combine this with a multi-drone problem and solutions are not instantly obvious.

To handle this problem I decided to implement the most straight-forward algorithm that is intuitively easy so follow: Raycasting. Each drones shoots rays on the grid from its own location and counts how many points it hits. It selects the direction to go where it has hit the most points.

This solution is both simple and easy to tune / upgrade. Ray length, test direction, conic raycasting, falloff... are potential additions. The current implements 8 rays, and a configurable length.

The rest of the system follows the action <==> reaction pattern in the form of agent <==> simulator. 

## Usage

The command line interface implements the requirements as follows:
```
❯ cargo run -- -h
   Compiling planner v0.1.0 (/home/dev/repos/planner)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.91s
     Running `target/debug/planner -h`
A CLI tool for grid-based simulations

Usage: planner [OPTIONS]

Options:
  -n, --size <SIZE>                  Grid size [default: 5]
  -t, --time-steps <TIME_STEPS>      Discrete time steps [default: 32]
  -T, --max-duration <MAX_DURATION>  Max duration in ms [default: 100]
  -x, --pos-x <POS_X>                Starting positions x
  -y, --pos-y <POS_Y>                Starting positions y
  -g, --grid <GRID>                  Source grid [default: GRID_S]
  -h, --help                         Print help
  -V, --version                      Print version
```

Please note that using RUST_DEBUG=INFO or DEBUG will switch behavior from normal mode to the slower DEBUG mode. DEBUG has a baked in sleep of 100ms per iteration, for visualization purposes.

### Examples
For a quick single-drone simulation, run:
```
RUST_LOG=DEBUG cargo run --example solo
```
or for an example multi drone simulation
```
RUST_LOG=DEBUG cargo run --example multi
```

You can also run the above in RUST_DEBUG=INFO

### Configure yourselves
For more configuration freedom you can run, for example:
```
RUST_LOG=INFO cargo run -- -T 10 -t 16 -n 10 -x 10 -y 1 -x 5 -y 18
```
Which returns:
```
❯ RUST_LOG=INFO cargo run -- -T 10 -t 16 -n 10 -x 10 -y 1 -x 5 -y 18
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/planner -T 10 -t 16 -n 10 -x 10 -y 1 -x 5 -y 18`
[2024-07-23T14:38:08Z INFO  planner] starting up
[2024-07-23T14:38:08Z INFO  planner::run::main] Max time steps reached
[2024-07-23T14:38:08Z INFO  planner::utils::plotter] Paths on grid:

       0    0   20   10   10   20   10    0    0   10   20    0    0   20   10    0   10   10   20   10
      10    0   20   10   10    0   10   10   20   10   20    0   10   20   10   10    0   20   20    0
      20   20   20    0   10    0    0   20    0    0   10   20    0    0   10  [20] [20] [20] [20]   0
      20   20   20   10   20   10    0   10    0   20    0   10  [20] [20] [10] [ 0]  20   10  [20]  10
       0   10    0    0   20   10   10    0   20   10   10   20  [20] [20]  10    0    0   20  [10]  10
       0   10   10    0    0    0   20    0   10   20   10  [20] [20]   0   20    0    0   20 1X280   0
       0   20    0   10   20    0   10   10   10   20   20    0  [20]   0   10   10   10   20   10   20
      20    0   10   20    0   20   20    0    0   20    0   10   10   20   10    0   20    0   20   20
       0    0  [10] [20] [10] [20] [ 0] [20] [20]   0   20    0   10   10   20   10   20    0    0   20
      10  [10] [10]  20    0    0   10   10  [20] [20] [ 0] [20] [20] [20]  20   10   10   20    0   20
       0 0X220  10    0    0   10   10   10   20    0   20   20   10    0   10    0   20    0    0   20
      20    0    0    0    0   20    0    0    0   10   20    0   10    0   10    0    0   20    0   20
       0    0    0   10   10    0   10   10   10   20    0   20   20    0    0   10   10   20    0   20
      20    0    0    0   10   10    0    0    0   10    0   10   10   10   10   10   10   20   10   10
      20   10    0    0   10    0   20   10   20   20   20   10    0    0   10    0   10   10   10    0
      20   10    0   20   20   10   20   20   10   20   20   20   20   20    0   20   10    0   20    0
       0    0   10   20   20    0   10   10    0   10    0   20    0    0   20   10    0   10    0   10
       0   10   20   10   10   10   10   10   10   20    0   10    0   20   10    0    0    0   10    0
      20    0   10   20   20   20    0    0    0    0   20   10   20   10    0    0   10    0    0   10
       0   10   10   20    0    0   20   20    0   20    0   20   20   10   10    0   20   20   20    0

Drone 0:
  Path 0: (10,1), (9,1), (9,2), (8,2), (8,3), (8,4), (8,5), (8,6), (8,7), (8,8), (9,8), (9,9), (9,10),
          (9,11), (9,12), (9,13) (Total cost: 220)
Drone 1:
  Path 0: (5,18), (4,18), (3,18), (2,18), (2,17), (2,16), (2,15), (3,15), (3,14), (3,13), (4,13), (3,12),
          (4,12), (5,12), (5,11), (6,12) (Total cost: 280)
```

The displayed grid shows in [] where the drone will travel, the starting position in AXB form, where A=drone number, B=Score achieved in the run.

### Modules

There is a more lower level form of documentation available at:
```
cargo doc --open
```
Other code comments can be found in-lined.

## Tests

This project has the most basic tests implemented.
```
❯ cargo test
   Compiling planner v0.1.0 (/home/dev/repos/planner)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.63s
     Running unittests src/lib.rs (target/debug/deps/planner-16922e74ad9b37bb)

running 2 tests
test planners::ray_casting::tests::test_ray_casting_planner ... ok
test simulators::incremental::tests::test_incremental_solve ... ok
```
