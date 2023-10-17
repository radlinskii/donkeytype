//! Module with test statistics structure.
//!
//! Used to display to the user results of the test
//! and save those results and configuration of the test to a file.

use anyhow::{Context, Result};
use chrono::{DateTime, Datelike, Local, Timelike};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    prelude::{Backend, Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Bar, BarGroup, Block},
    widgets::{BarChart, Paragraph},
    Terminal,
};
use serde::{Deserialize, Serialize};

use std::{fs::create_dir_all, path::PathBuf, rc::Rc, thread::sleep, time::Duration};

use crate::{
    config::Config,
    runner::{FrameWrapper, FrameWrapperInterface},
};

/// TestResults struct is combining test statistics with configuration of the test.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResults {
    pub local_datetime: DateTime<Local>,

    pub wpm: Option<f64>,
    pub raw_accuracy: Option<f64>,
    pub raw_valid_characters_count: Option<u64>,
    pub raw_mistakes_count: Option<u64>,
    pub raw_typed_characters_count: Option<u64>,
    pub accuracy: Option<f64>,
    pub valid_characters_count: Option<u64>,
    pub typed_characters_count: Option<u64>,
    pub mistakes_count: Option<u64>,

    pub duration: Option<u64>,
    pub numbers: Option<bool>,
    pub numbers_ratio: Option<f64>,
    pub dictionary_path: Option<String>,
    pub uppercase: Option<bool>,
    pub uppercase_ratio: Option<f64>,

    // tells if test was successfully completed and results should be displayed and saved.
    #[serde(skip)]
    pub completed: bool,
    #[serde(skip)]
    pub save: bool,
}

/// Struct holding numeric test results.
#[derive(Debug)]
pub struct Stats {
    pub wpm: f64,
    pub raw_accuracy: f64,
    pub raw_valid_characters_count: u64,
    pub raw_mistakes_count: u64,
    pub raw_typed_characters_count: u64,
    pub accuracy: f64,
    pub valid_characters_count: u64,
    pub typed_characters_count: u64,
    pub mistakes_count: u64,
}

impl Stats {
    pub fn default() -> Self {
        Stats {
            wpm: 0.0,
            raw_accuracy: 0.0,
            raw_valid_characters_count: 0,
            raw_mistakes_count: 0,
            raw_typed_characters_count: 0,
            accuracy: 0.0,
            valid_characters_count: 0,
            mistakes_count: 0,
            typed_characters_count: 0,
        }
    }
}

impl TestResults {
    /// creates TestResults object from Stats and Config
    pub fn new(stats: Stats, config: Config, completed: bool) -> Self {
        TestResults {
            local_datetime: Local::now(),
            // stats
            wpm: Some(stats.wpm),
            raw_accuracy: Some(stats.raw_accuracy),
            raw_valid_characters_count: Some(stats.raw_valid_characters_count),
            raw_mistakes_count: Some(stats.raw_mistakes_count),
            raw_typed_characters_count: Some(stats.raw_typed_characters_count),
            accuracy: Some(stats.accuracy),
            valid_characters_count: Some(stats.valid_characters_count),
            typed_characters_count: Some(stats.typed_characters_count),
            mistakes_count: Some(stats.mistakes_count),
            // config
            duration: Some(config.duration.as_secs()),
            numbers: Some(config.numbers),
            numbers_ratio: Some(config.numbers_ratio),
            dictionary_path: if let Some(str) = config.dictionary_path.to_str() {
                Some(str.to_string())
            } else {
                None
            },
            uppercase: Some(config.uppercase),
            uppercase_ratio: Some(config.uppercase_ratio),

            completed,
            save: config.save_results,
        }
    }

    /// saves test statistics and configuration to a file in users home directory
    pub fn save_to_file(&self) -> Result<(), anyhow::Error> {
        create_results_dir_if_not_exist()
            .context("Unable to ensure that results directory exist")?;
        let results_file_path =
            get_results_file_path().context("Unable to ge results file path")?;

        let records = read_previous_results().context("Unable to read previous results")?;

        let mut writer =
            csv::Writer::from_path(results_file_path).context("Unable to create CSV Writer")?;

        for record in &records {
            writer
                .serialize(record)
                .context("Unable to serialize one of previous results")?;
        }

        writer
            .serialize(self)
            .context("Unable to serialize current test results")?;

        writer
            .flush()
            .context("Unable to flush inner csv crate buffer to writer")?;

        Ok(())
    }

    pub fn render_results<B: Backend>(&self, terminal: &mut Terminal<B>) -> Result<()> {
        let mut records = read_previous_results().context("Unable to read previous results")?;
        records.push(self.clone());

        loop {
            terminal.draw(|frame| {
                let areas = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Length(2),
                            Constraint::Length(1),
                            Constraint::Length(1),
                            Constraint::Length(1),
                            Constraint::Length(1),
                            Constraint::Length(1),
                            Constraint::Length(1),
                            Constraint::Length(1),
                            Constraint::Length(1),
                            Constraint::Length(2),
                            Constraint::Length(12),
                            Constraint::Length(1),
                            Constraint::Length(1),
                            Constraint::Min(1),
                        ]
                        .as_ref(),
                    )
                    .split(frame.size());

                frame.render_widget(Paragraph::new("Test completed"), areas[0]);
                frame.render_widget(
                    Paragraph::new("Press <Esc> to quit")
                        .alignment(ratatui::prelude::Alignment::Right)
                        .yellow(),
                    areas[0],
                );

                let mut frame_wrapper = FrameWrapper::new(frame);
                self.render_stats(&mut frame_wrapper, &areas);
                self.render_chart(&mut frame_wrapper, &areas, &records);
            })?;

            if event::poll(Duration::from_millis(100)).context("Unable to poll for event")? {
                if let Event::Key(key) = event::read().context("Unable to read event")? {
                    match key.code {
                        KeyCode::Esc => {
                            break;
                        }
                        _ => {}
                    }
                }
            }
            sleep(Duration::from_millis(100));
        }

        Ok(())
    }

    fn render_chart(
        &self,
        frame: &mut impl FrameWrapperInterface,
        areas: &Rc<[Rect]>,
        records: &Vec<TestResults>,
    ) {
        let mut records_to_show = records.clone();
        let bar_width = 5;
        let frame_width = frame.size().width;
        let bars_to_show = ((frame_width + 1) / (bar_width + 1)) as usize;
        if records.len() >= bars_to_show {
            records_to_show = records[records.len() - bars_to_show..].to_vec();
        }

        frame.render_widget(
            BarChart::default()
                .block(Block::default().title("Previous results:"))
                .bar_width(bar_width)
                .bar_gap(1)
                .bar_style(Style::new().white().on_black())
                .value_style(Style::new().black().on_white())
                .data(
                    BarGroup::default().bars(
                        &records_to_show
                            .iter()
                            .map(|r| {
                                Bar::default().value(if let Some(wpm) = r.wpm {
                                    wpm as u64
                                } else {
                                    0
                                })
                            })
                            .collect::<Vec<Bar>>(),
                    ),
                ),
            areas[10],
        );
        frame.render_widget(
            Paragraph::new(
                records_to_show
                    .iter()
                    .map(|r| {
                        format!(
                            "{}:{} ",
                            fmt_num(r.local_datetime.hour()),
                            fmt_num(r.local_datetime.minute())
                        )
                    })
                    .collect::<String>(),
            ),
            areas[11],
        );
        frame.render_widget(
            Paragraph::new(
                records_to_show
                    .iter()
                    .map(|r| {
                        format!(
                            "{}/{} ",
                            fmt_num(r.local_datetime.month()),
                            fmt_num(r.local_datetime.day())
                        )
                    })
                    .collect::<String>(),
            ),
            areas[12],
        );
        frame.render_widget(
            Paragraph::new(
                records_to_show
                    .iter()
                    .map(|r| format!("{}  ", r.local_datetime.year()))
                    .collect::<String>(),
            ),
            areas[13],
        );
    }

    fn render_stats(&self, frame: &mut impl FrameWrapperInterface, areas: &Rc<[Rect]>) {
        if let Some(wpm) = self.wpm {
            frame.render_widget(Paragraph::new(format!("WPM: {:.2}", wpm)), areas[1]);
        }
        if let Some(raw_accuracy) = self.raw_accuracy {
            frame.render_widget(
                Paragraph::new(format!("Raw accuracy: {:.2}%", raw_accuracy)),
                areas[2],
            );
        }
        if let Some(raw_valid_characters_count) = self.raw_valid_characters_count {
            frame.render_widget(
                Paragraph::new(format!(
                    "Raw valid characters: {}",
                    raw_valid_characters_count
                )),
                areas[3],
            );
        }
        if let Some(raw_mistakes_count) = self.raw_mistakes_count {
            frame.render_widget(
                Paragraph::new(format!("Raw mistakes: {}", raw_mistakes_count)),
                areas[4],
            );
        }
        if let Some(raw_typed_characters_count) = self.raw_typed_characters_count {
            frame.render_widget(
                Paragraph::new(format!(
                    "Raw characters typed: {}",
                    raw_typed_characters_count
                )),
                areas[5],
            );
        }
        if let Some(accuracy) = self.accuracy {
            frame.render_widget(
                Paragraph::new(format!("Accuracy after corrections: {:.2}%", accuracy)),
                areas[6],
            );
        }
        if let Some(valid_characters_count) = self.valid_characters_count {
            frame.render_widget(
                Paragraph::new(format!(
                    "Valid characters after corrections: {}",
                    valid_characters_count
                )),
                areas[7],
            );
        }
        if let Some(mistakes_count) = self.mistakes_count {
            frame.render_widget(
                Paragraph::new(format!("Mistakes after corrections: {}", mistakes_count)),
                areas[8],
            );
        }

        if let Some(typed_characters_count) = self.typed_characters_count {
            frame.render_widget(
                Paragraph::new(format!(
                    "Characters typed after corrections: {}",
                    typed_characters_count,
                )),
                areas[9],
            );
        }
    }
}

fn get_results_dir_path() -> Result<PathBuf> {
    let dir_path = dirs::home_dir()
        .context("Unable to get home directory")?
        .join(".local")
        .join("share")
        .join("donkeytype");

    Ok(dir_path)
}

fn get_results_file_path() -> Result<PathBuf> {
    let dir_path = get_results_dir_path().context("Unable to get results directory path")?;
    let file_path = dir_path.join("donkeytype-results.csv");

    Ok(file_path)
}

fn create_results_dir_if_not_exist() -> Result<()> {
    let results_dir_path =
        get_results_dir_path().context("Unable to get results directory path")?;

    if !results_dir_path.exists() {
        create_dir_all(results_dir_path.clone())
            .context("Unable to create results directory for results file")?;
    }

    Ok(())
}

fn read_previous_results() -> Result<Vec<TestResults>> {
    let results_file_path = get_results_file_path().context("Unable to get results file path")?;

    let mut reader =
        csv::Reader::from_path(results_file_path.clone()).context("Unable to create CSV Reader")?;

    let records: Vec<TestResults> = reader
        .deserialize()
        .collect::<Result<_, csv::Error>>()
        .context("Unable to deserialize results")?;

    Ok(records)
}

fn fmt_num(number: u32) -> String {
    if number < 10 {
        format!("0{}", number)
    } else {
        format!("{}", number)
    }
}
