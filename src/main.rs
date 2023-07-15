use std::{thread, ops::Deref};
use tokio::{
    runtime::Runtime,
    sync::oneshot
};

#[derive(Debug, Clone, Copy)]
struct Frame(usize);
impl Deref for Frame {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
struct Canvas; // TODO


struct CanvasAndFrame {
    canvas: Canvas,
    frame_number: Frame,
}

struct State; // TODO

struct StartingPoint {
    canvas_and_frame: CanvasAndFrame,
    state: State,
}

const FRAMES_PER_SECOND: u32 = 30;
const FRAME_DURATION: f64 = 1.0 / (FRAMES_PER_SECOND as f64);

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

const COMPLETED_FRAMES_BUFFER: usize = 128;
const COMPLETED_FRAMES_CONSUMERS: usize = 8;

async fn advance_canvas_by_1_frame(canvas: &mut Canvas, state: &mut State) {
    todo!();
}

#[tokio::main]
async fn main() {
    let blank_canvas = Canvas; // todo!();

    // let starting_points = [];
    let starting_and_end_points = [
        (
            StartingPoint{ 
                canvas_and_frame: CanvasAndFrame {
                    canvas: blank_canvas.clone(),
                    frame_number: Frame(0),
                },
                state: State, // TODO
            },
            Frame(26),
        ),
    ];

    let (completed_frames_sender, completed_frames_receiver) = async_channel::bounded(COMPLETED_FRAMES_CONSUMERS);

    let mut frame_creator_threads = Vec::with_capacity(starting_and_end_points.len());

    for (starting_point, ending_frame) in starting_and_end_points {
        let (completed_tx, completed_rx) = oneshot::channel();

        frame_creator_threads.push(completed_rx);

        let completed_frames_sender_local = completed_frames_sender.clone();

        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            
            rt.block_on(async {
                let starting_canvas_and_frame = starting_point.canvas_and_frame;
                let starting_canvas = starting_canvas_and_frame.canvas;
                let starting_frame = starting_canvas_and_frame.frame_number;
                let starting_state = starting_point.state;

                let mut canvas = starting_canvas;
                let mut state = starting_state;

                for frame_number in starting_frame.0..ending_frame.0 {
                    let frame_number = Frame(frame_number);

                    advance_canvas_by_1_frame(&mut canvas, &mut state).await;

                    let completed_frame = CanvasAndFrame {
                        canvas: canvas.clone(),
                        frame_number,
                    };

                    completed_frames_sender_local.send(completed_frame).await.unwrap();
                }

                // TODO: what was I trying to write here??
                // let 
            });

            completed_tx.send(()).unwrap();
        });
    }
    drop(completed_frames_sender);


    let mut frame_saver_threads = Vec::with_capacity(COMPLETED_FRAMES_CONSUMERS);
    for _ in 0..COMPLETED_FRAMES_CONSUMERS {
        let (completed_tx, completed_rx) = oneshot::channel();

        frame_saver_threads.push(completed_rx);

        let completed_frames_receiver_local = completed_frames_receiver.clone();

        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            
            rt.block_on(async {
                while let Ok(completed_frame) = completed_frames_receiver_local.recv().await {
                    todo!(); // save it to the filesystem
                }
            });

            completed_tx.send(()).unwrap();
        });
    }
    drop(completed_frames_receiver);


    for thread in frame_creator_threads {
        thread.await.unwrap();
    }

    for thread in frame_saver_threads {
        thread.await.unwrap();
    }
}
