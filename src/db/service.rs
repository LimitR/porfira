pub struct DB<'a> {
	table_name: &'a str,
	params: Vec<&'a str>,
	operator: &'a str,
	search: Option<&'a str>
}

impl DB {
	pub fn new() -> DB{
		return DB {
			table_name: "",
			params: Vec::new(),
			operator: "",
			search: None
		}
	}
	
	pub fn add_table_name(&mut self, table_name: &str) -> &mut DB {
		this.table_name = table_name;
		self
	}
	
	pub fn add_params(&mut self, params: Vec<&'a str>) -> &mut DB {
		self.params.push(params.iter());
		self
	}
	
	pub fn select(&mut self) -> &mut DB {
		self.operator = "SELECT";
		self
	}
	
	pub fn search(&mut self, element: &str) -> &mut DB {
		self.search = Some(element);
		self
	}
	
	pub fn build(&mut self) -> String {
		format!("{} {} FROM {}", self.operator, self.search.unwrap_or_else(|| "*"), self.table_name)
	}
}

fn test(){
	let mut sql = DB::new();
	let stringSQL = sql.select().search("token").add_table_name("users").build();
}