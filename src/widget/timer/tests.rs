#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_new() {
        let timer = Timer::countdown(60);
        assert_eq!(timer.remaining_seconds(), 60);
        assert_eq!(timer.state(), TimerState::Stopped);
    }

    #[test]
    fn test_timer_format() {
        let timer = Timer::countdown(3661); // 1h 1m 1s
        assert_eq!(timer.format_remaining(), "01:01:01");

        let timer2 = Timer::countdown(65).format(TimerFormat::Short);
        assert_eq!(timer2.format_remaining(), "01:05");

        let timer3 = Timer::countdown(90).format(TimerFormat::Compact);
        assert_eq!(timer3.format_remaining(), "1m 30s");
    }

    #[test]
    fn test_timer_start_pause() {
        let mut timer = Timer::countdown(60);
        assert_eq!(timer.state(), TimerState::Stopped);

        timer.start();
        assert_eq!(timer.state(), TimerState::Running);

        timer.pause();
        assert_eq!(timer.state(), TimerState::Paused);

        timer.start();
        assert_eq!(timer.state(), TimerState::Running);
    }

    #[test]
    fn test_timer_progress() {
        let mut timer = Timer::countdown(100);
        assert_eq!(timer.progress(), 0.0);

        timer.remaining_ms = 50000; // 50%
        assert!((timer.progress() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_stopwatch_new() {
        let sw = Stopwatch::new();
        assert_eq!(sw.elapsed_millis(), 0);
        assert_eq!(sw.state, TimerState::Stopped);
    }

    #[test]
    fn test_stopwatch_lap() {
        let mut sw = Stopwatch::new();
        // Manually add laps to test lap storage
        sw.laps.push(1000);
        sw.laps.push(2500);

        assert_eq!(sw.laps().len(), 2);
        assert_eq!(sw.laps()[0], 1000);
        assert_eq!(sw.laps()[1], 2500);
    }

    #[test]
    fn test_format_ms() {
        assert_eq!(format_ms(3661000, TimerFormat::Full), "01:01:01");
        assert_eq!(format_ms(65000, TimerFormat::Short), "01:05");
        assert_eq!(format_ms(5500, TimerFormat::Precise), "00:05.500");
        assert_eq!(format_ms(90000, TimerFormat::Compact), "1m 30s");
    }

    #[test]
    fn test_pomodoro() {
        let timer = Timer::pomodoro();
        assert_eq!(timer.remaining_seconds(), 25 * 60);
        assert_eq!(timer.title, Some("Pomodoro".to_string()));
    }

    #[test]
    fn test_helper_functions() {
        let t = timer(120);
        assert_eq!(t.remaining_seconds(), 120);

        let sw = stopwatch();
        assert_eq!(sw.elapsed_millis(), 0);

        let p = pomodoro();
        assert_eq!(p.remaining_seconds(), 25 * 60);
    }
}
