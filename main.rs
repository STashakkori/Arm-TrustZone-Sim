// $t@$h
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

// Main Instruction Handlers
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

// System Instruction Handlers
fn init_trustzone() { println!("Initialized TrustZone in EL3"); }
fn setup_virtualization() { println!("Set up virtualization in EL2"); }
fn init_kernel() { println!("Kernel initialized in EL1"); }
fn start_user_apps() { println!("User space applications started in EL0"); }

fn provide_hint(mode: Mode, current_el: ExceptionLevel) {
    match current_el {
        ExceptionLevel::EL3 => println!("Hint: Type 'init_trustzone' to initialize TrustZone and transition to EL2"),
        ExceptionLevel::EL2 => println!("Hint: Type 'setup_virtualization' to set up virtualization and transition to EL1"),
        ExceptionLevel::EL1 => println!("Hint: Type 'init_kernel' to initialize the kernel and transition to EL0"),
        ExceptionLevel::EL0 => {
            match mode {
                Mode::Secure => println!("Hint: Perform secure operations or type 'switch_mode' to change to Non-Secure mode"),
                Mode::NonSecure => println!("Hint: You can now execute Non-Secure instructions like 'ADD', 'SUB', etc."),
            }
        }
    }
}

fn print_instructions_list(instruction_map: &HashMap<&str, InstructionHandler>) {
    println!("Available Instructions:");
    for instruction in instruction_map.keys() {
        println!(" {}", instruction);
    }
}

fn main() {
    let mut rl = Editor::<()>::new();
    let mut mode = Mode::Secure;
    let mut current_el = ExceptionLevel::EL3;
    let mut instruction_map: HashMap<&str, InstructionHandler> = HashMap::new();
    let mut instruction_modes: HashMap<&str, Mode> = HashMap::new();

    // Arm Instructions Simplified
    let instructions = [
        ("ADD", add_handler as InstructionHandler, Mode::NonSecure),
        ("SUB", sub_handler as InstructionHandler, Mode::NonSecure),
        ("AND", and_handler as InstructionHandler, Mode::NonSecure),
        ("ORR", orr_handler as InstructionHandler, Mode::NonSecure),
        ("EOR", eor_handler as InstructionHandler, Mode::NonSecure),
        ("B", b_handler as InstructionHandler, Mode::NonSecure),
        ("BL", bl_handler as InstructionHandler, Mode::NonSecure),
        ("CMP", cmp_handler as InstructionHandler, Mode::NonSecure),
        ("CMN", cmn_handler as InstructionHandler, Mode::NonSecure),
        ("MOV", mov_handler as InstructionHandler, Mode::NonSecure),
        ("MVN", mvn_handler as InstructionHandler, Mode::NonSecure),
        ("LDR", ldr_handler as InstructionHandler, Mode::NonSecure),
        ("STR", str_handler as InstructionHandler, Mode::NonSecure),
        ("VADD", vadd_handler as InstructionHandler, Mode::NonSecure),
        ("VSUB", vsub_handler as InstructionHandler, Mode::NonSecure),
        ("FADD", fadd_handler as InstructionHandler, Mode::NonSecure),
        ("FSUB", fsub_handler as InstructionHandler, Mode::NonSecure),
        ("FMUL", fmul_handler as InstructionHandler, Mode::NonSecure),
        ("VMOV", vmov_handler as InstructionHandler, Mode::NonSecure),
        ("SADD", sadd_handler as InstructionHandler, Mode::NonSecure),
        ("SSUB", ssub_handler as InstructionHandler, Mode::NonSecure),
        ("LD1", ld1_handler as InstructionHandler, Mode::NonSecure),
        ("ST1", st1_handler as InstructionHandler, Mode::NonSecure),
        ("MATMUL", matmul_handler as InstructionHandler, Mode::NonSecure),
        ("MMV", mmv_handler as InstructionHandler, Mode::NonSecure),
        ("encrypt", encrypt_handler as InstructionHandler, Mode::Secure),
        ("decrypt", decrypt_handler as InstructionHandler, Mode::Secure),
        ("init_trustzone", init_trustzone as InstructionHandler, Mode::Secure),
        ("setup_virtualization", setup_virtualization as InstructionHandler, Mode::Secure),
        ("init_kernel", init_kernel as InstructionHandler, Mode::Secure),
        ("start_user_apps", start_user_apps as InstructionHandler, Mode::Secure),
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
		    "hint" => provide_hint(mode, current_el),
                    "instructions" => print_instructions_list(&instruction_map),
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
