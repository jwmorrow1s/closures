use std::{thread, time, marker, collections, hash };
use time::Duration;
use marker::PhantomData;
use collections::HashMap;
use hash::Hash;

#[allow(unused)]
struct Cacher<'a, F, U, V>
where 
F: Fn(&'a U) -> &'a V,
U: Eq + Hash,
{
    calculation: F,
    cache: HashMap<&'a U, &'a V>,
    _phantom: PhantomData<&'a U>,
}

impl<'a, F, U, V> Cacher<'a, F, U, V>
where 
F: Fn(&'a U) -> &'a V,
U: Eq + Hash
{
    #[allow(unused)]
    fn new(calculation: F) -> Self {
        Cacher {
            calculation: calculation,
            cache: HashMap::new(),
            _phantom: PhantomData{},
        }
    }

    #[allow(unused)]
    fn value(&mut self, arg: &'a U) -> &'a V {
        match &self.cache.get(&arg) {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.cache.insert(arg, v);
                v
            }
        }
    }
}

fn main() {
    generate_workout(5, 1);
    println!("Hello, world!");
}

#[allow(unused)]
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new((|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    }));

    if intensity > 25 {
        println!("Today, do {} pushups!", expensive_result.value(&intensity));
        println!("Next, do {} situps!", expensive_result.value(&intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(&intensity)
            )
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_cacher_different_values(){
        let mut c = Cacher::new(|a: &u32|a);

        let _v1 = c.value(&1);
        let v2 = c.value(&2);

        assert_eq!(v2, &2);
    }
}
