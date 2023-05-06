pub fn init_tracing(
    stdout: bool,
    filter: tracing::Level,
) -> tracing_appender::non_blocking::WorkerGuard {

    let (non_blocking, guard) = if stdout {
        let (non_blocking, guard) = tracing_appender::non_blocking(std::io::stdout());
        (non_blocking, guard)
    } else {
        let file_appender = tracing_appender::rolling::daily("logs", "service.log");
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        (non_blocking, guard)
    };

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_max_level(filter)
        .with_ansi(stdout)
        .with_target(false)
        .with_file(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    return guard;
}
