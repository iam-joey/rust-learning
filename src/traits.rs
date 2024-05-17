pub struct Person<petType>
where
    petType: Animal + Friendly,
{
    pub first_name: String,
    pub pet: petType,
}

pub struct Dog;
#[allow(dead_code)]
pub struct Cat;

#[allow(dead_code)]
pub struct Lion;
#[allow(dead_code)]

pub struct Tiger;

trait Animal {}

impl Animal for Dog {}

impl Animal for Cat {}

impl Animal for Tiger {}

impl Animal for Lion {}

trait Friendly {}

impl Friendly for Cat {}

impl Friendly for Dog {}

pub trait Sounds {
    fn sound(&self) {}
}

impl Sounds for Dog {
    fn sound(&self) {
        println!("I'm Barking");
    }
}

impl Sounds for Cat {
    fn sound(&self) {
        println!("I'm meowing");
    }
}
