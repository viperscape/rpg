
pub trait Perks {
	fn buff (k:K) -> bool {}
	fn debuff (k:&K) -> bool {}
	fn remove () {}
	fn add () {}
}