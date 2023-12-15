use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Mode {
    Secure,
    NonSecure,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ExceptionLevel {
    EL3,
    EL2,
    EL1,
    EL0,
}

type InstructionHandler = fn();

// Original Instruction Handlers
fn add_handler() { println!("Executed ADD instruction"); }
fn sub_handler() { println!("Executed SUB instruction"); }
fn and_handler() { println!("Executed AND instruction"); }
fn orr_handler() { println!("Executed ORR instruction"); }
fn eor_handler() { println!("Executed EOR instruction"); }
fn b_handler() { println!("Executed B instruction"); }
fn bl_handler() { println!("Executed BL instruction"); }
fn cmp_handler() { println!("Executed CMP instruction"); }
fn cmn_handler() { println!("Executed CMN instruction"); }
fn mov_handler() { println!("Executed MOV instruction"); }
fn mvn_handler() { println!("Executed MVN instruction"); }
fn ldr_handler() { println!("Executed LDR instruction"); }
fn str_handler() { println!("Executed STR instruction"); }
fn vadd_handler() { println!("Executed VADD instruction"); }
fn vsub_handler() { println!("Executed VSUB instruction"); }
fn fadd_handler() { println!("Executed FADD instruction"); }
fn fsub_handler() { println!("Executed FSUB instruction"); }
fn fmul_handler() { println!("Executed FMUL instruction"); }
fn vmov_handler() { println!("Executed VMOV instruction"); }
fn sadd_handler() { println!("Executed SADD instruction"); }
fn ssub_handler() { println!("Executed SSUB instruction"); }
fn ld1_handler() { println!("Executed LD1 instruction"); }
fn st1_handler() { println!("Executed ST1 instruction"); }
fn matmul_handler() { println!("Executed MATMUL instruction"); }
fn mmv_handler() { println!("Executed MMV instruction"); }
fn encrypt_handler() { println!("Executed ENCRYPT instruction"); }
fn decrypt_handler() { println!("Executed DECRYPT instruction"); }

// System-level Instruction Handlers
fn init_trustzone() { println!("Initialized TrustZone in EL3"); }
fn setup_virtualization() { println!("Set up virtualization in EL2"); }
fn init_kernel() { println!("Kernel initialized in EL1"); }
fn start_user_apps() { println!("User space applications started in EL0"); }

fn main() {
    let mut rl = Editor::<()>::new();
    let mut mode = Mode::Secure;
    let mut current_el = ExceptionLevel::EL3;
    let mut instruction_map: HashMap<&str, InstructionHandler> = HashMap::new();
    let mut instruction_modes: HashMap<&str, Mode> = HashMap::new();

    // Populate the instruction map with both original and system instructions
    let instructions = [
        ("ADD", add_handler, Mode::NonSecure),
        ("SUB", sub_handler, Mode::NonSecure),
        ("AND", and_handler, Mode::NonSecure),
        ("ORR", orr_handler, Mode::NonSecure),
        ("EOR", eor_handler, Mode::NonSecure),
        ("B", b_handler, Mode::NonSecure),
        ("BL", bl_handler, Mode::NonSecure),
        ("CMP", cmp_handler, Mode::NonSecure),
        ("CMN", cmn_handler, Mode::NonSecure),
        ("MOV", mov_handler, Mode::NonSecure),
        ("MVN", mvn_handler, Mode::NonSecure),
        ("LDR", ldr_handler, Mode::NonSecure),
        ("STR", str_handler, Mode::NonSecure),
        ("VADD", vadd_handler, Mode::NonSecure),
        ("VSUB", vsub_handler, Mode::NonSecure),
        ("FADD", fadd_handler, Mode::NonSecure),
        ("FSUB", fsub_handler, Mode::NonSecure),
        ("FMUL", fmul_handler, Mode::NonSecure),
        ("VMOV", vmov_handler, Mode::NonSecure),
        ("SADD", sadd_handler, Mode::NonSecure),
        ("SSUB", ssub_handler, Mode::NonSecure),
        ("LD1", ld1_handler, Mode::NonSecure),
        ("ST1", st1_handler, Mode::NonSecure),
        ("MATMUL", matmul_handler, Mode::NonSecure),
        ("MMV", mmv_handler, Mode::NonSecure),
        ("encrypt", encrypt_handler, Mode::Secure),
        ("decrypt", decrypt_handler, Mode::Secure),
        ("init_trustzone", init_trustzone, Mode::Secure),
        ("setup_virtualization", setup_virtualization, Mode::Secure),
        ("init_kernel", init_kernel, Mode::Secure),
        ("start_user_apps", start_user_apps, Mode::Secure),
    ];

    for &(inst, handler, mode) in &instructions {
        instruction_map.insert(inst, handler);
        instruction_modes.insert(inst, mode);
    }

    loop {
        let prompt = format!("{:?}>> ", current_el);
        let readline = rl.readline(&prompt);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.is_empty() { continue; }

                match parts[0] {
                    "switch_mode" if mode == Mode::Secure => {
                        mode = Mode::NonSecure;
                        println!("Switched to Non-Secure mode");
                    },
                    "switch_mode" if mode == Mode::NonSecure => {
                        println!("Switching back to Secure mode is locked");
                    },
                    _ => execute_instruction(parts[0], &instruction_map, &instruction_modes, mode, &mut current_el),
                    "exit" => break,
                }
            },
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => println!("Error: {:?}", err),
        }
    }
}

fn execute_instruction(instruction: &str, instruction_map: &HashMap<&str, InstructionHandler>, instruction_modes: &HashMap<&str, Mode>, mode: Mode, current_el: &mut ExceptionLevel) {
    if let Some(&handler) = instruction_map.get(instruction) {
        if let Some(&required_mode) = instruction_modes.get(instruction) {
            if required_mode == mode {
                handler();
                // Transition logic for ELs based on specific system instructions
                if *current_el == ExceptionLevel::EL3 && instruction == "init_trustzone" {
                    *current_el = ExceptionLevel::EL2;
                    println!("Transitioned to EL2");
                } else if *current_el == ExceptionLevel::EL2 && instruction == "setup_virtualization" {
                    *current_el = ExceptionLevel::EL1;
                    println!("Transitioned to EL1");
                } else if *current_el == ExceptionLevel::EL1 && instruction == "init_kernel" {
                    *current_el = ExceptionLevel::EL0;
                    println!("Transitioned to EL0");
                }
            } else {
                println!("Error: '{}' cannot be executed in current mode", instruction);
            }
        } else {
            println!("Unknown instruction mode requirement");
        }
    } else {
        println!("Unknown instruction");
    }
}
