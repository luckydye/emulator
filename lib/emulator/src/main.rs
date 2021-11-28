mod cpu;

fn main() {

    let mut cpu_instance = cpu::CPU::new();

    cpu_instance.step();

    println!("{:?}", cpu_instance.registers.a);

}
