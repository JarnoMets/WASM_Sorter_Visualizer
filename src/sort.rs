use gloo::console::log;

pub struct BubbleSort {
    i: usize,
    max: usize,
    tmp: usize,
    pub vec: Vec<usize>,
}

impl BubbleSort {
    pub fn new(data: Vec<usize>) -> Self {
        Self {
            i: 0,
            max: data.len(),
            tmp: 0,
            vec: data
        }
    }


    pub fn step(&mut self) -> bool {
        if self.max > 0 {
            if self.vec[self.i] > self.vec[self.i+1] {
                self.tmp = self.vec[self.i];
                self.vec[self.i] = self.vec[self.i+1];
                self.vec[self.i+1] = self.tmp;
            }

            if self.i+1 < self.max-1 {
                self.i += 1;
            } else {
                self.max -= 1;
                self.i = 0;
            }
            false
        } else {
            true
        }
    }
}

pub struct QuickSort {

}

impl QuickSort {

}

pub struct MergeSort {

}

impl MergeSort {

}

pub struct InsertionSort {
    i: usize,
    j: isize,
    max: usize,
    tmp: usize,
    pub vec: Vec<usize>,
}

impl InsertionSort {
    pub fn new(data: Vec<usize>) -> Self {
        Self {
            i: 1,
            j: -1,
            max: data.len(),
            tmp: 0,
            vec: data
        }
    }

    pub fn step(&mut self) -> bool {
        if self.i < self.max {
            if self.j >= 0 && self.tmp < self.vec[self.j as usize] {
                self.vec[(self.j+1) as usize] = self.vec[self.j as usize];
                self.j-=1;
                return true;
            }
            self.tmp = self.vec[self.i];
            self.j = self.i as isize -1;
            self.vec[(self.j+1) as usize] = self.tmp;
            self.i += 1;
            false
        } else {
            true
        }
    }


}