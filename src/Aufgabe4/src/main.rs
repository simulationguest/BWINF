use std::{error::Error, mem::take};

#[derive(Clone)]
struct Task {
	start: u32,
	duration: u32,
	completed: bool,
}

fn show(wait_times: &[u32], name: &'static str) {
	let max = wait_times.iter().max().unwrap_or(&0);
	let avg = wait_times
		.iter()
		.sum::<u32>()
		.checked_div(wait_times.len() as u32)
		.unwrap_or(0);
	println!("Methode {name}: avg: {avg}, max: {max}");
}

fn main() -> Result<(), Box<dyn Error>> {
	// parse tasks file
	let file = shared::file_picker::to_string("Fahrradwerkstatt")?;
	let mut tasks: Vec<Task> = Vec::new();

	let pairs = shared::split_to_pairs::<u32>(&file);
	for (start, duration) in pairs {
		tasks.push(Task {
			completed: false,
			duration,
			start,
		});
	}

	simple(tasks.clone());
	better(tasks.clone());
	mixed(tasks.clone());

	Ok(())
}

fn better(mut tasks: Vec<Task>) {
	let mut time = 0;
	let mut wait_times: Vec<u32> = Vec::new();

	loop {
		let count = tasks.iter().filter(|task| !task.completed).count();

		if count == 0 {
			break;
		}

		let task = tasks
			.iter()
			.enumerate()
			.filter(|(_, task)| task.start < time && !task.completed)
			.min_by_key(|(_, task)| task.duration);

		match task {
			Some((index, task)) => {
				let task = task.clone();
				tasks[index].completed = true;
				time += task.duration;
				wait_times.push(time.checked_sub(task.start).unwrap_or(0));
			}
			None => {
				time += 1;
			}
		}
	}

	show(&wait_times, "Verfahren 2");
}

fn simple(tasks: Vec<Task>) {
	let mut wait_times: Vec<u32> = Vec::new();
	let mut tasks_iter = tasks.iter();
	let mut prev_task = tasks_iter.next().unwrap();
	for task in tasks_iter {
		let wait_time = task
			.start
			.checked_sub(prev_task.duration + prev_task.start)
			.unwrap_or(0);
		wait_times.push(wait_time);
		prev_task = task;
	}

	show(&wait_times, "Verfahren 1");
}

fn mixed(mut tasks: Vec<Task>) {
	let mut wait_times: Vec<u32> = Vec::new();

	let mut time = 0u32;

	let mut take_first = false;

	loop {
		let count = tasks.iter().filter(|task| !task.completed).count();
		if count == 0 {
			break;
		}

		take_first = !take_first;

		let mut available_tasks = tasks
			.iter()
			.enumerate()
			.filter(|(_, task)| task.start < time && !task.completed);

		let task;

		if take_first {
			task = available_tasks.next();
		} else {
			task = available_tasks.min_by_key(|(_, task)| task.duration);
		}

		match task {
			Some((index, task)) => {
				let task = task.clone();
				tasks[index].completed = true;
				time += task.duration;
				wait_times.push(time.checked_sub(task.start).unwrap_or(0));
			}
			None => {
				time += 1;
			}
		}
	}

	show(&wait_times, "Abwechselnd 1 und 2");
}
