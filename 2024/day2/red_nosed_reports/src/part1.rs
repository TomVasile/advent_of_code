//7 6 4 2 1
//1 2 7 8 9
//9 7 6 2 1
//1 3 2 4 5
//8 6 4 4 1
//1 3 6 7 9

/* 
The levels are either all increasing or all decreasing.
Any two adjacent levels differ by at least one and at most three.

In the example above, the reports can be found safe or unsafe by checking those rules:

7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
*/

fn is_within_distance(abs_value: i32) -> bool {
    return match abs_value {
        1 | 2 | 3 => true,  // Match on 1, 2, or 3
        _ => false,          // Match anything else
    }
}

fn is_increasing(report: &Vec<i32>) -> bool {
    for level in report.windows(2) {
        if level[0] >= level[1] {
            return false;
        }
    }
    return true
}

fn is_decreasing(report: &Vec<i32>) -> bool {
    for level in report.windows(2) {
        if level[0] <= level[1] {
            return false;
        }
    }
    return true
}

fn main() {
    // Define a 2D vector (list of lists)
    // let matrix = vec![
    //     vec![7, 6, 4, 2, 1,],
    //     vec![1, 2, 7, 8, 9,],
    //     vec![9, 7, 6, 2, 1,],
    //     vec![1, 3, 2, 4, 5,],
    //     vec![8, 6, 4, 4, 1,],
    //     vec![1, 3, 6, 7, 9,],       
    // ];

    let matrix = vec![
        vec![74,76,78,79,76],
        vec![38,40,43,44,44],
        vec![1,2,4,6,8,9,13],
        vec![65,68,70,72,75,76,81],
        vec![89,91,92,95,93,94],
        vec![15,17,16,18,19,17],
        vec![46,47,45,48,51,52,52],
        vec![77,78,79,82,79,83],
        vec![55,58,59,57,60,61,68],
        vec![79,82,84,84,85,86],
        vec![23,24,25,25,23],
        vec![23,26,27,28,31,31,31],
        vec![42,43,43,44,48],
        vec![72,74,74,77,82],
        vec![85,86,87,88,89,93,94],
        vec![48,50,51,53,57,59,62,61],
        vec![29,32,34,38,38],
        vec![71,73,75,78,82,85,88,92],
        vec![1,4,7,8,12,18],
        vec![16,17,20,25,27,30,33],
        vec![49,52,55,62,63,66,68,67],
        vec![69,71,74,76,78,80,85,85],
        vec![34,36,39,42,48,49,53],
        vec![48,51,52,55,61,67],
        vec![55,54,57,60,61],
        vec![34,33,34,37,39,40,43,40],
        vec![23,20,23,24,26,29,29],
        vec![22,21,23,26,27,30,31,35],
        vec![79,76,78,81,82,89],
        vec![39,38,41,38,39,42],
        vec![47,45,48,50,51,52,50,48],
        vec![57,55,58,59,62,60,61,61],
        vec![94,92,89,91,92,96],
        vec![67,66,69,70,71,72,71,78],
        vec![41,39,42,45,45,46],
        vec![39,38,41,41,44,41],
        vec![21,20,22,23,26,26,29,29],
        vec![36,35,36,39,39,43],
        vec![60,57,58,58,63],
        vec![12,11,13,17,18],
        vec![49,46,47,49,52,56,53],
        vec![30,28,29,31,32,36,36],
        vec![81,80,82,86,89,91,95],
        vec![60,59,63,66,71],
        vec![71,68,69,75,76,78],
        vec![24,22,25,27,28,30,37,35],
        vec![6,5,12,14,17,20,20],
        vec![56,54,60,62,66],
        vec![82,81,88,89,94],
        vec![52,52,54,57,60,62,64],
        vec![3,3,4,6,9,12,10],
        vec![89,89,92,95,97,97],
        vec![67,67,69,72,75,78,82],
        vec![8,8,10,12,19],
        vec![10,10,12,13,10,11],
        vec![59,59,60,63,61,64,65,64],
        vec![68,68,70,71,70,70],
        vec![31,31,30,31,33,37],
        vec![63,63,60,63,68],
        vec![52,52,52,55,58,60,61,64],
        vec![34,34,37,39,42,42,43,41],
        vec![35,35,36,39,39,40,40],
        vec![69,69,69,72,73,74,78],
        vec![41,41,41,43,50],
        vec![27,27,30,33,37,40,42],
        vec![61,61,63,65,69,67],
        vec![3,3,5,7,11,14,14],
        vec![58,58,60,62,64,68,69,73],
        vec![21,21,25,26,29,35],
        vec![76,76,79,85,88],
        vec![49,49,50,53,55,60,58],
        vec![79,79,84,87,87],
        vec![63,63,64,66,67,72,74,78],
        vec![68,68,70,72,78,81,82,87],
        vec![43,47,49,50,53,56,59,61],
        vec![62,66,68,70,73,74,75,72],
        vec![21,25,26,29,31,31],
        vec![16,20,21,23,24,27,31],
        vec![23,27,30,31,33,40],
        vec![42,46,47,50,47,50],
        vec![76,80,81,83,85,84,83],
        vec![13,17,20,23,26,27,25,25],
        vec![8,12,9,10,13,14,15,19],
        vec![18,22,23,26,27,29,27,34],
        vec![42,46,46,48,49],
        vec![52,56,59,60,60,62,64,61],
        vec![77,81,82,82,84,84],
        vec![76,80,80,82,86],
        vec![53,57,60,62,63,64,64,70],
        vec![16,20,24,27,29,32],
        vec![51,55,57,61,63,65,62],
        vec![82,86,90,91,91],
        vec![61,65,66,69,73,77],
        vec![38,42,44,48,50,53,55,62],
        vec![13,17,19,25,27,28,31],
        vec![32,36,41,43,41],
        vec![63,67,68,71,77,77],
        vec![23,27,28,31,38,40,44],
        vec![45,49,50,55,56,58,59,64],
        vec![71,77,78,81,83],
        vec![26,33,35,36,37,35],
        vec![57,64,65,66,66],
        vec![11,16,18,20,22,24,25,29],
        vec![35,41,44,47,48,49,50,55],
        vec![52,59,60,57,60,63],
        vec![51,58,57,59,61,59],
        vec![4,10,8,9,12,13,15,15],
        vec![27,33,35,32,33,35,39],
        vec![20,26,25,26,27,29,31,38],
        vec![36,41,43,44,44,47],
        vec![48,54,57,60,61,61,63,62],
        vec![40,46,49,50,52,52,52],
        vec![13,20,22,22,25,29],
        vec![45,52,52,55,61],
        vec![73,78,79,82,85,88,92,95],
        vec![51,57,60,61,65,67,70,67],
        vec![6,12,16,19,20,22,23,23],
        vec![12,18,22,25,26,30],
        vec![56,61,65,66,72],
        vec![25,30,33,38,41,43],
        vec![64,70,76,79,77],
        vec![36,41,48,49,49],
        vec![59,64,66,68,74,77,81],
        vec![44,51,57,59,64],
        vec![87,86,83,82,81,78,77,80],
        vec![55,52,51,49,49],
        vec![46,43,41,39,38,37,33],
        vec![33,31,28,27,26,24,22,17],
        vec![83,80,78,77,79,76,73],
        vec![51,49,47,46,48,46,44,46],
        vec![56,55,52,55,52,52],
        vec![13,10,9,12,11,7],
        vec![36,33,31,34,27],
        vec![77,76,75,75,73,70,68],
        vec![93,91,91,89,88,86,83,86],
        vec![92,89,89,86,86],
        vec![51,48,48,46,42],
        vec![41,38,38,37,34,29],
        vec![51,50,48,46,42,40],
        vec![84,83,79,78,80],
        vec![19,16,13,11,9,5,5],
        vec![70,67,66,62,61,57],
        vec![73,71,70,69,68,64,58],
        vec![93,92,91,88,81,80,78],
        vec![56,54,52,50,45,46],
        vec![51,48,45,43,37,37],
        vec![82,80,74,71,67],
        vec![59,56,54,52,50,47,40,35],
        vec![17,20,19,18,16,13,10,7],
        vec![74,76,73,72,69,67,65,67],
        vec![95,96,93,92,92],
        vec![25,28,26,23,20,17,13],
        vec![43,45,42,40,38,32],
        vec![91,92,93,91,88],
        vec![41,44,42,39,38,39,42],
        vec![41,43,42,41,42,40,40],
        vec![63,66,68,67,63],
        vec![65,66,64,62,64,61,59,53],
        vec![84,85,82,82,81,78,77,75],
        vec![23,26,25,24,24,22,23],
        vec![30,31,29,27,25,25,25],
        vec![31,32,31,31,29,28,26,22],
        vec![12,15,15,13,6],
        vec![13,14,12,9,5,2],
        vec![10,12,10,6,8],
        vec![61,64,62,60,58,54,51,51],
        vec![92,93,89,86,82],
        vec![69,70,67,66,63,59,54],
        vec![80,81,79,77,74,68,66],
        vec![64,67,61,59,56,58],
        vec![65,67,60,57,56,54,52,52],
        vec![51,52,51,49,47,40,38,34],
        vec![51,52,51,49,43,36],
        vec![87,87,84,82,81,78,75,73],
        vec![37,37,36,35,33,36],
        vec![71,71,69,68,65,63,63],
        vec![16,16,15,13,9],
        vec![49,49,46,44,38],
        vec![64,64,67,65,63],
        vec![7,7,10,8,6,3,5],
        vec![30,30,27,24,21,19,21,21],
        vec![57,57,56,58,56,52],
        vec![79,79,82,79,74],
        vec![23,23,21,19,16,16,14,13],
        vec![91,91,91,88,87,84,83,84],
        vec![98,98,95,94,94,94],
        vec![93,93,91,88,88,87,83],
        vec![32,32,31,29,29,28,22],
        vec![33,33,31,30,28,24,23,22],
        vec![49,49,46,43,41,37,34,35],
        vec![65,65,61,58,58],
        vec![56,56,55,54,50,47,45,41],
        vec![17,17,16,12,5],
        vec![63,63,60,59,58,51,50,47],
        vec![48,48,43,42,41,40,37,39],
        vec![90,90,84,83,83],
        vec![48,48,42,40,39,35],
        vec![50,50,47,44,41,36,34,29],
        vec![34,30,27,24,23],
        vec![84,80,79,78,80],
        vec![79,75,73,71,69,67,64,64],
        vec![16,12,11,9,8,4],
        vec![30,26,25,23,22,21,20,15],
        vec![13,9,11,8,7],
        vec![26,22,21,20,19,20,21],
        vec![27,23,25,22,20,18,18],
        vec![37,33,31,32,29,25],
        vec![45,41,38,37,34,32,35,28],
        vec![56,52,50,50,48],
        vec![19,15,13,10,10,8,11],
        vec![18,14,14,12,12],
        vec![81,77,76,76,72],
        vec![81,77,77,74,68],
        vec![43,39,38,36,32,31,28,27],
        vec![98,94,93,89,92],
        vec![64,60,56,53,52,51,51],
        vec![93,89,88,85,81,77],
        vec![31,27,23,21,18,17,16,9],
        vec![84,80,79,77,75,69,68],
        vec![80,76,73,68,65,68],
        vec![97,93,86,83,81,81],
        vec![47,43,38,36,35,34,33,29],
        vec![69,65,63,58,53],
        vec![37,30,27,24,22,19,16],
        vec![95,88,85,84,83,85],
        vec![22,17,14,11,11],
        vec![93,88,87,86,83,81,80,76],
        vec![86,81,80,77,76,73,66],
        vec![96,91,89,90,89,87],
        vec![84,77,75,78,76,73,76],
        vec![12,7,9,7,7],
        vec![99,94,92,94,92,90,87,83],
        vec![32,25,23,20,17,16,17,10],
        vec![94,89,88,88,86,84,83,80],
        vec![56,51,51,50,49,47,46,48],
        vec![89,84,82,80,80,77,77],
        vec![71,64,63,63,59],
        vec![71,64,62,62,55],
        vec![41,35,32,29,25,22,19],
        vec![76,69,67,65,64,60,58,60],
        vec![50,44,43,41,40,36,36],
        vec![43,37,33,32,28],
        vec![33,27,23,21,18,15,14,9],
        vec![60,53,47,46,45],
        vec![52,45,39,37,34,35],
        vec![20,13,10,8,3,3],
        vec![47,40,38,32,30,28,26,22],
        vec![72,65,58,55,50],
        vec![9,10,11,14,17,19,17],
        vec![47,49,51,54,57,57],
        vec![38,40,41,43,45,49],
        vec![39,40,41,43,44,45,52],
        vec![18,20,18,21,24,25,27,30],
        vec![86,89,87,88,90,93,94,91],
        vec![85,86,88,90,88,90,93,93],
        vec![51,53,54,56,54,58],
        vec![27,30,28,31,32,37],
        vec![57,60,63,63,65],
        vec![52,54,54,56,55],
        vec![25,27,30,30,30],
        vec![69,70,73,74,74,76,80],
        vec![15,16,18,18,19,21,28],
        vec![23,24,27,31,33],
        vec![24,27,30,34,36,37,35],
        vec![50,52,54,56,59,63,66,66],
        vec![78,79,83,84,87,88,92],
        vec![17,18,20,21,25,31],
        vec![39,42,44,51,54,56,58],
        vec![10,13,20,23,20],
        vec![8,9,10,15,18,18],
        vec![49,50,56,58,60,62,65,69],
        vec![11,14,21,22,29],
        vec![88,87,90,93,94,96],
        vec![90,87,88,90,87],
        vec![48,45,47,48,48],
        vec![37,36,39,41,42,45,46,50],
        vec![3,1,2,3,4,10],
        vec![77,76,75,78,81,82],
        vec![38,35,33,34,31],
        vec![41,38,41,44,42,42],
        vec![13,12,10,12,16],
        vec![19,17,14,15,16,22],
        vec![14,12,14,15,15,18,20,22],
        vec![29,27,30,30,31,30],
        vec![60,57,59,59,59],
        vec![78,76,77,80,80,82,86],
        vec![57,54,57,57,63],
        vec![36,35,37,40,44,46],
        vec![42,40,41,44,48,45],
        vec![74,72,74,77,78,82,82],
        vec![12,11,12,16,20],
        vec![79,77,78,81,83,86,90,95],
        vec![39,37,43,46,48],
        vec![53,51,57,58,59,58],
        vec![23,20,21,26,26],
        vec![77,74,81,84,87,91],
        vec![66,63,69,72,79],
        vec![27,27,30,31,33,35,36,38],
        vec![30,30,33,34,36,38,41,40],
        vec![56,56,57,58,58],
        vec![75,75,76,79,83],
        vec![33,33,34,35,38,40,42,49],
        vec![8,8,11,14,12,14,16,18],
        vec![36,36,39,41,42,44,42,40],
        vec![48,48,50,51,50,52,52],
        vec![20,20,18,19,22,25,29],
        vec![54,54,53,56,62],
        vec![68,68,68,70,72,74,76,78],
        vec![22,22,22,24,25,28,27],
        vec![93,93,93,95,95],
        vec![70,70,70,72,74,77,79,83],
        vec![79,79,80,82,82,83,85,92],
        vec![52,52,56,59,61],
        vec![1,1,4,7,10,12,16,15],
        vec![41,41,43,47,47],
        vec![39,39,43,44,46,48,49,53],
        vec![38,38,41,45,50],
        vec![71,71,72,75,81,83,84,87],
        vec![79,79,85,87,88,86],
        vec![29,29,31,37,37],
        vec![41,41,43,46,48,51,57,61],
        vec![62,62,63,65,70,71,74,79],
        vec![39,43,46,49,50,52],
        vec![37,41,42,45,48,47],
        vec![4,8,9,12,14,14],
        vec![26,30,31,34,36,37,39,43],
        vec![1,5,7,9,12,14,15,20],
        vec![18,22,25,27,24,26,28],
        vec![35,39,40,42,43,41,44,43],
        vec![69,73,71,73,74,76,76],
        vec![6,10,12,13,12,14,16,20],
        vec![77,81,82,81,84,89],
        vec![37,41,41,44,46,48,49,50],
        vec![47,51,52,53,53,56,54],
        vec![7,11,12,13,15,15,16,16],
        vec![51,55,56,58,61,63,63,67],
        vec![55,59,59,60,62,64,66,71],
        vec![47,51,52,55,57,58,62,64],
        vec![23,27,31,32,34,36,34],
        vec![75,79,83,84,87,87],
        vec![20,24,25,27,31,35],
        vec![62,66,70,72,78],
        vec![4,8,9,11,18,20,23],
        vec![65,69,71,76,77,79,78],
        vec![43,47,48,53,56,57,59,59],
        vec![36,40,43,46,51,55],
        vec![14,18,21,24,29,34],
        vec![44,50,51,54,56],
        vec![1,6,9,10,11,14,13],
        vec![24,30,31,34,37,38,41,41],
        vec![11,18,21,22,26],
        vec![50,56,59,60,62,68],
        vec![21,26,25,27,30],
        vec![37,44,41,44,41],
        vec![61,66,67,70,68,68],
        vec![89,94,91,92,94,95,99],
        vec![53,60,63,61,62,68],
        vec![8,15,16,16,17,18],
        vec![64,70,70,73,76,75],
        vec![74,81,81,83,83],
        vec![10,16,19,19,22,26],
        vec![57,63,63,64,70],
        vec![53,59,62,63,66,70,73],
        vec![6,13,17,19,17],
        vec![16,22,24,28,29,31,31],
        vec![65,70,72,76,80],
        vec![56,63,64,67,71,73,80],
        vec![65,72,74,76,78,84,87],
        vec![37,42,47,49,50,48],
        vec![63,69,72,77,78,79,79],
        vec![67,73,76,83,87],
        vec![59,66,68,73,79],
        vec![35,33,32,29,31],
        vec![29,26,25,24,22,20,20],
        vec![53,52,51,48,44],
        vec![42,39,37,34,33,31,26],
        vec![14,12,9,8,5,3,4,1],
        vec![89,87,89,88,86,88],
        vec![50,47,48,45,42,40,39,39],
        vec![22,20,17,20,17,13],
        vec![86,84,87,85,82,75],
        vec![86,84,82,81,80,80,78],
        vec![20,19,17,15,13,10,10,13],
        vec![14,13,12,12,12],
        vec![85,82,82,79,77,74,71,67],
        vec![77,74,74,72,65],
        vec![21,18,15,12,10,6,5],
        vec![22,20,19,17,13,10,7,9],
        vec![61,60,57,56,52,52],
        vec![74,72,69,65,63,60,59,55],
        vec![74,73,71,70,66,63,61,55],
        vec![79,78,75,68,66,65,63],
        vec![21,19,16,13,6,7],
        vec![91,89,82,81,78,77,74,74],
        vec![57,55,54,52,47,43],
        vec![82,79,78,75,68,66,60],
        vec![79,82,80,78,76],
        vec![89,91,90,89,88,89],
        vec![22,23,21,20,20],
        vec![90,92,90,89,85],
        vec![86,89,86,85,84,79],
        vec![80,83,85,82,80],
        vec![59,60,58,55,57,59],
        vec![59,60,61,59,58,57,57],
        vec![28,30,28,29,27,23],
        vec![5,7,5,8,1],
        vec![12,14,14,11,9,7],
        vec![17,20,17,17,18],
        vec![27,30,30,27,24,24],
        vec![24,27,26,26,24,22,18],
        vec![21,23,21,18,15,15,8],
        vec![33,35,31,28,27,24,21],
        vec![60,62,58,57,58],
        vec![9,11,8,7,5,1,1],
        vec![51,54,50,47,43],
        vec![61,63,60,58,56,52,50,45],
        vec![90,91,88,81,79,78,77],
        vec![53,56,55,48,45,48],
        vec![31,32,31,26,24,21,18,18],
        vec![31,34,32,25,23,19],
        vec![17,18,11,8,3],
        vec![95,95,93,90,87,84,81,80],
        vec![92,92,91,89,87,85,87],
        vec![64,64,62,61,58,58],
        vec![14,14,11,8,6,2],
        vec![81,81,80,78,72],
        vec![24,24,22,20,23,20],
        vec![73,73,70,72,70,73],
        vec![50,50,49,46,48,48],
        vec![28,28,26,28,24],
        vec![81,81,84,81,75],
        vec![91,91,89,89,87,85,82],
        vec![32,32,32,30,28,29],
        vec![33,33,32,29,26,26,26],
        vec![82,82,82,81,77],
        vec![56,56,55,52,52,50,47,42],
        vec![29,29,25,23,22,19,16],
        vec![55,55,52,49,45,42,44],
        vec![85,85,81,78,78],
        vec![19,19,16,13,9,8,5,1],
        vec![29,29,26,24,20,19,14],
        vec![99,99,96,90,87,84],
        vec![27,27,21,18,15,18],
        vec![50,50,43,40,40],
        vec![65,65,63,62,61,56,52],
        vec![37,37,30,29,23],
        vec![82,78,77,76,73,72,71],
        vec![58,54,52,51,48,47,44,47],
        vec![29,25,22,21,18,17,15,15],
        vec![67,63,62,61,59,56,52],
        vec![38,34,32,29,22],
        vec![86,82,81,79,81,80,78,76],
        vec![49,45,46,44,46],
        vec![94,90,92,89,88,88],
        vec![39,35,36,35,34,31,30,26],
        vec![84,80,77,76,79,78,76,71],
        vec![16,12,9,9,8,5],
        vec![57,53,51,51,52],
        vec![26,22,22,19,16,14,13,13],
        vec![62,58,57,57,56,53,49],
        vec![54,50,48,45,45,42,35],
        vec![70,66,62,60,58,55,53],
        vec![45,41,37,36,33,32,34],
        vec![39,35,32,28,26,23,23],
        vec![94,90,86,83,82,79,75],
        vec![73,69,65,63,62,55],
        vec![98,94,93,90,83,80],
        vec![80,76,74,73,71,66,65,66],
        vec![75,71,68,65,58,55,55],
        vec![21,17,15,14,9,8,5,1],
        vec![72,68,62,59,58,56,50],
        vec![89,83,81,79,78,77,75],
        vec![67,60,57,54,52,50,49,51],
        vec![74,68,67,64,61,61],
        vec![99,94,91,90,89,85],
        vec![68,63,62,61,60,57,54,47],
        vec![71,65,63,60,61,58,56,53],
        vec![43,36,33,32,33,35],
        vec![20,15,14,16,16],
        vec![23,18,20,18,17,13],
        vec![32,27,24,23,26,25,19],
        vec![69,64,64,61,60,57,54,51],
        vec![27,20,17,17,19],
        vec![80,75,75,72,69,66,66],
        vec![99,92,91,90,90,89,85],
        vec![74,68,68,65,63,56],
        vec![64,59,56,53,52,49,45,44],
        vec![94,89,86,84,80,83],
        vec![34,27,23,20,19,18,18],
        vec![41,34,32,28,27,26,24,20],
        vec![70,65,64,63,61,57,51],
        vec![50,43,42,40,39,34,33,32],
        vec![79,72,67,65,66],
        vec![76,70,68,67,62,59,59],
        vec![19,12,7,6,2],
        vec![35,29,26,25,18,13],
        vec![32,30,29,28,25,25],
        vec![65,68,70,73,75,77,80,84],
        vec![80,80,77,80,82],
        vec![88,90,88,86,83,80,80,76],
        vec![70,72,75,73,74],
        vec![14,10,9,6,4,3,2,1],
        vec![84,85,84,82,79,79,78],
        vec![62,60,61,63,64,63,64,66],
        vec![85,89,91,94,94,96],
        vec![22,22,26,27,28,28],
        vec![46,46,44,43,39,37,32],
        vec![43,49,50,53,52,54,58],
        vec![65,66,65,61,56],
        vec![31,35,41,42,39],
        vec![32,29,26,24,21,20,16,16],
        vec![19,14,11,10,9,5],
        vec![16,20,23,30,30],
        vec![1,1,3,7,10],
        vec![24,22,20,23,20,16],
        vec![15,14,16,19,20,21,20],
        vec![53,53,49,48,51],
        vec![57,57,54,51,48,50],
        vec![89,85,85,84,80],
        vec![57,55,50,47,46,42],
        vec![26,29,28,25,25,23,17],
        vec![16,18,16,11,10],
        vec![6,13,13,16,21],
        vec![88,88,90,91,93,93,95,92],
        vec![20,16,13,11,7],
        vec![26,33,35,36,37,40,39],
        vec![30,25,23,21,19,17,19,13],
        vec![26,26,29,32,33,40,42,46],
        vec![69,63,59,57,55,50],
        vec![90,93,90,89,86,86],
        vec![74,78,81,84,88],
        vec![39,40,40,42,46],
        vec![87,84,84,83,81,79],
        vec![77,72,68,65,64,62],
        vec![41,41,39,42,44],
        vec![82,82,85,92,98],
        vec![11,11,8,6,4,4],
        vec![89,83,82,84,83,80],
        vec![5,3,7,8,11,8],
        vec![36,41,44,44,48],
        vec![26,19,17,15,13,12,10,7],
        vec![52,49,50,49,48,47],
        vec![26,32,32,33,33],
        vec![38,42,45,46,46,46],
        vec![66,69,66,63,64],
        vec![37,42,43,44,47,48,51],
        vec![75,75,73,70,66,62],
        vec![97,93,86,85,85],
        vec![63,63,66,69,71,73,75,82],
        vec![20,16,15,13,13],
        vec![77,77,76,73,73,71,70],
        vec![12,16,17,15,16,21],
        vec![68,65,58,57,57],
        vec![56,56,60,63,60],
        vec![16,16,19,16,14,13,10,7],
        vec![59,55,54,50,53],
        vec![43,39,33,31,30],
        vec![79,80,78,78,79],
        vec![49,47,46,44,41,41,41],
        vec![23,25,23,21,20,17,19,19],
        vec![28,35,38,41,42,42,44],
        vec![56,60,62,60,64],
        vec![18,18,20,19,19],
        vec![75,74,78,79,80,83,83],
        vec![58,55,53,47,44,45],
        vec![24,24,23,21,17,16,16],
        vec![27,20,19,15,12,11,8,9],
        vec![59,56,53,51,49,47,43,39],
        vec![64,64,62,61,54,52,55],
        vec![66,66,71,74,75,77,78,75],
        vec![77,77,76,69,66,64,57],
        vec![30,28,29,32,38,40],
        vec![72,71,72,73,75,76,77,77],
        vec![81,80,83,81,83],
        vec![40,40,39,37,36,35,34,32],
        vec![52,52,55,54,56,59,65],
        vec![18,15,18,19,21,28],
        vec![48,46,49,47,48,48],
        vec![51,47,45,43,45],
        vec![16,19,19,20,22,25,27,29],
        vec![8,12,11,14,17,20],
        vec![77,82,85,86,85,88,94],
        vec![47,44,49,52,53,57],
        vec![64,60,55,53,46],
        vec![93,92,89,87,85,81,79,80],
        vec![95,92,93,96,94,97,95],
        vec![93,91,91,90,86],
        vec![21,15,13,11,12,14],
        vec![25,26,25,22,21,18,14],
        vec![41,41,39,38,33,29],
        vec![18,22,26,29,27],
        vec![88,86,89,91,91,98],
        vec![55,55,55,53,51,49,49],
        vec![32,35,32,33,30],
        vec![61,60,60,57,56,51],
        vec![12,11,14,13,15,19],
        vec![68,64,64,62,61,61],
        vec![20,27,30,36,39,41],
        vec![17,17,19,23,24,26,33],
        vec![51,49,50,51,55,56,60],
        vec![21,16,15,12,11,8,11],
        vec![36,35,38,43,44,44],
        vec![18,18,20,26,26],
        vec![24,29,31,34,34,36,33],
        vec![75,76,74,72,71,71,71],
        vec![88,88,87,85,80,78],
        vec![74,73,72,68,67,65,64,57],
        vec![74,67,67,64,63,58],
        vec![71,70,73,74,77],
        vec![69,63,60,57,55,50],
        vec![3,5,11,12,14,19],
        vec![64,68,70,71,69],
        vec![40,36,35,38,36,34],
        vec![38,39,35,32,31,30,27,24],
        vec![59,58,60,63,67,68,75],
        vec![80,78,77,76,77,77],
        vec![72,79,81,82,84,90,92,91],
        vec![46,39,39,37,38],
        vec![10,17,18,19,21,21],
        vec![67,73,74,73,74,72],
        vec![57,57,55,58,54],
        vec![57,59,61,61,63,66,65],
        vec![6,10,10,11,14,19],
        vec![6,7,10,11,15,16,19,19],
        vec![60,62,63,66,73],
        vec![26,19,19,16,14,14],
        vec![28,32,39,42,44,46],
        vec![78,84,85,92,96],
        vec![52,52,56,58,60,64],
        vec![46,44,50,53,59],
        vec![56,52,49,47,46,42,39,36],
        vec![32,32,34,35,36,38,41,38],
        vec![38,32,29,23,21],
        vec![70,71,76,79,80,82,83,87],
        vec![40,36,35,32,34,31,32],
        vec![51,57,60,64,65,67,70],
        vec![31,31,32,33,32,33,33],
        vec![37,38,36,30,26],
        vec![71,67,65,60,58,55,51],
        vec![84,80,80,77,74,75],
        vec![32,38,35,36,39,42],
        vec![60,60,62,63,63,66,73],
        vec![42,42,43,46,49],
        vec![58,59,56,53,52,50,47],
        vec![67,67,64,58,56,53,52,52],
        vec![42,48,50,54,58],
        vec![80,83,78,77,76,75,73,67],
        vec![77,78,74,71,72],
        vec![48,50,52,54,61,63,65,66],
        vec![46,50,51,54,54,56,54],
        vec![20,24,25,28,31,38],
        vec![46,48,49,50,53,53],
        vec![21,18,20,21,25,26,29],
        vec![54,60,64,67,69,71,71],
        vec![67,60,53,51,50,49,48,48],
        vec![44,45,45,46,47,48,50,57],
        vec![55,58,61,61,62,64,65,65],
        vec![49,48,48,51,52,54,55,59],
        vec![38,34,32,28,24],
        vec![78,82,85,87,89,92,94,95],
        vec![35,31,29,28,31,27],
        vec![71,72,73,70,70],
        vec![54,50,46,44,43,37],
        vec![29,22,17,16,17],
        vec![58,59,60,64,67,71],
        vec![21,25,26,27,27],
        vec![19,23,25,28,29,26,29,27],
        vec![77,77,79,81,81,82,82],
        vec![34,34,32,29,27,25,21,20],
        vec![22,29,31,33,36,34,37,37],
        vec![14,20,23,26,33],
        vec![40,37,34,30,29],
        vec![85,89,91,90,92,92],
        vec![65,64,67,68,70,72,73,77],
        vec![63,63,66,71,72,73],
        vec![15,13,16,16,19,22],
        vec![11,13,9,7,7],
        vec![88,88,88,89,90,93,94],
        vec![89,89,88,88,86,85,84,87],
        vec![2,1,2,3,3,5,5],
        vec![61,63,70,72,73,73],
        vec![53,48,47,50,48,47,46,42],
        vec![37,41,44,48,53],
        vec![47,50,48,44,43,40,36],
        vec![19,24,25,31,37],
        vec![70,70,69,67,65,65,61],
        vec![38,44,45,48,54,57,58,58],
        vec![54,49,46,44,42,38,38],
        vec![81,82,84,86,91,92,91],
        vec![15,13,12,10,9,6,2],
        vec![51,56,59,60,61,65],
        vec![44,45,44,41,34],
        vec![75,77,74,75,77,76],
        vec![51,45,44,41,39,37,37],
        vec![19,23,27,29,30,30],
        vec![89,85,83,81,80,79,73],
        vec![18,19,12,10,9,7,5,6],
        vec![41,40,34,32,29,28],
        vec![88,84,80,78,77,76,76],
        vec![86,89,86,85,83,85,83,79],
        vec![43,43,44,43,44,45,48,52],
        vec![74,68,62,61,60,57,55,49],
        vec![60,60,63,63,67],
        vec![13,17,19,21,27,31],
        vec![41,41,39,39,36,29],
        vec![85,85,82,79,72],
        vec![61,59,57,54,53,55],
        vec![59,55,54,54,53,50],
        vec![80,79,79,77,74,76],
        vec![24,21,23,29,28],
        vec![1,7,8,10,14,13],
        vec![95,95,94,97,94,89],
        vec![79,77,75,74,67,64,58],
        vec![5,6,8,5,7,8,10,16],
        vec![44,41,39,37,36,35,28],
        vec![64,67,71,74,75,78,77],
        vec![49,49,48,47,45,42,38],
        vec![63,59,58,58,56,51],
        vec![26,26,25,28,30,29],
        vec![77,77,79,82,82],
        vec![11,18,19,23,30],
        vec![17,14,11,13,8],
        vec![98,92,91,90,87,85,81,77],
        vec![65,62,65,63,65,72],
        vec![11,15,16,21,28],
        vec![95,93,95,95,96,98,97],
        vec![87,80,79,76,75,75,72],
        vec![66,67,66,59,58,55,55],
        vec![24,19,13,10,9,6,5,1],
        vec![4,8,8,11,15],
        vec![72,73,76,74,73,68],
        vec![95,91,89,87,90,90],
        vec![1,2,5,3,4,7],
        vec![60,55,52,52,49,45],
        vec![62,55,54,51,52,52],
        vec![7,11,13,15,17,21,25],
        vec![2,4,5,7,11,12,14],
        vec![60,62,63,64,67,68,72,79],
        vec![8,8,9,10,11,13,17],
        vec![83,79,77,78,75,70],
        vec![73,69,62,61,59,57,60],
        vec![75,77,80,83,82],
        vec![47,50,51,52,51,55],
        vec![95,93,91,89,86,85,84,81],
        vec![96,95,92,91,88,85,82],
        vec![14,15,16,17,20,21,23],
        vec![3,4,7,8,10,11,13,15],
        vec![43,45,48,51,53,56,59,62],
        vec![67,66,65,64,63,62],
        vec![34,31,30,28,25],
        vec![64,61,60,59,58,56,55],
        vec![2,5,8,9,11,13,15],
        vec![48,51,52,54,55,58,60,61],
        vec![33,34,35,36,38],
        vec![12,14,17,20,23,24,25],
        vec![4,7,9,11,13,16,19,22],
        vec![34,32,31,28,26,24,21,19],
        vec![58,57,56,55,54,53],
        vec![72,73,76,78,81,84,85],
        vec![66,68,70,73,75,76,79],
        vec![40,42,44,46,47],
        vec![13,15,18,21,22,25],
        vec![49,51,52,55,56,57],
        vec![49,50,52,53,54,57,59],
        vec![24,27,28,30,31],
        vec![4,5,7,10,12,13,14,15],
        vec![59,58,56,55,54],
        vec![18,16,14,13,12],
        vec![1,4,6,9,12,15,16],
        vec![66,64,63,62,59,57,54,53],
        vec![90,88,86,85,83,82,80],
        vec![67,70,72,74,76,79,82,85],
        vec![57,56,54,52,49,48],
        vec![11,14,16,18,20,23,25],
        vec![50,53,55,57,59,62,64],
        vec![77,78,81,82,84,86],
        vec![67,70,72,74,76,79,81,84],
        vec![70,69,67,65,64,62,60,58],
        vec![6,9,12,14,17,18,21,23],
        vec![72,74,76,77,78,81,83,86],
        vec![43,40,39,36,34,32,29],
        vec![49,47,46,44,43,40],
        vec![65,62,60,57,56],
        vec![71,73,76,79,80],
        vec![74,71,69,68,66],
        vec![21,22,25,28,31],
        vec![85,84,82,81,78],
        vec![42,44,46,47,48,50,51,54],
        vec![54,53,51,50,48,47,44,42],
        vec![95,92,90,89,86],
        vec![80,81,82,84,85,86],
        vec![59,58,55,54,51],
        vec![58,57,56,55,52],
        vec![32,35,36,38,41],
        vec![30,28,26,23,22,19],
        vec![36,33,31,30,29,26,25,23],
        vec![53,55,56,58,59,61,62],
        vec![40,42,44,47,49],
        vec![49,52,54,57,58],
        vec![71,74,77,78,81,83],
        vec![18,16,14,12,10,8,7,4],
        vec![28,31,34,37,38,41],
        vec![5,8,11,14,16,19],
        vec![10,13,16,19,22,24],
        vec![77,79,82,83,85,86,88,90],
        vec![49,46,43,40,38,37],
        vec![7,8,10,12,13,15,18,19],
        vec![69,70,71,72,73,76],
        vec![52,51,48,47,45,42,40,39],
        vec![19,21,23,24,27],
        vec![47,44,42,39,36,34],
        vec![35,36,37,38,40,42,44,46],
        vec![70,73,75,78,79,82,84,85],
        vec![33,34,35,36,39,40,42],
        vec![71,73,76,78,80,83,85],
        vec![77,75,74,73,71,68],
        vec![84,82,81,78,76,75],
        vec![32,31,29,28,26,23,22],
        vec![34,31,30,28,26],
        vec![83,82,81,80,77,74,72],
        vec![81,79,78,77,75,74,71],
        vec![9,12,14,16,18,19,21],
        vec![30,29,28,25,24],
        vec![46,47,48,50,52,54,57,59],
        vec![27,25,22,19,16,14],
        vec![50,48,47,44,42],
        vec![27,24,22,21,19,18,17,15],
        vec![24,25,27,30,32,35],
        vec![54,57,59,62,65,67,69],
        vec![90,88,85,83,82,81,79,76],
        vec![52,50,47,44,43,42,40,39],
        vec![53,56,59,61,63,65,68,70],
        vec![75,74,72,70,69],
        vec![71,74,75,77,78,79,82],
        vec![11,14,15,17,20,22,25,26],
        vec![23,21,19,17,16,14,12],
        vec![83,84,86,89,92,93],
        vec![83,80,78,77,74,71,68,67],
        vec![51,52,55,57,60,63,64,66],
        vec![60,62,64,65,68],
        vec![70,68,65,63,62,60,58],
        vec![82,83,85,86,89,90,92,93],
        vec![62,65,68,70,72,75,76,79],
        vec![55,57,60,63,66,67,70,71],
        vec![23,26,29,31,34,35,36],
        vec![46,48,49,52,53,56],
        vec![44,41,38,35,33,31,28],
        vec![74,75,77,80,82,83],
        vec![27,30,32,33,36,39,40,43],
        vec![39,42,43,46,49,52,55,58],
        vec![29,27,25,22,19],
        vec![31,34,35,38,39,40],
        vec![57,56,53,51,50,47],
        vec![36,34,33,31,29],
        vec![76,77,78,79,80,83,84,86],
        vec![69,70,73,76,79,82,84],
        vec![84,87,90,92,93,94],
        vec![79,78,77,74,71,68,67],
        vec![50,51,53,55,57,60],
        vec![43,46,49,52,53],
        vec![87,86,84,83,82,79,76,74],
        vec![18,15,12,11,8,7,6,4],
        vec![87,85,84,83,82],
        vec![30,33,35,38,39],
        vec![76,74,72,71,68,67,64],
        vec![56,57,59,62,63,65],
        vec![47,49,52,53,55,56,58,61],
        vec![63,66,67,69,70],
        vec![51,52,54,56,58,61],
        vec![73,75,78,81,82,84,86,89],
        vec![50,51,54,57,58,60,62],
        vec![92,89,86,84,82,79],
        vec![45,48,51,54,57,60],
        vec![9,12,13,15,17,18],
        vec![13,12,9,6,5,4,2,1],
        vec![26,24,22,21,18],
        vec![57,59,61,64,67],
        vec![2,5,7,9,10,12],
        vec![11,10,8,7,6,3],
        vec![45,46,48,51,53,55,56,57],
        vec![27,28,30,33,35,36,37],
        vec![23,22,21,18,15,13,10],
        vec![72,74,75,78,81,84,86,89],
        vec![38,40,42,45,47],
        vec![1,4,7,8,10,11,14,16],
        vec![71,73,74,75,78,80,83,85],
        vec![16,19,20,23,26,29],
        vec![65,64,62,61,60,59,58,56],
        vec![51,49,48,45,44,42,41],
        vec![80,81,84,87,88,90,93],
        vec![37,35,34,32,31,29],
        vec![89,90,91,93,94,95,96],
        vec![62,64,66,69,70,72,75,78],
        vec![85,86,89,90,91,93],
        vec![54,52,50,47,44,42,41,39],
        vec![79,80,81,83,84,86,89],
        vec![15,18,21,22,25,27,30,31],
        vec![68,65,64,62,60,58],
        vec![80,81,83,84,85,87,90],
        vec![53,54,55,57,58],
        vec![20,21,24,25,28,30,33,35],
        vec![78,79,82,85,86],
        vec![12,15,17,19,21,24,26],
        vec![36,33,32,30,28,26,25],
        vec![68,70,73,75,78,81,82],
        vec![33,32,30,27,24,23,20],
        vec![32,31,30,29,26],
        vec![82,79,76,75,74,72,71],
        vec![37,34,31,28,25,24],
        vec![55,58,59,62,64,65,68,69],
        vec![27,28,31,33,36,38,41],
        vec![60,59,58,56,55,53,52],
        vec![81,78,76,74,72,69],
        vec![72,70,68,65,64,63,60],
        vec![43,41,40,37,34,31],
        vec![66,69,70,73,76],
        vec![24,25,28,31,32,34],
        vec![69,70,72,75,78,81],
        vec![26,27,29,31,33,34],
        vec![7,8,10,12,15,16],
        vec![61,64,65,66,67,68,71],
        vec![65,62,60,58,57,55],
        vec![15,18,21,23,24,26,28,31],
        vec![89,90,92,93,94,96,97],
        vec![17,20,22,24,27],
        vec![45,44,41,39,37],
        vec![53,55,58,60,62,64],
        vec![18,17,16,15,14,11,9],
        vec![60,61,64,66,69,72],
        vec![90,88,87,86,84,81,79,77],
        vec![49,52,53,56,58,61,63],
        vec![84,82,81,80,78,76],
        vec![15,13,12,11,9,8,5],
        vec![11,13,15,17,19,22,24,27],
        vec![85,82,80,77,75,72,70],
        vec![51,54,57,59,62,65],
        vec![86,83,82,81,79,78,77],
        vec![25,22,20,18,16],
        vec![69,67,64,61,58,55,52],
        vec![61,59,58,57,55],
        vec![97,95,94,91,88,85,84,83],
        vec![73,72,69,68,65,62],
        vec![56,58,61,62,63,64,65,67],
        vec![31,34,36,37,39,41],
        vec![95,94,91,89,88,87],
        vec![71,72,73,76,77,79,82,85],
        vec![44,47,49,52,55],
        vec![69,70,72,75,77],
        vec![65,68,69,72,75,77,79],
        vec![33,32,29,28,27,26],
        vec![78,77,75,73,71,70,69,68],
        vec![44,46,48,49,52,53,54],
        vec![56,53,51,48,47,46],
        vec![50,52,53,55,58,60,63,66],
        vec![35,36,37,40,43,46,48,51],
        vec![32,35,37,39,42],
        vec![17,14,13,10,7,6],
        vec![50,53,55,57,58,59,60,63],
        vec![26,29,32,33,35],
        vec![62,63,65,66,67,70,72,73],
        vec![65,66,69,72,73,75],
        vec![41,39,37,36,35,34,32,29],
        vec![77,74,72,70,67,66,65,62],
        vec![89,91,92,95,96,98,99],
        vec![21,19,17,15,13,10],
        vec![75,78,79,80,83,85,88,91],
        vec![12,13,14,15,17,18,19,22],
        vec![23,20,19,17,14],
        vec![55,52,49,47,45,44],
        vec![33,30,29,27,24,22,20],
        vec![92,91,90,87,84,81,78],
        vec![54,55,58,61,64,66,69,71],
        vec![66,68,70,72,75,77],
        vec![37,34,31,28,26],
        vec![74,76,79,81,82,85],
        vec![39,36,35,32,31,29,26,25],
        vec![55,54,51,48,46,45],
        vec![20,19,17,14,12,9],
        vec![71,70,69,66,64],
        vec![11,9,6,5,4,2,1],
        vec![66,63,62,59,57,55,54],
        vec![59,56,53,52,49,46,43,42],
        vec![66,67,70,71,74,76,79],
        vec![23,22,21,20,18,17],
        vec![86,83,81,78,77,75],
        vec![38,39,40,41,42],
        vec![42,41,39,37,34],
        vec![62,60,58,56,54],
        vec![92,91,89,86,84,83,81],
        vec![31,33,36,37,40,42,43,44],
        vec![35,32,31,28,26,25],
        vec![62,61,60,59,57,56,53],
        vec![65,64,62,61,60,57,55],
        vec![78,76,75,73,71],
        vec![95,94,92,89,88,86],
        vec![54,51,50,47,46,44,43],
        vec![58,61,62,65,67,70,73],
        vec![40,37,34,33,32,31],
        vec![47,50,53,54,57,58,60,61],
        vec![59,58,55,53,52],       
    ];


    let good_row_count = matrix.len();
    let mut bad_row_count = 0;

    // Iterate over each row (inner vector)
    for row in &matrix {
        if is_increasing(row) || is_decreasing(row) {
            //println!("Check 1: Semi Safe Row - {:?}", row);
            // Iterate over each element in the row
            // check if abs_value is within 3
            for col in row.windows(2) {
                //println!("{} .. {}", col[0], col[1]);
                if !is_within_distance((col[0] - col[1]).abs()) {
                    bad_row_count += 1;
                    println!("Check 2: Not Safe Row - {:?}", row);
                    break;
                }
            }
        } else {
            println!("Check 1: Not Safe Row - {:?}", row);
            bad_row_count += 1;
        }
    }
    println!("Bad reports: {}", bad_row_count);
    let total = good_row_count - bad_row_count;
    println!("Good reports: {}", total)
}
