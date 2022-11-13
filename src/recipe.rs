use serde::{Serialize, Deserialize};

static RECIPE_BOOK: Vec<CraftingRecipe> = vec![
	CraftingRecipe {
		input: vec![],
		output: vec![],
	},
	
];

#[derive(Serialize, Deserialize)]
pub struct CraftingRecipe {
	pub input: Vec<Item>,
	pub output: Vec<Item>,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
	pub item_type: ItemType,
}

pub(crate) trait ItemType {
	fn identifier(&self) -> &'static str;
	
}

struct BasicItemType {
	id: String,
}

impl ItemType for BasicItemType {
	fn identifier(&self) -> &'static str {
		&self.id
	}
}
