//Code only used in the *.test modules is ignored by the linter
#![allow(dead_code)]

use std::fs::File;
use ::anyhow::Result;
use ::serde::{Deserialize, Serialize};
use crate::{platform::Games, resource::ResourceManager};

const TestPathsFilePath: &'static str = "testpaths.json";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InstallPathData
{
	pub game: i32,
	pub path: String,
}

fn readTestPaths() -> Result<Vec<InstallPathData>>
{
	let file = File::open(TestPathsFilePath)?;
	let data = serde_json::from_reader(file)?;
	
	return Ok(data);
}

pub fn updateResourceManager(manager: &ResourceManager) -> Result<()>
{
	let paths = readTestPaths()?;
	for data in paths
	{
		if !data.path.is_empty()
		{
			if let Ok(game) = TryInto::<Games>::try_into(data.game)
			{
				manager.setInstallPath(game, data.path);
			}
		}
	}
	
	return Ok(());
}
