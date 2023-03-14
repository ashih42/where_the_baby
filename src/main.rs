type Baby = String;

#[derive(Default, Debug)]
struct House {
    name: String,
    baby: Option<Baby>,
}

impl House {
    fn new(name: &str) -> House {
        House {
            name: String::from(name),
            baby: None,
        }
    }
}

#[derive(Default, Debug)]
struct Car {
    gas: f64,
    baby: Option<Baby>,
}

impl Car {
    fn drive(&self, dest: &House) {
        println!("Driving to {}", dest.name);
    }
}

#[derive(Default, Debug)]
struct Jimmy<'a> {
    baby: Option<Baby>,
    home: House,
    car: Car,
    current_location: Option<&'a mut House>,
}

impl<'a> Jimmy<'a> {
    fn new() -> Jimmy<'a> {
        let mut jimmy = Jimmy {
            baby: None,
            home: House::new("Jimmyhaus"),
            car: Car::default(),
            current_location: None,
        };

        // this line causes a borrow on `jimmy`, so `jimmy` cannot be returned
        jimmy.current_location = Some(&mut jimmy.home);
        jimmy
    }

    fn go_home(&'a mut self) {
        self.current_location = Some(&mut self.home);
    }

    fn pickup_baby(&mut self, baby: Baby) {
        match self.baby {
            Some(_) => panic!("Oh no, now I have 2 babies!"),
            None => self.baby = Some(baby),
        }
    }

    fn go_to(&mut self, dest: &'a mut House) {
        self.car.baby = Some(self.find_baby());
        self.car.drive(dest);
        dest.baby = Some(self.find_baby());
        self.current_location = Some(dest);
    }

    fn find_baby(&mut self) -> Baby {
        // Check if baby is on Jimmmy
        if let Some(baby) = self.baby.take() {
            return baby;
        }

        // Check if baby is in Jimmy's car
        if let Some(baby) = self.car.baby.take() {
            return baby;
        }

        // Check if baby is at Jimmy's house via baby monitor
        if let Some(baby) = self.home.baby.take() {
            return baby;
        }

        // Check if baby is at wherever Jimmy is at
        if let Some(current_location) = self.current_location.as_deref_mut() {
            if let Some(baby) = current_location.baby.take() {
                return baby;
            }
        }

        panic!("Oh no, where is baby?");
    }
}

fn main() {
    let mut jimmy = Jimmy::new();
    let aldi: Baby = String::from("Aldi");
    let mut aq_hq = House::new("Mikehaus");

    // jimmy.go_home();
    jimmy.pickup_baby(aldi);
    println!("jimmy:\n{:#?}\n", jimmy);
    // println!();

    jimmy.go_to(&mut aq_hq);
    println!("jimmy at aq_hq:\n{:#?}\n", jimmy);
    println!("aq_hq:\n{:#?}\n", aq_hq);
}

fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
