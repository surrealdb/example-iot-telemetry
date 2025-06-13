use crate::{
    Args,
    event::{AppEvent, Event, EventHandler},
    queries::{SensorAverage, SensorData, queries_run},
    sensors::sensors_run,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;
use std::{
    collections::HashMap,
    sync::{
        Arc, RwLock,
        atomic::{AtomicBool, AtomicIsize, Ordering},
    },
    thread,
};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use tokio::sync::Mutex;

/// Application.
pub struct App {
    pub sensor_count: usize,
    delay: u64,
    db: Option<Arc<Mutex<Surreal<Client>>>>,
    pub sensors: Arc<RwLock<SensorData>>,
    pub avgs: Arc<RwLock<Vec<SensorAverage>>>,
    pub running: Arc<AtomicBool>,
    pub selected_sensor: usize,
    pub force_outlier_on_sensor: Arc<AtomicIsize>,
    pub events: EventHandler,
    pub window_in_minutes: Arc<RwLock<u32>>,
    pub query_delay: u64,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(args: Args) -> Self {
        Self {
            db: None,
            sensors: Arc::new(RwLock::new(HashMap::new())),
            avgs: Arc::new(RwLock::new(Vec::new())),
            sensor_count: args.count,
            delay: args.delay,
            running: Arc::new(AtomicBool::new(true)),
            selected_sensor: 0,
            force_outlier_on_sensor: Arc::new(AtomicIsize::new(-1)),
            events: EventHandler::new(),
            window_in_minutes: Arc::new(RwLock::new(1)),
            query_delay: args.query_delay,
        }
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        let db = Surreal::new::<Ws>("0.0.0.0:8000").await?;
        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await?;
        db.use_ns("telemetry-simulator").use_db("demo").await?;
        let db_ref: Arc<Mutex<Surreal<Client>>> = Arc::new(Mutex::new(db));
        let db_clone_2 = db_ref.clone();
        self.db = Some(Arc::clone(&db_ref));

        // spawn sensors
        let running_clone = self.running.clone();
        let force_outlier_on_sensor = self.force_outlier_on_sensor.clone();
        thread::spawn(move || {
            let _ = sensors_run(
                self.sensor_count,
                self.delay,
                running_clone,
                force_outlier_on_sensor,
                db_ref,
            );
        });

        // spawn queries
        let running_clone_2 = self.running.clone();
        let sensors_clone = Arc::clone(&self.sensors);
        let avgs_clone = Arc::clone(&self.avgs);
        let win_in_min = Arc::clone(&self.window_in_minutes);
        thread::spawn(move || {
            queries_run(
                db_clone_2,
                running_clone_2,
                sensors_clone,
                avgs_clone,
                win_in_min,
                self.query_delay,
            );
        });

        while self.running.load(Ordering::Relaxed) {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => {
                    let Some(key_event) = event.as_key_press_event() else {
                        continue;
                    };
                    self.handle_key_events(key_event)?
                }
                Event::App(app_event) => match app_event {
                    AppEvent::Increment => self.increment_counter(),
                    AppEvent::Decrement => self.decrement_counter(),
                    AppEvent::ForceOutlier => self.force_outlier(),
                    AppEvent::Quit => self.quit(),
                },
            }
        }

        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Down => self.events.send(AppEvent::Increment),
            KeyCode::Up => self.events.send(AppEvent::Decrement),
            KeyCode::Char(' ') => self.events.send(AppEvent::ForceOutlier),
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running.store(false, Ordering::Relaxed);
    }

    pub fn increment_counter(&mut self) {
        self.selected_sensor = self.selected_sensor.saturating_add(1) % self.sensor_count;
    }

    pub fn decrement_counter(&mut self) {
        if self.selected_sensor == 0 {
            self.selected_sensor = self.sensor_count - 1;
        } else {
            self.selected_sensor -= 1;
        }
    }

    pub fn force_outlier(&mut self) {
        self.force_outlier_on_sensor
            .store(self.selected_sensor as isize, Ordering::Relaxed);
    }
}
