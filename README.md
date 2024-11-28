## num_traits

- library for trait

## Trait

```rs
trait Vehicle {
    // abstract method
    fn start(&self);

    // default method
    fn stop(&self) {
        println!("Stopped");
    }
}
```

- A trait is a set of methods
- it can contain abstract methods which don't have an implementation
- it can contain default methods, which have an implementation

```rs
trait Vehicle {
    fn start(&self);

    fn stop(&self) {
        println!("Stopped");
    }
}

struct Car {};

impl Vehicle for Car {
    fn start(&self) {
        println!("Start !!!!");
    }
}
```

- A struct/enum/primitive can `implement` a trait
- The implementor has to provide an implementation for all ofr the `abstract methods`
- The implementor can `optionally` override the default methods

```rs
// Type T must be something that implements the VEhicle trait
fn start_and_stop<T: Vehicle>(vehicle:T) {
    vehicle.start();
    
    vehicle.stop();

}

fn main() {
    let car = Car{};
    start_and_stop(car);
}
```