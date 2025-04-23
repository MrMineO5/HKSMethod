use std::marker::PhantomData;
use crate::model::{Couplings, Model};
use crate::scanner::consumer::ScanConsumer;
use crate::simulation::{IntegrationParameters, IntegrationResult, Integrator};
use std::sync::mpsc::Sender;
use std::thread;
use indicatif::{ProgressBar, ProgressStyle};
use crate::scanner::scanner::Scanner;

pub type CouplingRanges<const N: usize> = [(f64, f64); N];

pub struct MultiThreadedScanner<M: Model<N> + Clone, T: ScanConsumer<N>, const N: usize> {
    coupling_ranges: CouplingRanges<N>,
    params: IntegrationParameters,
    model: M,
    pub consumer: T,
}
impl<M: Model<N> + Clone + Send + 'static, T: ScanConsumer<N> + Send + 'static, const N: usize> MultiThreadedScanner<M, T, N> {
    pub fn new(
        coupling_ranges: CouplingRanges<N>,
        params: IntegrationParameters,
        model: M,
        consumer: T,
    ) -> Self {
        Self {
            coupling_ranges,
            params,
            model,
            consumer,
        }
    }

    pub fn scan(
        &mut self,
        num_threads: usize,
        num_samples: u64,
    ) {
        let total_samples = num_threads as u64 * num_samples;

        let (tx, rx) = std::sync::mpsc::channel();

        println!(
            "Starting {} simulation threads with {} samples each",
            num_threads, num_samples
        );

        let mut join_handles: Vec<_> = (0..num_threads)
            .map(|_| {
                let tx = tx.clone();
                let mut send_consumer = self.consumer.clone();

                let coupling_ranges = self.coupling_ranges;
                let params = self.params.clone();
                let model = self.model.clone();

                thread::spawn(move || {
                    let mut scanner = Scanner::new(
                        coupling_ranges,
                        params,
                        Box::new(model),
                    );

                    scanner.scan(num_samples, &mut send_consumer, tx);

                    send_consumer
                })
            })
            .collect();

        println!("Simulating...");

        let sty = ProgressStyle::with_template(
            "[{elapsed_precise}] {percent:>2}% {bar:40.cyan/blue} {pos:>7}/{len:7} @ {per_sec} ({eta})",
        )
            .unwrap()
            .progress_chars("##-");
        let progress_bar = ProgressBar::new(total_samples);
        progress_bar.set_style(sty);

        while !join_handles.is_empty() {
            for i in (0..join_handles.len()).rev() {
                if join_handles[i].is_finished() {
                    // Join and remove the finished handle
                    let consumer = join_handles.swap_remove(i).join().unwrap();
                    self.consumer.merge(consumer);
                }
            }

            while let Ok(i) = rx.try_recv() {
                progress_bar.inc(i);
            }
        }
    }
}
