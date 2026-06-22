/// Vedic Bytecode OpCodes for Kasturi VM
#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    /// Push a literal onto the stack
    Push(crate::evaluator::Value),
    /// Pop the top of the stack
    Pop,
    /// Store top of stack into a local variable
    StoreLocal(String),
    /// Load a local variable onto the stack
    LoadLocal(String),
    /// Add top two elements
    Add,
    /// Subtract top two elements
    Sub,
    /// Multiply top two elements
    Mul,
    /// Divide top two elements
    Div,
    /// Check equality
    Equal,
    /// Call a function with N arguments
    Call(String, usize),
    /// Invoke a Dhatu (Root) with suffix and N arguments
    InvokeDhatu {
        root: String,
        suffix: String,
        arg_count: usize,
    },
    /// Jump to an instruction index
    Jump(usize),
    /// Jump backward by an instruction count (for While loops)
    JumpBack(usize),
    /// Jump if the top of the stack is truthy
    JumpIfTrue(usize),
    /// Jump if the top of the stack is falsy
    JumpIfFalse(usize),
    /// Chatushkoti check (Nyaya Logic)
    NyayaCheck,
    /// Return from the current frame
    Return,

    // ═══════════════════════════════════════════
    // Sabha DAO Contract OpCodes
    // ═══════════════════════════════════════════

    /// Store a value in persistent contract storage (survives across calls)
    ContractStore {
        contract_id: String,
        key: String,
    },
    /// Load a value from persistent contract storage
    ContractLoad {
        contract_id: String,
        key: String,
    },
    /// Emit an event to the chain log (proposal created, vote cast, etc.)
    EmitEvent {
        event_name: String,
        arg_count: usize,
    },
    /// Invoke a function on another contract (cross-contract call)
    CrossCall {
        target_contract: String,
        function_name: String,
        arg_count: usize,
    },
    /// Enforce msg.sender authorization — reverts if unauthorized
    RequireAuth(String),
    /// Check Strilinga (feminine) immutability constraint.
    /// Blocks mutation of variables ending in ā/ī from external scopes.
    LinganushasanamGuard {
        variable_name: String,
    },
}
