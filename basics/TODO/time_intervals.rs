/// There's a room with a TV and people are coming in and out to watch it. 
/// The TV is on only when there's at least a person in the room.
/// For each person that comes in, we record the start and end time. 
/// We want to know for how long the TV has been on. 
/// In other words:Given a list of arrays of time intervals, 
/// write a function that calculates the total amount of time covered by the intervals.
///```rust
///input = [(1,4), (2,3)]> 3    input = [(4,6), (1,2)]> 3   
///input = [(1,4), (6,8), (2,4), (7,9), (10, 15)]> 11
///```

/// 时间段类型
type TimeIntervals = (i32, i32);

/// 合并有重叠的时间段
fn merge_intervals(input: &[TimeIntervals]) -> Vec<TimeIntervals> {
    let mut output: Vec<TimeIntervals> = Vec::new();
    let mut j = 0;
    let mut current_end = 0;
    for i in 0..input.len() {
        if input[i].1 > current_end {
            current_end = input[i].1;
        }
        println!("j {:?}", &j);
        if i == (input.len() - 1) || input[i+1].0 > current_end {
            output.push((input[j].0, current_end));
            j = i + 1; // 
        }
        println!("current_end {:?}", &current_end);   
    }
    output
}

/// 累加时间段
fn sum_time_interval(input: &TimeIntervals) -> i32 {
    input.1 - input.0
}

fn main() {
    //let mut input = [(1,4), (2,3)];
    //let mut input = [(4,6), (1,2)];
    let mut input = [(1,4), (6,8), (2,4), (7,9), (10,15), (2,6)];
    //let mut input = [(1,3), (2,6), (8,10), (15,18)];
    input.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    println!("sorted input {:?}", &input);
    
    let merged_input = merge_intervals(&input);
    println!("merged input {:?}", &merged_input);

    let mut sum = 0;
    for item in &merged_input {
        sum = sum + sum_time_interval(item);
    }
    println!("finally sum {:?}", sum);
}