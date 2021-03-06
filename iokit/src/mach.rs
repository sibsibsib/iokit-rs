use iokit_sys::base::*;
use iokit_sys::mach_sys::kern_return_t;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KernReturn {
    Success,
    InvalidAddress,
    ProtectionFailure,
    NoSpace,
    InvalidArgument,
    Failure,
    ResourceShortage,
    NotReceiver,
    NoAccess,
    MemoryFailure,
    MemoryError,
    AlreadyInSet,
    NotInSet,
    NameExists,
    Aborted,
    InvalidName,
    InvalidTask,
    InvalidRight,
    InvalidValue,
    UrefsOverflow,
    InvalidCapability,
    RightExists,
    InvalidHost,
    MemoryPresent,
    MemoryDataMoved,
    MemoryRestartCopy,
    InvalidProcessorSet,
    PolicyLimit,
    InvalidPolicy,
    InvalidObject,
    AlreadyWaiting,
    DefaultSet,
    ExceptionProtected,
    InvalidLedger,
    InvalidMemoryControl,
    InvalidSecurity,
    NotDepressed,
    Terminated,
    LockSetDestroyed,
    LockUnstable,
    LockOwned,
    LockOwnedSelf,
    SemaphoreDestroyed,
    RpcServerTerminated,
    RpcTerminateOrphan,
    RpcContinueOrphan,
    NotSupported,
    NodeDown,
    NotWaiting,
    OperationTimedOut,
    ReturnMax,
    Unknown(kern_return_t),
}

impl From<kern_return_t> for KernReturn {
    fn from(code: kern_return_t) -> KernReturn {
        match code {
            KERN_SUCCESS => KernReturn::Success,
            KERN_INVALID_ADDRESS => KernReturn::InvalidAddress,
            KERN_PROTECTION_FAILURE => KernReturn::ProtectionFailure,
            KERN_NO_SPACE => KernReturn::NoSpace,
            KERN_INVALID_ARGUMENT => KernReturn::InvalidArgument,
            KERN_FAILURE => KernReturn::Failure,
            KERN_RESOURCE_SHORTAGE => KernReturn::ResourceShortage,
            KERN_NOT_RECEIVER => KernReturn::NotReceiver,
            KERN_NO_ACCESS => KernReturn::NoAccess,
            KERN_MEMORY_FAILURE => KernReturn::MemoryFailure,
            KERN_MEMORY_ERROR => KernReturn::MemoryError,
            KERN_ALREADY_IN_SET => KernReturn::AlreadyInSet,
            KERN_NOT_IN_SET => KernReturn::NotInSet,
            KERN_NAME_EXISTS => KernReturn::NameExists,
            KERN_ABORTED => KernReturn::Aborted,
            KERN_INVALID_NAME => KernReturn::InvalidName,
            KERN_INVALID_TASK => KernReturn::InvalidTask,
            KERN_INVALID_RIGHT => KernReturn::InvalidRight,
            KERN_INVALID_VALUE => KernReturn::InvalidValue,
            KERN_UREFS_OVERFLOW => KernReturn::UrefsOverflow,
            KERN_INVALID_CAPABILITY => KernReturn::InvalidCapability,
            KERN_RIGHT_EXISTS => KernReturn::RightExists,
            KERN_INVALID_HOST => KernReturn::InvalidHost,
            KERN_MEMORY_PRESENT => KernReturn::MemoryPresent,
            KERN_MEMORY_DATA_MOVED => KernReturn::MemoryDataMoved,
            KERN_MEMORY_RESTART_COPY => KernReturn::MemoryRestartCopy,
            KERN_INVALID_PROCESSOR_SET => KernReturn::InvalidProcessorSet,
            KERN_POLICY_LIMIT => KernReturn::PolicyLimit,
            KERN_INVALID_POLICY => KernReturn::InvalidPolicy,
            KERN_INVALID_OBJECT => KernReturn::InvalidObject,
            KERN_ALREADY_WAITING => KernReturn::AlreadyWaiting,
            KERN_DEFAULT_SET => KernReturn::DefaultSet,
            KERN_EXCEPTION_PROTECTED => KernReturn::ExceptionProtected,
            KERN_INVALID_LEDGER => KernReturn::InvalidLedger,
            KERN_INVALID_MEMORY_CONTROL => KernReturn::InvalidMemoryControl,
            KERN_INVALID_SECURITY => KernReturn::InvalidSecurity,
            KERN_NOT_DEPRESSED => KernReturn::NotDepressed,
            KERN_TERMINATED => KernReturn::Terminated,
            KERN_LOCK_SET_DESTROYED => KernReturn::LockSetDestroyed,
            KERN_LOCK_UNSTABLE => KernReturn::LockUnstable,
            KERN_LOCK_OWNED => KernReturn::LockOwned,
            KERN_LOCK_OWNED_SELF => KernReturn::LockOwnedSelf,
            KERN_SEMAPHORE_DESTROYED => KernReturn::SemaphoreDestroyed,
            KERN_RPC_SERVER_TERMINATED => KernReturn::RpcServerTerminated,
            KERN_RPC_TERMINATE_ORPHAN => KernReturn::RpcTerminateOrphan,
            KERN_RPC_CONTINUE_ORPHAN => KernReturn::RpcContinueOrphan,
            KERN_NOT_SUPPORTED => KernReturn::NotSupported,
            KERN_NODE_DOWN => KernReturn::NodeDown,
            KERN_NOT_WAITING => KernReturn::NotWaiting,
            KERN_OPERATION_TIMED_OUT => KernReturn::OperationTimedOut,
            KERN_RETURN_MAX => KernReturn::ReturnMax,
            code => KernReturn::Unknown(code),
        }
    }
}
