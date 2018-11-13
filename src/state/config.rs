use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::ops::Range;
use std::path::Path;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "tracker")]
pub struct CmdInput {
    #[structopt(short = "c", long = "config")]
    setup_config: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StartBox {
    pub h_range: Range<f32>,
    pub v_range: Range<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub particle_count: usize,
    pub particle_color: [f32; 4],
    pub stddev: f32,
    pub noise: f32,
    pub init_box: StartBox,
    pub animals_pos: Vec<([f32; 2], [f32; 3])>,
}

impl Config {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Config, Box<Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;

        Ok(config)
    }
    pub fn load() -> Result<Config, Box<Error>> {
        let input = CmdInput::from_args();

        Self::from_path(input.setup_config)
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            particle_count: 500,
            particle_color: [1.0, 0.0, 0.0, 0.3],
            animals_pos: vec![([0.0; 2], [1.0; 3])],
            stddev: 0.5,
            noise: 0.2,
            init_box: StartBox {
                h_range: -10.0..10.0,
                v_range: -10.0..10.0,
            },
        }
    }
}
