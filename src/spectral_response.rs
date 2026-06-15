use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::time::Instant;
use astroimsim_geometry::grid1d::{Location1D, Neighbors, GRID1D};


#[derive(Debug,Clone)]
pub struct SpectralResponseCurve {
    pub grid1d: GRID1D,
    pub data: Vec<(usize, f64)>, //a vector of values (point number on grid, data value at that point)
    pub label: &'static str,
    pub dat_path: &'static str,
}


impl SpectralResponseCurve {

    pub fn new(label:&'static str, grid1d: GRID1D,dat_path:&'static str)-> SpectralResponseCurve{
        let mut new = SpectralResponseCurve{
            grid1d,
            data:vec![],
            label,
            dat_path
        };
        new.load_data();
        new

    }

    pub fn get_data(&self, index: usize) -> f64 {
        assert_eq!(self.data[index].0, index);
        self.data[index].1.clone()
    }

    pub fn write_to_dat(&mut self,path:&str,header:&str){
        let mut file = File::create(path).expect("Could not create file");
        file.write_all(format!(
            "{:?} \n {:?} \n {:?} \n {:?} \n \n \n",
            self.label, self.dat_path, self.grid1d, header).as_bytes()).expect("Failed to write header");

        for (point,value) in &self.data{
            let location = self.grid1d.location(*point);
            file.write_all(format!("\n{:?}   {:?}", location, value).as_bytes()).unwrap();
        }

    }
    
    pub fn load_data(&mut self) {
        let delineator = "    ";
        assert_eq!(0, self.data.len(), "Loading data {:?} into would overwrite current data", self.label);
        println!("Loading {:?} into {:?}", self.dat_path, self.label);
        let start = Instant::now();
        let file = File::open(self.dat_path).expect("Failed to open file");
        let reader = BufReader::new(file);
        let mut data: Vec<Vec<f64>> = Vec::new();
        for line in reader.lines(){
            let mut parsable = true;
            let line = line.expect("Failed to read line");
            let line = line.trim()
                .split(&delineator)
                .map(|a|a.parse::<f64>())
                .map(|result|
                    match result {
                        Ok(value) => {println!("{:?}",value);value},
                        Err(_) => {parsable = false; 0.0},
                    }).collect();
            if parsable{
                data.push(line)
            }
        }
        println!("Parsed {:?} lines in {:?}", data.len(), start.elapsed().as_millis());
        assert_eq!(data.len(), self.grid1d.num(), "Retrieved a different number of records than expected");


        let mut same_num_data_points_per_record = true;
        for i in 0..data.len() - 1 {
            if data[i].len() - data[i + 1].len() != 0 {
                same_num_data_points_per_record = false;
            }
        }
        if !same_num_data_points_per_record {
            println!("Warning! There are different numbers of records for each line. This may mess up plotting or indicate a loading error. Will be plotting with {:?}", data[0].len() - 1)
        } //TODO run this function witha  "verbose" to list out the differences
 
        println!("The first record is {:?} and the last is {:?}, snapping to grid: {:?}", data[0], data[data.len() - 1], self.grid1d);
        let mut snapped_data = Vec::new();
        for values in data {
            let location = values[0];
            let data_point = values[1];
            let index = self.grid1d.snap(location as f64);
            snapped_data.push((index,data_point)) //TODO this must change to plot multiple
        }
        self.data = snapped_data;
    }

    pub fn re_grid(&mut self, new_grid: GRID1D) {
        assert!(new_grid.snap_precision <= self.grid1d.snap_precision, "Snap precision of new grid must be less than or equal to that of the original grid");
        self.data.sort_by_key(|x| x.0); //TODO move this into a validation function
        let mut new_data = Vec::new();
        for point in 0..new_grid.num() {
            let new_location = new_grid.location(point);
            let value = match self.grid1d.inside_or_outside(new_location) {
                Location1D::TooHigh => {
                    println!("new gridding {:?},location is {:?}, too high", point, new_location);
                    self.data[self.data.len() - 1].1.clone()

                }
                Location1D::TooLow => {
                    println!("new gridding {:?},location is {:?}, too low", point, new_location);
                    self.data[0].1.clone()
                }
                Location1D::JustRight => {
                    match self.grid1d.find_neighbors(new_location) {
                        Neighbors::Two(lower_index, upper_index) => {
                            println!("new gridding {:?},location is {:?}, just right, two neiborhs: {:?}", point, new_location, (lower_index, upper_index));
                            let lower = self.grid1d.location(lower_index);
                            let upper = self.grid1d.location(upper_index);
                            let lower_delta = new_location - lower;
                            let upper_delta = upper - new_location;
                            let lower_weight = lower_delta / (lower_delta + upper_delta);
                            let upper_weight = upper_delta / (lower_delta + upper_delta);
                            //let upper_data = self.get_data(upper_index);
                            // let lower_data = self.get_data(lower_index);

                            let upper_data = self.get_data(upper_index);

                            let lower_data = self.get_data(lower_index);
                            //assert_eq!(upper_data.units, lower_data.units,"Can not interpolate between points with data of different units. This shouldn't happen.... Something is suspicious with your grid.");


                            let new_value = upper_data*upper_weight+lower_data*lower_weight;//upper_data.iter().zip(lower_data.iter()).map(|(a, b)|
                               // a * upper_weight + b * lower_weight).collect();

                            new_value
                        }
                        Neighbors::One(snap) => { self.get_data(snap) }
                    }
                }
            };
            new_data.push((point, value))
        }
        let mut new_frequency_file = (*self).clone();
        new_frequency_file.data = new_data;
    }



}