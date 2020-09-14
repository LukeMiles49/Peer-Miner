use super::*;

use std::convert::TryFrom;

use lib::{Colour, weighted_random};

use higher_order_functions::{Init, Map};
use sized_matrix::Vector;
use noise_fn::{
	Seedable,
	NoiseDomain,
	Config,
	Octaves,
	ScaleNoise,
	AddNoise,
	Gradient,
	Simplex,
	HashNoise,
	ToFloat,
	helpers::IgnoreSeed,
};

type GameNoise = AddNoise<ScaleNoise<Octaves<Simplex, 10>, f64, f64>, IgnoreSeed<u64, Gradient<f64, 2>>>;
type DataNoise = ToFloat<HashNoise>;

pub struct GameRules {
	blocks: Vec<Block>,
	layers: Vec<Layer>,
	depth: Config<GameNoise>,
	data: Config<DataNoise>,
}

pub struct Layer {
	start: f64,
	end: f64,
	blocks: BlockPalette,
}

pub struct BlockPalette {
	blocks: Vec<(u16, f64)>,
}

impl Layer {
	pub fn new(start: f64, end: f64, blocks: Vec<(u16, f64)>) -> Self {
		let mut total = 0.0;
		for (_, weight) in blocks.iter() {
			total += weight;
		}
		Layer {
			start,
			end,
			blocks: BlockPalette { blocks: blocks.map(|(id, weight)| (id, weight / total)) }
		}
	}
}

fn register<T>(list: &mut Vec<T>, value: T) -> u16 {
	let index = list.len() as u16;
	list.push(value);
	index
}

impl GameRules {
	pub fn load() -> Self {
		let mut blocks = Vec::new();
		let b = &mut blocks;
		
		let air = register(b, Block::new(Colour::try_from("#000").unwrap()));
		let sand = register(b, Block::new(Colour::try_from("#C82").unwrap()));
		let stone = register(b, Block::new(Colour::try_from("#741").unwrap()));
		let rock = register(b, Block::new(Colour::try_from("#432").unwrap()));
		
		let mut layers = Vec::new();
		let l = &mut layers;
		
		register(l, Layer::new(f64::NEG_INFINITY, 0., vec![(air, 1.0)]));
		register(l, Layer::new(0., 200., vec![(sand, 1.0), (stone, 0.01)]));
		register(l, Layer::new(50., f64::INFINITY, vec![(rock, 1.0)]));
		
		Self {
			blocks,
			layers,
			depth: AddNoise::new(
				ScaleNoise::new(
					Octaves::<_, 10>::new(
						Simplex::new(),
						2.0, 0.5,
					),
					1.0 / 4096.0, 192.0,
				),
				IgnoreSeed::new(
					Gradient::new(
						Vector::vector([0.0, 1.0]),
					)
				),
			),
			data: ToFloat::new(
				HashNoise::new(),
			),
		}
	}
	
	pub fn block(&self, id: u16) -> &Block {
		&self.blocks[id as usize]
	}
	
	pub fn generate_chunk(&self, world: &WorldGenParams, chunk: Vector<i32, 2>) -> Chunk {
		let chunk = Chunk::init(|local| -> u16 {
			let pos = chunk * Chunk::I_SIZE + local.map(|x| x as i32);
			let depth = world.depth.noise(pos.map(f64::from));
			let mut blocks = Vec::new();
			let layers: Vec<&Layer> = self.layers.iter().filter(|l| l.start <= depth && depth <= l.end).collect();
			if layers.len() == 1 {
				weighted_random(layers[0].blocks.blocks.clone(), world.data.noise(pos))
			} else {
				for i in 0..layers.len() {
					let layer = layers[i];
					let mut max = 0.0;
					for j in 0..layers.len() {
						let other = layers[j];
						if j != i {
							let distance =
								if other.start < layer.start { depth - layer.start }
								else if other.end > layer.end { layer.end - depth }
								else { f64::abs(depth - (other.start + other.end) / 2.0) };
							if distance > max {
								max = distance;
							}
						}
					}
					for (id, weight) in &layer.blocks.blocks {
						blocks.push((*id, *weight * max));
					}
				}
				weighted_random(blocks, world.data.noise(pos))
			}
		});
		
		chunk
	}
}

pub struct WorldGenParams {
	depth: GameNoise,
	data: DataNoise,
}

impl WorldGenParams {
	pub fn new(rules: &GameRules, seed: u64) -> Self {
		Self {
			depth: rules.depth.seed(seed),
			data: rules.data.seed(seed),
		}
	}
}
