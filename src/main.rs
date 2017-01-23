/*
 * main.rs
 * This file is part of Oblique_Projection
 *
 * Copyright (C) 2017 - Lilith Wynter
 *
 * Oblique_Projection is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * Oblique_Projection is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Oblique_Projection. If not, see <http://www.gnu.org/licenses/>.
 */

#[macro_use]

extern crate image;
extern crate csv;

use std::fs::File;
use std::path::Path;

fn array_to_image(image: &mut Vec<[f64; 2]>) {
    println!("Creating 2D projection");
    
    let mut image_lengthx: i64 = 0;
    let mut image_lengthy: i64 = 0;
  
    // Find largest point in x and y direction to determine image size
    for i in image.iter_mut() {
      if i[0] < 0.0 {
        if i[0] as i64*-1 > image_lengthx {
          image_lengthx = i[0] as i64*-1;
        }
      } else {
        if i[0] as i64 > image_lengthx {
          image_lengthx = i[0] as i64;
        }
      }

      if i[1] < 0.0 {
        if i[1] as i64 * -1 > image_lengthy {
          image_lengthy = i[1] as i64*-1;
        }
      } else {
        if i[1] as i64 > image_lengthy {
          image_lengthy = i[1] as i64;
        }
      }
    } 
    
    let image = image;
    
    let mut done: bool = false;

    // Have image size a multiple of 10
    while !done {
      let mut changed: bool = false;
      if image_lengthx%10 != 0 {
        changed = true;
        image_lengthx+=1;
      } 
      if image_lengthy%10 != 0 {
        changed = true;
        image_lengthy+=1;
      }
      
      if changed == false {
        done = true;
      }
    }
    
    // Double the largest x and y point, so that 0, 0 can be in the center
    let sizex: u64 = image_lengthx as u64 * 2; 
    let sizey: u64 = image_lengthy as u64 * 2;
    
    println!("    image Width: {} Height: {}", sizex, sizey);
    
    let mut imgbuffer = image::ImageBuffer::new(sizex as u32, sizey as u32);
    
    for (x, y, pixel) in imgbuffer.enumerate_pixels_mut() {
      let crnt_px: i64 = (x as i64 - sizex as i64 /2) as i64;
      let crnt_py: i64 = (y as i64 - sizey as i64 /2) as i64;
      
      for i in image.iter() {

        if i[0] as i64 == crnt_px {
         if i[1] as i64 == crnt_py {
            // Place white pixel
            *pixel = image::Luma([255]);
          }
        }
      }
    }
    
    let ref mut fout = File::create(&Path::new("2DVisualisation.png")).unwrap();
    
    let _ = image::ImageLuma8(imgbuffer).save(fout, image::PNG);
    
    println!("Projection generated!!");
}

fn read_nd_data(file: &str) -> Vec<Vec<f64>> {
    let mut temp_array_data = Vec::new();
    
    // Read the csv file
    let mut rdr = csv::Reader::from_file(file).unwrap();
    
    for row in rdr.records().map(|r| r.unwrap()) {
      let data = row;
      temp_array_data.push(data);
    }

    let mut array_data = Vec::new();

    // Change vector from string to f64
    for i in 0..temp_array_data.len() {
      array_data.push(Vec::new());
      for j in 0..temp_array_data[i].len() {
        if temp_array_data[i][j].is_empty() {
          array_data[i].push(0.0 as f64);
        } else {
          array_data[i].push( temp_array_data[i][j].parse::<f64>().unwrap());
        }
      }
    }
    array_data
}

fn oblique_projection_from_nd(file: &str) {
  // Gather data from specified file
  let data = read_nd_data(file);

  // The standard angle in a oblique projection
  let angle: f64 = 63.4;
  
  let mut final_array = data;
  
  // Number of points in dataset 
  let num_points = final_array.len();
  
  // Number of dimensions
  let mut num_dim = final_array[0].len();
  
  // Apply oblique projection to the vector until it is 2 dimensional array
  while num_dim > 2 {
    let mut temp_data: Vec<Vec<f64>> = Vec::new(); 
    
    for i in 0..num_points {
      temp_data.push(Vec::new());
      for j in 0..num_dim-1 {
        if j%2 == 0 {
          temp_data[i].push(final_array[i][j] as f64 + 0.5*final_array[i][num_dim-1]*angle.cos() as f64);
        } else {
          temp_data[i].push(final_array[i][j] as f64 + 0.5*final_array[i][num_dim-1]*angle.sin() as f64);
        }
      }
    }
    num_dim-=1;

    final_array.clear();
    final_array = temp_data;
  }
  
  let mut largest_num: f64 = 0.0;
  for i in 0..final_array.len() {
    for j in 0..final_array[i].len() {
      if largest_num < final_array[i][j] {
        largest_num = final_array[i][j];
      }
    }
  }
  
  // Shrink or grow the results as to stop memory overloads 
  // Results in more resonable image output and size
  if largest_num > 10000.0 {
    let mut larger_than_1000: bool = true;
  
    while larger_than_1000 {
      let mut check: bool = false;
      for i in 0..final_array.len() {
        for j in 0..final_array[i].len() {
          if final_array[i][j] > 10000.0 {
            check = true;
          }
          final_array[i][j] /= 10.0;
        }
      }
      if check == false {
        larger_than_1000 = false;
      }
    }
  } else if largest_num < -10000.0 {
    let mut smaller_than_10000: bool = true;
    
    while smaller_than_10000 {
      let mut check: bool = false;
      for i in 0..final_array.len() {
        for j in 0..final_array[i].len() {
          if final_array[i][j] < -10000.0 {
            check = true;
          }
          final_array[i][j] *= 10.0;
          println!("{}", final_array[i][j]);
        }
      }
      if check == false {
        smaller_than_10000 = false;
      }
    }
  }
  
  let mut array_2d: Vec<[f64; 2]> = Vec::new();

  // Change array structure for imaging
  for i in 0..num_points {
    array_2d.push([final_array[i][0], final_array[i][1]]);
  }
  
  array_to_image(&mut array_2d);
}

fn main() {
  
  oblique_projection_from_nd("./data/DorotheaData.csv");
}
