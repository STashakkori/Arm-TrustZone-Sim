// $t@$h
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Mode {
    Secure,
    NonSecure,
}

type InstructionHandler = fn();

fn main() {
    let mut rl = Editor::<()>::new();
    let mut mode = Mode::Secure;
    let mut locked = false;
    let mut instruction_map: HashMap<&str, InstructionHandler> = HashMap::new();
    let mut instruction_modes: HashMap<&str, Mode> = HashMap::new();

    // Instructions
    let instructions = [
        ("ADD", add_handler as InstructionHandler, Mode::NonSecure),
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
        ("encrypt", encrypt_handler, Mode::Secure), // TODO: Need legit instr here
        ("decrypt", decrypt_handler, Mode::Secure), // TODO: Need legit instr here
    ];

    // Handlers
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

    for instruction in &instructions {
      let inst = instruction.0;
      let handler = instruction.1;
      let inst_mode = instruction.2.clone();
  
      instruction_map.insert(inst, handler);
      instruction_modes.insert(inst, inst_mode);
    }

    loop {
      let readline = rl.readline(">> ");
      match readline {
          Ok(line) => {
              rl.add_history_entry(line.as_str());
              let parts: Vec<&str> = line.split_whitespace().collect();
              if parts.is_empty() { continue; }

                match parts[0] {
                    "switch_mode" => {
                        if !locked {
                            mode = match mode {
                                Mode::Secure => {
                                    locked = true;
                                    Mode::NonSecure
                                },
                                Mode::NonSecure => Mode::Secure,
                            };
                            println!("Switched to {:?} mode", mode);
                        }
                        else { println!("Switching back to Secure mode is locked"); }
                    },
                    "exit" => break,
                    _ => {
                      let inst = parts[0];
                      execute_instruction(inst, &instruction_map, &instruction_modes, &mode);
                  },
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn execute_instruction(instruction: &str, instruction_map: &HashMap<&str, InstructionHandler>, instruction_modes: &HashMap<&str, Mode>, mode: &Mode) {
  if let Some(&handler) = instruction_map.get(instruction) {
      if let Some(required_mode) = instruction_modes.get(instruction) {
          if *required_mode == Mode::NonSecure || (*required_mode == Mode::Secure && *mode == Mode::Secure) { handler(); }
          else { println!("'{}' can't be executed in Non-Secure mode", instruction); }
      }
      else { println!("Unknown required mode"); }
  }
  else { println!("Unknown instruction"); }
}
