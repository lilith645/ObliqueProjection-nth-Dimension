# Oblique_projection with nth dimension
Simple Oblique projection program

## Description



## Installation

Installing from GitHub:

    $ git clone https://github.com/lilith645/ObliqueProjection-nth-Dimension
    $ cd ObliqueProjection-nth-Dimension/
    $ cargo build

## Usage

Place your data in a csv file in data/ folder formated

'''
## Example csv file
 Leave 1st row empty
 
|    |    |    |

| -5 | -5 | -5 |

| -5 |  5 | -5 |

|  5 |  5 | -5 |

|  5 | -5 | -5 |

 etc..
 
'''

Within main.rs change the parameter of function call oblique_projection_from_nd 
in main to your dataset name.

Then build and run:

    $ cargo run

## Data Sets

Data sets used in examples

Dorothea data set from
    http://archive.ics.uci.edu/ml/machine-learning-databases/dorothea/DOROTHEA/

## Contributing
Feel free to contribute :) 

## License
Please see the included LICENSE
