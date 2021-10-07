use crate::naval::Class;

pub fn ship(class: Class) -> String {
    let ret: &str = match class {
        Class::Corvette => "Corvette",
        Class::Frigate => "Frigate",
    };
    ret.to_string()
}
pub fn constellation() -> String {
    let ret = "Constellation";
    ret.to_string()
}
pub fn world() -> String {
    let ret = "World";
    ret.to_string()
}
