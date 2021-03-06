extern crate byteorder;
extern crate clap;
extern crate crc;
extern crate csv;
extern crate flate2;
extern crate num;
extern crate pid_control;
extern crate protobuf;
extern crate rand;
extern crate serde_yaml;
extern crate time;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate slugify;
extern crate specs;
#[macro_use]
extern crate specs_derive;
extern crate core;
extern crate nalgebra;

pub mod base;
pub mod dubins;
pub mod simulation;
pub mod simulation_2d;
pub mod tasks;
pub mod tf_record;
pub mod trajectory;

use base::*;
use clap::{App, Arg, SubCommand};
use num::Zero;
use rand::thread_rng;
use simulation::{Simulation, SimulationResult};
use std::f64::consts::PI;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use tf_record::ResultsWriter;

fn main() {
    pretty_env_logger::init();

    let matches = App::new("datagen")
        .version("0.1")
        .author("Matt Jadczak <mnj24@cam.ac.uk>")
        .about("Generates trajectory data for simulated robots")
        .subcommand(SubCommand::with_name("test").about("Tests simple trajectory evaluation"))
        .subcommand(
            SubCommand::with_name("gen-traj")
                .about("Generates reference trajectories")
                .arg(
                    Arg::with_name("length")
                        .short("l")
                        .long("length")
                        .takes_value(true)
                        .help("Sets the (minimum) length of the generated trajectory, in seconds"),
                )
                .arg(
                    Arg::with_name("variability")
                        .short("v")
                        .long("variability")
                        .takes_value(true)
                        .help("Sets variability of the trajectory"),
                )
                .arg(
                    Arg::with_name("rsd")
                        .short("r")
                        .long("rsd")
                        .takes_value(true)
                        .help("Sets rsd of the trajectory"),
                )
                .arg(
                    Arg::with_name("num")
                        .short("n")
                        .long("num-trajectories")
                        .takes_value(true)
                        .help("How many trajectories to generate")
                        .required(true),
                )
                .arg(
                    Arg::with_name("output_dir")
                        .short("o")
                        .long("output-dir")
                        .takes_value(true)
                        .help("Output directory to place generated trajectories in")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("gen-traj-2d")
                .about("Generates 2D reference trajectories")
                .arg(
                    Arg::with_name("length")
                        .short("l")
                        .long("length")
                        .takes_value(true)
                        .help("Sets the (minimum) length of the generated trajectory, in seconds"),
                )
                .arg(
                    Arg::with_name("variability")
                        .short("v")
                        .long("variability")
                        .takes_value(true)
                        .help("Sets variability of the trajectory"),
                )
                .arg(
                    Arg::with_name("rsd")
                        .short("r")
                        .long("rsd")
                        .takes_value(true)
                        .help("Sets rsd of the trajectory"),
                )
                .arg(
                    Arg::with_name("turnability")
                        .short("t")
                        .long("turnability")
                        .takes_value(true)
                        .help("Sets turnability of the trajectory"),
                )
                .arg(
                    Arg::with_name("num")
                        .short("n")
                        .long("num-trajectories")
                        .takes_value(true)
                        .help("How many trajectories to generate")
                        .required(true),
                )
                .arg(
                    Arg::with_name("output_dir")
                        .short("o")
                        .long("output-dir")
                        .takes_value(true)
                        .help("Output directory to place generated trajectories in")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("gen-data")
                .about("Generates trajectory data")
                .arg(
                    Arg::with_name("length")
                        .short("l")
                        .long("length")
                        .takes_value(true)
                        .help("Sets the (minimum) length of the generated trajectory, in seconds"),
                )
                .arg(
                    Arg::with_name("variability")
                        .short("v")
                        .long("variability")
                        .takes_value(true)
                        .help("Sets variability of the trajectory"),
                )
                .arg(
                    Arg::with_name("rsd")
                        .short("r")
                        .long("rsd")
                        .takes_value(true)
                        .help("Sets rsd of the trajectory"),
                )
                .arg(
                    Arg::with_name("num")
                        .short("n")
                        .long("num-trajectories")
                        .takes_value(true)
                        .help("How many trajectories to generate")
                        .required(true),
                )
                .arg(
                    Arg::with_name("output_dir")
                        .short("o")
                        .long("output-dir")
                        .takes_value(true)
                        .help("Output directory to place generated trajectories in")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("gen-dubins")
                .about("Generates Dubins trajectory data")
                .arg(
                    Arg::with_name("num")
                        .short("n")
                        .long("num-trajectories")
                        .takes_value(true)
                        .help("How many trajectories to generate")
                        .required(true),
                )
                .arg(
                    Arg::with_name("length")
                        .short("l")
                        .long("length")
                        .takes_value(true)
                        .help("Sets the (minimum) length of the generated trajectory, in seconds"),
                )
                .arg(
                    Arg::with_name("arena_size")
                        .short("a")
                        .long("arena-size")
                        .takes_value(true)
                        .help("Sets size of the arena")
                        .required(true),
                )
                .arg(
                    Arg::with_name("speed")
                        .short("s")
                        .long("speed")
                        .takes_value(true)
                        .help("Sets speed of the vehicle")
                        .required(true),
                )
                .arg(
                    Arg::with_name("turning_radius")
                        .short("t")
                        .long("turning-radius")
                        .takes_value(true)
                        .help("Turning radius of the vehicle")
                        .required(true),
                )
                .arg(
                    Arg::with_name("resolution")
                        .short("r")
                        .long("resolution")
                        .takes_value(true)
                        .help("Resolution at which the trajectory is produced"),
                )
                .arg(
                    Arg::with_name("output_dir")
                        .short("o")
                        .long("output-dir")
                        .takes_value(true)
                        .help("Output directory to place generated trajectories in")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("gen-data-2d")
                .about("Generates trajectory data")
                .arg(
                    Arg::with_name("length")
                        .short("l")
                        .long("length")
                        .takes_value(true)
                        .help("Sets the (minimum) length of the generated trajectory, in seconds"),
                )
                .arg(
                    Arg::with_name("variability")
                        .short("v")
                        .long("variability")
                        .takes_value(true)
                        .help("Sets variability of the trajectory"),
                )
                .arg(
                    Arg::with_name("rsd")
                        .short("r")
                        .long("rsd")
                        .takes_value(true)
                        .help("Sets rsd of the trajectory"),
                )
                .arg(
                    Arg::with_name("turnability")
                        .short("t")
                        .long("turnability")
                        .takes_value(true)
                        .help("Sets turnability of the trajectory"),
                )
                .arg(
                    Arg::with_name("num")
                        .short("n")
                        .long("num-trajectories")
                        .takes_value(true)
                        .help("How many trajectories to generate")
                        .required(true),
                )
                .arg(
                    Arg::with_name("output_dir")
                        .short("o")
                        .long("output-dir")
                        .takes_value(true)
                        .help("Output directory to place generated trajectories in")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("gen-tf-data")
                .about("Generates TFRecord trajectory data")
                .arg(
                    Arg::with_name("length")
                        .short("l")
                        .long("length")
                        .takes_value(true)
                        .help("Sets the (minimum) length of the generated trajectory, in seconds"),
                )
                .arg(
                    Arg::with_name("variability")
                        .short("v")
                        .long("variability")
                        .takes_value(true)
                        .help("Sets variability of the trajectory"),
                )
                .arg(
                    Arg::with_name("rsd")
                        .short("r")
                        .long("rsd")
                        .takes_value(true)
                        .help("Sets rsd of the trajectory"),
                )
                .arg(
                    Arg::with_name("num")
                        .short("n")
                        .long("num-trajectories")
                        .takes_value(true)
                        .help("How many trajectories to generate")
                        .required(true),
                )
                .arg(
                    Arg::with_name("output_dir")
                        .short("o")
                        .long("output-dir")
                        .takes_value(true)
                        .help("Output directory to place generated trajectories in")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("task")
                .about("Executes a YAML task file")
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .long("file")
                        .takes_value(true)
                        .help("Sets the filepath of the task file")
                        .required(false),
                )
                .arg(
                    Arg::with_name("print_file")
                        .short("p")
                        .long("print_file")
                        .help("Prints the full path of the YAML dginfo file which was generated"),
                )
                .arg(
                    Arg::with_name("generic")
                        .short("g")
                        .long("generic")
                        .help("Uses the generic task spec for the YAML file"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("test", _) => test_traj_gen(),
        ("gen-traj", Some(m)) => {
            let length = m
                .value_of("length")
                .map_or(10., |s| s.parse::<f64>().unwrap());
            let variability = m
                .value_of("variability")
                .map_or(2., |s| s.parse::<f64>().unwrap());
            let rsd = m.value_of("rsd").map_or(0.1, |s| s.parse::<f64>().unwrap());
            let num = m.value_of("num").unwrap().parse::<usize>().unwrap();
            let out = m.value_of("output_dir").unwrap();
            traj_gen(length, variability, rsd, num, out);
        }
        ("gen-traj-2d", Some(m)) => {
            let length = m
                .value_of("length")
                .map_or(10., |s| s.parse::<f64>().unwrap());
            let variability = m
                .value_of("variability")
                .map_or(2., |s| s.parse::<f64>().unwrap());
            let rsd = m.value_of("rsd").map_or(0.1, |s| s.parse::<f64>().unwrap());
            let num = m.value_of("num").unwrap().parse::<usize>().unwrap();
            let out = m.value_of("output_dir").unwrap();
            let turnability = m
                .value_of("turnability")
                .map_or(1., |s| s.parse::<f64>().unwrap());
            traj_gen_2d(length, variability, rsd, turnability, num, out);
        }
        ("gen-data", Some(m)) => {
            let length = m
                .value_of("length")
                .map_or(10., |s| s.parse::<f64>().unwrap());
            let variability = m
                .value_of("variability")
                .map_or(2., |s| s.parse::<f64>().unwrap());
            let rsd = m.value_of("rsd").map_or(0.1, |s| s.parse::<f64>().unwrap());
            let num = m.value_of("num").unwrap().parse::<usize>().unwrap();
            let out = m.value_of("output_dir").unwrap();
            data_gen(length, variability, rsd, num, out);
        }
        ("gen-data-2d", Some(m)) => {
            let length = m
                .value_of("length")
                .map_or(10., |s| s.parse::<f64>().unwrap());
            let variability = m
                .value_of("variability")
                .map_or(2., |s| s.parse::<f64>().unwrap());
            let rsd = m.value_of("rsd").map_or(0.1, |s| s.parse::<f64>().unwrap());
            let num = m.value_of("num").unwrap().parse::<usize>().unwrap();
            let out = m.value_of("output_dir").unwrap();
            let turnability = m
                .value_of("turnability")
                .map_or(1., |s| s.parse::<f64>().unwrap());
            data_gen_2d(length, variability, rsd, turnability, num, out);
        }
        ("gen-tf-data", Some(m)) => {
            let length = m
                .value_of("length")
                .map_or(10., |s| s.parse::<f64>().unwrap());
            let variability = m
                .value_of("variability")
                .map_or(2., |s| s.parse::<f64>().unwrap());
            let rsd = m.value_of("rsd").map_or(0.1, |s| s.parse::<f64>().unwrap());
            let num = m.value_of("num").unwrap().parse::<usize>().unwrap();
            let out = m.value_of("output_dir").unwrap();
            tf_data_gen(length, variability, rsd, num, out).unwrap();
        }
        ("task", Some(m)) => {
            let file = m.value_of("file");
            let print = m.is_present("print_file");
            let generic = m.is_present("generic");
            match exec_task(file, print, generic) {
                Ok(()) => return,
                Err(err) => {
                    eprintln!("An error has occurred: {}", err);
                    ::std::process::exit(1);
                }
            }
        }
        ("gen-dubins", Some(m)) => {
            let length = m
                .value_of("length")
                .map_or(10., |s| s.parse::<f64>().unwrap());
            let out = m.value_of("output_dir").unwrap();
            let num = m.value_of("num").unwrap().parse::<usize>().unwrap();
            let arena_size = m.value_of("arena_size").unwrap().parse::<f64>().unwrap();
            let speed = m.value_of("speed").unwrap().parse::<f64>().unwrap();
            let turning_radius = m
                .value_of("turning_radius")
                .unwrap()
                .parse::<f64>()
                .unwrap();
            let resolution = m
                .value_of("resolution")
                .map_or(1. / 8., |s| s.parse::<f64>().unwrap());

            dubins_gen(
                length,
                num,
                arena_size,
                speed,
                turning_radius,
                resolution,
                out,
            );
        }
        _ => eprintln!("Command needs to be specified. Use `--help` to view usage."),
    }
}

fn dubins_gen(
    length: f64,
    num: usize,
    arena_size: f64,
    speed: f64,
    turning_radius: f64,
    resolution: f64,
    out: &str,
) {
    let num_len = num.to_string().len();
    let out_dir_path = Path::new(out);
    std::fs::create_dir_all(out_dir_path).unwrap();
    let mut rng = thread_rng();
    let mut all_trajs: Vec<dubins::MultiDubinsPath> = Vec::with_capacity(num);
    for i in 0..num {
        print!(
            "\rWorking... [{:0width$}/{:0width$}]",
            i + 1,
            num,
            width = num_len
        );
        let trajectory = dubins::MultiDubinsPath::generate(
            turning_radius,
            speed,
            length,
            &mut rng,
            OrientedPosition2D::new(0., 0., PI / 2.),
            arena_size,
        ).unwrap();
        all_trajs.push(trajectory.clone());
        let data = trajectory.to_dynamic_trajectory(resolution);

        let file_name = format!("dtraj_{:0width$}.csv", i, width = num_len);
        let mut writer = csv::Writer::from_path(out_dir_path.join(&file_name)).unwrap();
        writer
            .write_record(&["t", "x", "y", "r", "v", "w"])
            .unwrap();
        for (t, (pos, r), (v, w)) in data {
            writer
                .write_record(&[
                    t.to_string(),
                    pos.x.to_string(),
                    pos.y.to_string(),
                    r.to_string(),
                    v.to_string(),
                    w.to_string(),
                ])
                .unwrap();
        }
        writer.flush().unwrap();
    }
    let dump_file_path = out_dir_path.join("traj_dump.yaml");
    let mut dump_file = File::create(&dump_file_path).unwrap();
    serde_yaml::to_writer(&mut dump_file, &all_trajs).unwrap();
    println!("\nDone!");
}

fn traj_gen(length: f64, variability: f64, rsd: f64, num: usize, out: &str) {
    let max_speed = 0.5;
    let num_len = num.to_string().len();
    let out_dir_path = Path::new(out);
    std::fs::create_dir_all(out_dir_path).unwrap();
    let mut all_trajs: Vec<Vec<(Seconds, Metres)>> = Vec::with_capacity(num);
    for i in 0..num {
        print!(
            "\rWorking... [{:0width$}/{:0width$}]",
            i + 1,
            num,
            width = num_len
        );
        let trajectory =
            trajectory::generate_1d_trajectory_points_simple(max_speed, length, variability, rsd);
        all_trajs.push(trajectory.clone());

        // write to a test path
        let file_name = format!("traj_{:0width$}.csv", i, width = num_len);
        let mut writer = csv::Writer::from_path(out_dir_path.join(&file_name)).unwrap();
        writer.write_record(&["t", "x"]).unwrap();
        for &(t, r) in trajectory.iter() {
            writer
                .write_record(&[t.to_string(), r.to_string()])
                .unwrap();
        }
        writer.flush().unwrap();
    }
    let dump_file_path = out_dir_path.join("traj_dump.yaml");
    let mut dump_file = File::create(&dump_file_path).unwrap();
    serde_yaml::to_writer(&mut dump_file, &all_trajs).unwrap();
    println!("\nDone!");
}

fn traj_gen_2d(length: f64, variability: f64, rsd: f64, turnability: f64, num: usize, out: &str) {
    let max_speed = 0.5;
    let num_len = num.to_string().len();
    let out_dir_path = Path::new(out);
    std::fs::create_dir_all(out_dir_path).unwrap();
    let mut all_trajs: Vec<Vec<(Seconds, Metres2D)>> = Vec::with_capacity(num);
    for i in 0..num {
        print!(
            "\rWorking... [{:0width$}/{:0width$}]",
            i + 1,
            num,
            width = num_len
        );
        let trajectory = trajectory::generate_2d_trajectory_points_simple(
            max_speed,
            length,
            variability,
            rsd,
            turnability,
        );
        all_trajs.push(trajectory.clone());

        // write to a test path
        let file_name = format!("traj_{:0width$}.csv", i, width = num_len);
        let mut writer = csv::Writer::from_path(out_dir_path.join(&file_name)).unwrap();
        writer.write_record(&["t", "x", "y"]).unwrap();
        for &(t, r) in trajectory.iter() {
            writer
                .write_record(&[t.to_string(), r.x.to_string(), r.y.to_string()])
                .unwrap();
        }
        writer.flush().unwrap();
    }
    let dump_file_path = out_dir_path.join("traj_dump.yaml");
    let mut dump_file = File::create(&dump_file_path).unwrap();
    serde_yaml::to_writer(&mut dump_file, &all_trajs).unwrap();
    println!("\nDone!");
}

fn tf_data_gen(
    length: f64,
    variability: f64,
    rsd: f64,
    num: usize,
    out: &str,
) -> tf_record::TfRecordResult<()> {
    let max_speed = 0.5;
    let num_len = num.to_string().len();
    let mut writer = ResultsWriter::from_path(out)?;

    for i in 0..num {
        print!(
            "\rWorking... [{:0width$}/{:0width$}]",
            i + 1,
            num,
            width = num_len
        );
        let trajectory =
            trajectory::generate_1d_trajectory_points_simple(max_speed, length, variability, rsd);
        let resolution = 1. / 10.;
        let trajectory_mode = simulation::LeaderTrajectoryMode::Follow;
        let converted_trajectory = trajectory::NaiveTrajectory::from_points(resolution, trajectory);

        // simulate
        let cparams = simulation::PControllerParams::default();
        let controllers = vec![
            simulation::PController::new(cparams),
            simulation::PController::new(cparams),
        ];
        let sensors = vec![
            simulation::SharpIrSensor::new(),
            simulation::SharpIrSensor::new(),
        ];
        let formation = simulation::SimpleFormation::new(2, 0., vec![0.2, 0.]);
        let simulation0 = simulation::SimpleSimulation::new(
            2,
            0,
            sensors.clone(),
            controllers.clone(),
            &formation,
            &converted_trajectory,
            trajectory_mode,
        );
        let simulation1 = simulation::SimpleSimulation::new(
            2,
            1,
            sensors,
            controllers,
            &formation,
            &converted_trajectory,
            trajectory_mode,
        );
        let mut observer = simulation::SimpleObserver::new(0.05);
        //let mut observer = simulation::PerfectObserver {};
        let result0 = simulation0.run(length, resolution, &mut observer);
        let result1 = simulation1.run(length, resolution, &mut observer);

        writer.write_record(result0, 0)?;
        writer.write_record(result1, 1)?;
    }
    writer.write_dginfo()?;
    println!("\nDone!");

    Ok(())
}

fn data_gen(length: f64, variability: f64, rsd: f64, num: usize, out: &str) {
    let max_speed = 0.5;
    let num_len = num.to_string().len();
    let out_dir_path = Path::new(out);
    std::fs::create_dir_all(out_dir_path).unwrap();

    for i in 0..num {
        print!(
            "\rWorking... [{:0width$}/{:0width$}]",
            i + 1,
            num,
            width = num_len
        );
        let trajectory =
            trajectory::generate_1d_trajectory_points_simple(max_speed, length, variability, rsd);
        let resolution = 1. / 10.;
        let trajectory_mode = simulation::LeaderTrajectoryMode::Follow;
        let converted_trajectory = trajectory::NaiveTrajectory::from_points(resolution, trajectory);

        // simulate
        let cparams = simulation::PIDControllerParams {
            d_gain: -2.7717956208417274,
            i_gain: -49.87846192197631,
            p_gain: -98.10637431966632,
            vel_limits: (-0.5, 0.5),
        };
        let controllers = vec![simulation::PIDController::new(cparams); 2];
        let sensors = vec![simulation::SharpIrSensor::new(); 2];
        let formation = simulation::SimpleFormation::new(2, 0., vec![0.2, 0.]);
        let simulation0 = simulation::SimpleSimulation::new(
            2,
            0,
            sensors.clone(),
            controllers.clone(),
            &formation,
            &converted_trajectory,
            trajectory_mode,
        );
        let simulation1 = simulation::SimpleSimulation::new(
            2,
            1,
            sensors,
            controllers,
            &formation,
            &converted_trajectory,
            trajectory_mode,
        );
        //        let mut observer = simulation::SimpleObserver::new(0.1);
        let mut observer = simulation::PerfectObserver {};
        let result0 = simulation0
            .run(length, resolution, &mut observer)
            .into_data();
        let result1 = simulation1
            .run(length, resolution, &mut observer)
            .into_data();

        let file_name = format!("traj_{:0width$}_l0.csv", i, width = num_len);
        let mut writer = csv::Writer::from_path(out_dir_path.join(&file_name)).unwrap();
        writer.write_record(&["t", "r1", "r2"]).unwrap();
        for (i, (r1, r2)) in result0[0].iter().zip(result0[1].iter()).enumerate() {
            let t = i as f64 * resolution;
            writer
                .write_record(&[t.to_string(), r1.to_string(), r2.to_string()])
                .unwrap();
        }
        writer.flush().unwrap();

        let file_name = format!("traj_{:0width$}_l1.csv", i, width = num_len);
        let mut writer = csv::Writer::from_path(out_dir_path.join(&file_name)).unwrap();
        writer.write_record(&["t", "r1", "r2"]).unwrap();
        for (i, (r1, r2)) in result1[0].iter().zip(result1[1].iter()).enumerate() {
            let t = i as f64 * resolution;
            writer
                .write_record(&[t.to_string(), r1.to_string(), r2.to_string()])
                .unwrap();
        }
        writer.flush().unwrap();
    }
    println!("\nDone!");
}

fn data_gen_2d(length: f64, variability: f64, rsd: f64, turnability: f64, num: usize, out: &str) {
    let max_speed = 0.5;
    let num_len = num.to_string().len();
    let out_dir_path = Path::new(out);
    std::fs::create_dir_all(out_dir_path).unwrap();

    for i in 0..num {
        print!(
            "\rWorking... [{:0width$}/{:0width$}]",
            i + 1,
            num,
            width = num_len
        );
        let trajectory = trajectory::generate_2d_trajectory_points_simple(
            max_speed,
            length,
            variability,
            rsd,
            turnability,
        );
        let resolution = 1. / 10.;
        let trajectory_mode = simulation::LeaderTrajectoryMode::Follow;
        let converted_trajectory = trajectory::NaiveTrajectory::from_points(resolution, trajectory);

        // simulate
        let cparams = simulation::PIDControllerParams {
            i_gain: -28.482764826067914,
            d_gain: 2.5848698420339744,
            p_gain: -123.57725960258975,
            ..Default::default()
        };
        let controllers = vec![simulation::UniformPIDController2D::new(cparams); 2];
        let sensors = vec![simulation::CombinedIrEncoderSensor::new(); 2];
        let formation = simulation::SimpleFormation::new(
            2,
            Metres2D::zero(),
            vec![Metres2D { x: 0.2, y: 0.2 }, Metres2D::zero()],
        );
        let simulation0 = simulation::SimpleSimulation::new(
            2,
            0,
            sensors.clone(),
            controllers.clone(),
            &formation,
            &converted_trajectory,
            trajectory_mode,
        );
        let simulation1 = simulation::SimpleSimulation::new(
            2,
            1,
            sensors,
            controllers,
            &formation,
            &converted_trajectory,
            trajectory_mode,
        );
        //let mut observer = simulation::SimpleObserver::new(0.05);
        let mut observer = simulation::PerfectObserver {};
        let result0 = simulation0
            .run(length, resolution, &mut observer)
            .into_data();
        let result1 = simulation1
            .run(length, resolution, &mut observer)
            .into_data();

        let file_name = format!("traj_{:0width$}_l0.csv", i, width = num_len);
        let mut writer = csv::Writer::from_path(out_dir_path.join(&file_name)).unwrap();
        writer
            .write_record(&["t", "r1_x", "r1_y", "r2_x", "r2_y"])
            .unwrap();
        for (i, (r1, r2)) in result0[0].iter().zip(result0[1].iter()).enumerate() {
            let t = i as f64 * resolution;
            writer
                .write_record(&[
                    t.to_string(),
                    r1.x.to_string(),
                    r1.y.to_string(),
                    r2.x.to_string(),
                    r2.y.to_string(),
                ])
                .unwrap();
        }
        writer.flush().unwrap();

        let file_name = format!("traj_{:0width$}_l1.csv", i, width = num_len);
        let mut writer = csv::Writer::from_path(out_dir_path.join(&file_name)).unwrap();
        writer
            .write_record(&["t", "r1_x", "r1_y", "r2_x", "r2_y"])
            .unwrap();
        for (i, (r1, r2)) in result1[0].iter().zip(result1[1].iter()).enumerate() {
            let t = i as f64 * resolution;
            writer
                .write_record(&[
                    t.to_string(),
                    r1.x.to_string(),
                    r1.y.to_string(),
                    r2.x.to_string(),
                    r2.y.to_string(),
                ])
                .unwrap();
        }
        writer.flush().unwrap();
    }
    println!("\nDone!");
}

fn test_traj_gen() {
    // Assume 0.5m/s speed
    let sample_trajectory_points = vec![
        // max speed for 3s
        (0., 0.),
        (3., 1.5),
        // stop for 2s
        (5., 1.5),
        // half speed for 2s
        (7., 2.),
        (11., 2.),
    ];

    let resolution = 1. / 10.;
    let leader_id = 1;
    let trajectory_mode = simulation::LeaderTrajectoryMode::Follow;

    // convert
    let converted_trajectory =
        trajectory::NaiveTrajectory::from_points(resolution, sample_trajectory_points);

    // simulate
    let cparams = simulation::PIDControllerParams::default();
    let controllers = vec![
        simulation::PIDController::new(cparams),
        simulation::PIDController::new(cparams),
    ];
    let sensors = vec![
        simulation::SharpIrSensor::new(),
        simulation::SharpIrSensor::new(),
    ];
    let formation = simulation::SimpleFormation::new(2, 0., vec![0.2, 0.]);
    let simulation = simulation::SimpleSimulation::new(
        2,
        leader_id,
        sensors,
        controllers,
        &formation,
        &converted_trajectory,
        trajectory_mode,
    );
    //let mut observer = simulation::SimpleObserver::new(0.1);
    let mut observer = simulation::PerfectObserver {};

    let result = simulation.run(10., resolution, &mut observer).into_data();

    // write to a test path
    let mut writer = csv::Writer::from_path("../test_data/test_traj.csv").unwrap();
    writer.write_record(&["time", "r1", "r2"]).unwrap();
    for (i, (r1, r2)) in result[0].iter().zip(result[1].iter()).enumerate() {
        let t = i as f64 * resolution;
        writer
            .write_record(&[t.to_string(), r1.to_string(), r2.to_string()])
            .unwrap();
    }
    writer.flush().unwrap();
}

fn exec_task(file: Option<&str>, print_file: bool, generic: bool) -> Result<(), failure::Error> {
    let mut contents = String::new();
    if let Some(filename) = file {
        File::open(filename)?.read_to_string(&mut contents)?;
    } else {
        io::stdin().read_to_string(&mut contents)?;
    }
    let path: PathBuf;
    if generic {
        let task: tasks::GenericScenarioSpec = serde_yaml::from_str(&contents)?;
        debug!("Processing task definition: {:?}", task);
        path = task.execute()?;
    } else {
        let task: tasks::ScenarioSpec = serde_yaml::from_str(&contents)?;
        debug!("Processing task definition: {:?}", task);
        path = task.execute()?;
    }

    if print_file {
        let path_str = path
            .to_str()
            .ok_or(format_err!("weird characters in filepath"))?;
        println!("{}", path_str);
    }
    Ok(())
}
