static ENABLED: AtomicBool = AtomicBool::new(false);
static LEVEL: AtomicU8 = AtomicU8::new(LogLevel::Info as u8);
static CAPACITY: AtomicUsize = AtomicUsize::new(DEFAULT_CAPACITY);
static SEQUENCE: AtomicU64 = AtomicU64::new(0);
static DROPPED: AtomicU64 = AtomicU64::new(0);
static RECORDS: OnceLock<Mutex<VecDeque<LogRecord>>> = OnceLock::new();
