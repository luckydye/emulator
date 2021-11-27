mod cpu;

fn main() {

    let mut cpu_instance = cpu::CPU::new();

    cpu_instance.execute(cpu::instructions::Instruction::ADD(cpu::instructions::ArithmeticTarget::C));

    println!("{:?}", cpu_instance.registers.a);

}
