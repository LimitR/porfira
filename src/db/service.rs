// pub struct DB<'a> {
// 	table_name: &'a str,
// 	params: Option<Vec<&'a str>>,
// 	operator: &'a str,
// 	search: Option<&'a str>
// }
//
// pub struct Select<'a> {
// 	table_name: &'a str,
// 	params: Option<Vec<&'a str>>,
// 	operator: &'a str,
// 	search: Option<&'a str>
// }
//
// impl DB {
// 	pub fn new() -> DB{
// 		return DB {
// 			table_name: "",
// 			params: None,
// 			operator: "",
// 			search: None
// 		}
// 	}
//
//
// 	pub fn select(&mut self) -> Select {
// 		self.operator = "SELECT";
// 		Select {
// 			table_name: self.table_name,
// 			params: None,
// 			operator: self.operator,
// 			search: self.search
// 		}
// 	}
// }
//
//
// impl Select {
// 	pub fn add_table_name(&mut self, table_name: &str) -> &mut Select {
// 		this.table_name = table_name;
// 		self
// 	}
//
// 	pub fn add_params(&mut self, mut params: Vec<&str>) -> &mut Select {
// 		match self.params {
// 			Some(mut res) => res.append(&mut params),
// 			None => self.params = Some(params)
// 		};
// 		self
// 	}
//
// 	pub fn search(&mut self, element: &str) -> &mut Select {
// 		self.search = Some(element);
// 		self
// 	}
//
// 	pub fn build(&mut self) -> String {
// 		format!("{} {} FROM {}", self.operator, self.search.unwrap_or_else(|| "*"), self.table_name)
// 	}
// }
