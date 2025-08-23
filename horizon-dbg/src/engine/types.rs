pub enum RegionType {
  Free,
  Io,
  Static,
  ModuleCode,
  ModuleData,
  Heap,
  Aliased,
  AliasedCode,
  AliasedData,
  Ipc,
  Stack,
  ThreadLocalRegion,
  Transfered,
  SharedTransfered,
  SharedCode,
  NonSecureIpc,
  NonDeviceIpc,

  Kernel,
  GeneratedCode,
  CodeOut,
  Coverage,
  Insecure,
}

pub enum SchedulingStatus {
  Idle,
  Running,
  Terminated,
  Suspended,
}

pub struct ThreadState {
  scheduling_status: SchedulingStatus,
}

pub enum ThreadContext {
  Armv7 {
    gp_registers: [u32; 13],
    stack_pointer: u32,
    link_register: u32,
    program_counter: u32,
    current_program_status_register: u32,
    float_registers: [u64; 32],
    float_status_control: u32,
    float_exception_control: u32,
    thread_id_register: u32,
  },
  Armv8 {
    gp_registers: [u64; 29],
    frame_pointer: u64,
    link_register: u64,
    stack_pointer: u64,
    program_counter: u64,
    program_state: u32,
    vector_registers: [u128; 32],
    float_control_register: u32,
    float_status_register: u32,
    thread_id_register: u64,
  }
}
