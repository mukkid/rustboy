mod cpu;

fn main() {
    let mut c: cpu::Cpu = cpu::Cpu::new();
    for n in 0..11 {
        c.write(n, (42 + n) as u32);
    }

    for m in 0..=10 {
        let out = c.read(m);
        println!("{}", out)
    }
}
