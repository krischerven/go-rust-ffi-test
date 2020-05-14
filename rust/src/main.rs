extern crate elapsed;
extern crate num_format;
use num_format::ToFormattedString;

#[link(name = "ctest")]
extern "C" {
    fn cinc(x: u64) -> u64;
}

fn main() {
    let mut x = 0;
    unsafe {
        // trick the compiler into not removing dead code
        let mut dyn0 = true;
        let mut dyn1 = true;
        // ignore unused variable warning
        std::mem::swap(&mut dyn0, &mut dyn1);
        // warmup
        for _ in 0..10_000_000 {
            if dyn0 == dyn1 {
                x = pass(x)
            }
        }
        // Rust
        let t0 = elapsed::measure_time(|| {
            for _ in 0..10_000_000 {
                if dyn0 == dyn1 {
                    x = inc(x);
                }
            }
        })
        .0
        .millis() as f64;
        // warmup
        for _ in 0..10_000_000 {
            if dyn0 == dyn1 {
                x = pass(x)
            }
        }
        // C
        let t1 = elapsed::measure_time(|| {
            for _ in 0..10_000_000 {
                if dyn0 == dyn1 {
                    x = cinc(x);
                }
            }
        })
        .0
        .millis() as f64;
        println!("Rust time (ms): {}", t0);
        println!("FFI time (ms): {}", t1);
        println!("FFI overhead: {:.1}", t1 / t0);
        if t1 > t0 {
            println!(
                "FFI cost/call (nanoseconds): {:.1}",
                ((t1 - t0) / 10_000_000.0) * 1_000_000.0
            );
        } else {
            println!("FFI cost/call (nanoseconds): none");
        }
        println!(
            "FFI calls/second: {}",
            (((1000.0 / t1) * 10_000_000.0) as i32).to_formatted_string(&num_format::Locale::en)
        );
        println!(
            "Rust calls/second: {}",
            (((1000.0 / t0) * 10_000_000.0) as i32).to_formatted_string(&num_format::Locale::en)
        );
        if dyn0 == dyn1 {
            return;
        } else {
            println!("{}", dyn0);
            println!("{}", dyn1);
        }
    }
    println!("Counter: {}", x);
}

fn inc(x: u64) -> u64 {
    return x + 1;
}

fn pass(x: u64) -> u64 {
    return x + 1;
}
