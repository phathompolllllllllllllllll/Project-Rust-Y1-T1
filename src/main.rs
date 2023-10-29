use plotters::{prelude::*, style::full_palette::PINK};
use std::error::Error;
use std::fs::File;
extern crate csv;
use std::collections::HashMap;
use std::collections::HashSet;
use plotters::style::RGBColor;
use rand::prelude::*;


const OUT_FILE_NAME: &'static str = "plotters-doc-data/histogram.png";

fn read_csv_into_list(file_path: &str) -> Result<Vec<u32>, Box<dyn Error>> {

    let mut data = vec![];
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    if let Some(result) = rdr.records().next() {
        let record = result?;
        for field in record.iter() {
            let val: u32 = field.parse()?;
            data.push(val);
        }
    }
    Ok(data)
}


fn generate_random_colors(n: usize) -> Vec<RGBColor> {

    let mut colors = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..n {
        let r: u8 = rng.gen();
        let g: u8 = rng.gen();
        let b: u8 = rng.gen();
        let color = RGBColor(r, g, b);
        colors.push(color);
    }

    colors
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let agrs: Vec<String> = std::env::args().collect();

    if agrs.len() == 2 {
        println!("Please put another file name to read");
    }
    
    else if agrs.len() < 3 {
        println!("You are not put the two file names to read");

    } else {
        let arg_1 = &agrs[1];
        let arg_2 = &agrs[2];

   

    let data = read_csv_into_list(arg_1)?;
    let max_value = *data.iter().max().unwrap_or(&0);
    let max_frequency = data.iter().filter(|&&x| x == max_value).count() as u32;

    let root = BitMapBackend::new(OUT_FILE_NAME, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Bar graph", ("sans-serif", 40.0))
        .build_cartesian_2d((0..max_value).into_segmented(), 0u32..max_frequency)?;

    chart.configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Count Frequency")
        .x_desc("values")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;


    chart.draw_series(Histogram::vertical(&chart)
        .style(RED.mix(0.5).filled())
        .data(data.iter().map(|x| (*x, 1)))
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under the current dir");

    println!("Result has been saved to {}", OUT_FILE_NAME);


    let file_path = arg_2;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut age_data = Vec::new();
    let mut tip_data = Vec::new();

    for result in rdr.records() {
        let record = result?;
        if record.len() >= 2 {
            if let (Some(age_str), Some(tip_str)) = (record.get(0), record.get(1)) {
                if let (Ok(age_val), Ok(tip_val)) = (age_str.parse::<f64>(), tip_str.parse::<f64>()) {
                    age_data.push(age_val);
                    tip_data.push(tip_val);
                } else {
                    eprintln!("Error: parsing data");
                }
            } else {
                eprintln!("Error: Missing columns");
            }
        } else {
            eprintln!("Error: Not enough columns");
        }
}

// create scatter plot
    let root = BitMapBackend::new("plotters-doc-data/scatter_plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Scatter Plot", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(20.0..50.0, 0.0..10.0)?;// Adjust the x and y-axis ranges

    chart.configure_mesh().draw()?;
    chart.draw_series(
        age_data
            .iter()
            .zip(tip_data.iter())
            .map(|(&age, &tip)| {
                Circle::new((age, tip), 5, BLUE.filled())
            }),

    )?;

    chart.configure_series_labels().background_style(&WHITE.mix(0.8)).border_style(&BLACK);

    // Printa success message
    println!("Scatter plot created and saved as 'scatter_plot.png'");


    const OUT2_FILE_NAME: &'static str = "plotters-doc-data/pie-chart.png";

    let mut frequency_map: HashMap<u32, u32> = HashMap::new();

    for &num in &data {
        let count = frequency_map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut frequency_list: Vec<f64> = Vec::new();

    for num in 1..=10 {
        let frequency = *frequency_map.get(&num).unwrap_or(&0);
        frequency_list.push(frequency as f64);
    }


    let mut unique_list: Vec<u32> = Vec::new();
    let mut set: HashSet<u32> = HashSet::new();

    for &num in &data {
        if set.insert(num) {
            unique_list.push(num);
        }
    }


    let root_area = BitMapBackend::new(&OUT2_FILE_NAME, (950, 700)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let title_style = TextStyle::from(("sans-serif", 30).into_font()).color(&(BLACK));
    root_area.titled("Pie chart", title_style).unwrap();

    let dims = root_area.dim_in_pixel();
    let center = (dims.0 as i32 / 2, dims.1 as i32 / 2);
    let radius = 300.0;
    let sizes = frequency_list;
    // let _rgba = RGBAColor(0, 50, 255, 1.0);
    let colors = generate_random_colors(unique_list.len());
    let labels = &unique_list;
    let la: Vec<String> = labels
    .iter()
    .map(|num| format!("The frequency of {}", num))
    .collect();

    let mut pie = Pie::new(&center, &radius, &sizes, &colors, &la);
    pie.start_angle(66.0);
    pie.label_style((("sans-serif", 16).into_font()).color(&(PINK)));
    pie.percentages((("sans-serif", 20).into_font()).color(&BLACK));
    root_area.draw(&pie)?;
    println!("Pie chart created and saved as 'pie-chart.png'");
    }

    Ok(())
    
}



#[test]
fn entry_point() {
    main().unwrap()
}
